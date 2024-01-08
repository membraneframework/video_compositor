use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use anyhow::{anyhow, Result};
use log::info;

use crate::utils;

const ARM_MAC_TARGET: &str = "aarch64-apple-darwin";
const ARM_OUTPUT_FILE: &str = "video_compositor_darwin_aarch64.tar.gz";
const ARM_WITH_WEB_RENDERER_OUTPUT_FILE: &str =
    "video_compositor_with_web_renderer_darwin_aarch64.tar.gz";

const INTEL_MAC_TARGET: &str = "x86_64-apple-darwin";
const INTEL_OUTPUT_FILE: &str = "video_compositor_darwin_x86_64.tar.gz";
const INTEL_WITH_WEB_RENDERER_OUTPUT_FILE: &str =
    "video_compositor_with_web_renderer_darwin_x86_64.tar.gz";

pub fn bundle_macos_app() -> Result<()> {
    tracing_subscriber::fmt().init();

    if cfg!(target_arch = "x86_64") {
        bundle_app(INTEL_MAC_TARGET, INTEL_OUTPUT_FILE, false)?;
        bundle_app(INTEL_MAC_TARGET, INTEL_WITH_WEB_RENDERER_OUTPUT_FILE, true)?;
    } else if cfg!(target_arch = "aarch64") {
        bundle_app(ARM_MAC_TARGET, ARM_OUTPUT_FILE, false)?;
        bundle_app(ARM_MAC_TARGET, ARM_WITH_WEB_RENDERER_OUTPUT_FILE, true)?;
    } else {
        panic!("Unknown architecture")
    }
    Ok(())
}

fn bundle_app(target: &'static str, output_name: &str, enable_web_rendering: bool) -> Result<()> {
    if enable_web_rendering {
        info!("Bundling compositor with web rendering");
    } else {
        info!("Bundling compositor without web rendering");
    }

    let root_dir_str = env!("CARGO_MANIFEST_DIR");
    let root_dir: PathBuf = root_dir_str.into();
    let build_dir = root_dir.join(format!("target/{target}/release"));
    let tmp_dir = root_dir.join("video_compositor");
    utils::setup_bundle_dir(&tmp_dir)?;

    info!("Build main_process binary.");
    utils::cargo_build("main_process", target, !enable_web_rendering)?;

    info!("Create macOS bundle.");
    if enable_web_rendering {
        use compositor_chromium::cef;

        info!("Build process_helper binary.");
        utils::cargo_build("process_helper", target, false)?;
        cef::bundle_app(&build_dir, &tmp_dir.join("video_compositor.app"))?;
    }

    fs::copy(
        build_dir.join("main_process"),
        tmp_dir.join("video_compositor"),
    )?;

    if enable_web_rendering {
        info!("Removing extended attributes");
        let exit_code = Command::new("sudo")
            .args(["xattr", "-cr", "video_compositor"])
            .spawn()?
            .wait()?
            .code();

        if exit_code != Some(0) {
            return Err(anyhow!(
                "Command xattr failed with exit code {:?}",
                exit_code
            ));
        }
    }

    info!("Create tar.gz archive.");
    let exit_code = Command::new("tar")
        .args(["-C", root_dir_str, "-czvf", output_name, "video_compositor"])
        .spawn()?
        .wait()?
        .code();
    if exit_code != Some(0) {
        return Err(anyhow!("Command tar failed with exit code {:?}", exit_code));
    }

    Ok(())
}
