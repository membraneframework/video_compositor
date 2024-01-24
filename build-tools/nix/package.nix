{ rustPlatform
, openssl
, pkg-config
, ffmpeg_6-headless
, llvmPackages_16
, libGL
, cmake
, libopus
, lib
, vulkan-loader
, mesa
, darwin
, stdenv
, makeWrapper
, x264
}:
let
  ffmpeg =
    (if stdenv.isDarwin then
      (ffmpeg_6-headless.override {
        x264 = x264.overrideAttrs (old: {
          postPatch = old.postPatch + ''
            substituteInPlace Makefile --replace '$(if $(STRIP), $(STRIP) -x $@)' '$(if $(STRIP), $(STRIP) -S $@)'
          '';
        });
      })
    else
      ffmpeg_6-headless
    );
  buildInputs = [
    ffmpeg
    openssl
    libopus
    libGL
    mesa.drivers
    vulkan-loader
  ] ++ lib.optionals stdenv.isDarwin [
    darwin.apple_sdk.frameworks.Metal
    darwin.apple_sdk.frameworks.Foundation
    darwin.apple_sdk.frameworks.QuartzCore
    darwin.libobjc
  ];
  rpath = lib.makeLibraryPath buildInputs;
in
rustPlatform.buildRustPackage {
  pname = "video_compositor";
  version = "0.2.0-rc.1";
  src = ../..;
  cargoSha256 = "sha256-KwwfdE6o3drmhzClwoBjwjXLVKYuDgq/SDG5ZGsqzCU=";

  buildNoDefaultFeatures = true;
  doCheck = false;

  inherit buildInputs;
  nativeBuildInputs = [ pkg-config llvmPackages_16.clang cmake makeWrapper ];

  env.LIBCLANG_PATH = "${llvmPackages_16.libclang.lib}/lib";

  postFixup = ''
    patchelf --set-rpath ${rpath} $out/bin/video_compositor
    wrapProgram $out/bin/video_compositor \
      --prefix XDG_DATA_DIRS : "${mesa.drivers}/share" \
      --prefix LD_LIBRARY_PATH : "${mesa.drivers}/lib"
  '';
}
        
