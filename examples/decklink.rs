use anyhow::Result;
use live_compositor::{server, types::Resolution};
use log::{error, info};
use serde_json::json;
use std::{process::{Command, Stdio}, thread, time::Duration};

use crate::common::{start_ffplay, start_websocket_thread};

#[path = "./common/common.rs"]
mod common;

const BUNNY_URL: &str =
    "https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4";

const VIDEO_RESOLUTION: Resolution = Resolution {
    width: 1280,
    height: 720,
};

const IP: &str = "127.0.0.1";
const OUTPUT_VIDEO_PORT: u16 = 8002;
const OUTPUT_AUDIO_PORT: u16 = 8004;

fn main() {
    ffmpeg_next::format::network::init();

    thread::spawn(|| {
        if let Err(err) = start_example_client_code() {
            error!("{err}")
        }
    });

    server::run();
}

fn start_example_client_code() -> Result<()> {
    info!("[example] Start listening on output port.");
    thread::sleep(Duration::from_secs(2));
    start_websocket_thread();

    info!("[example] Send register input request.");
    common::post(
        "input/input_1/register",
        &json!({
            "type": "decklink",
            "subdevice": 0,
        }),
    )?;

    std::thread::sleep(Duration::from_millis(2000));

    info!("[example] Send register output video request.");
    common::post(
        "output/output_1/register",
        &json!({
            "type": "rtp_stream",
            "transport_protocol": "tcp_server",
            "port": OUTPUT_VIDEO_PORT,
            "video": {
                "resolution": {
                    "width": VIDEO_RESOLUTION.width,
                    "height": VIDEO_RESOLUTION.height,
                },
                "encoder": {
                    "type": "ffmpeg_h264",
                    "preset": "ultrafast"
                },
                "initial": {
                    "root": {
                        "type": "view",
                        "background_color_rgba": "#FF0000FF",
                        "children": [
                            {
                                "type": "rescaler",
                                "top": 10,
                                "left": 10,
                                "child": {
                                    "id": "input_1",
                                    "type": "input_stream",
                                    "input_id": "input_1",
                                }
                            }
                        ]
                    }
                }
            }
        }),
    )?;

   // info!("[example] Send register output audio request.");
   // common::post(
   //     "output/output_2/register",
   //     &json!({
   //         "type": "rtp_stream",
   //         "port": OUTPUT_AUDIO_PORT,
   //         "ip": IP,
   //         "audio": {
   //             "initial": {
   //                 "inputs": [
   //                     {"input_id": "input_1"}
   //                 ]
   //             },
   //             "encoder": {
   //                 "type": "opus",
   //                 "channels": "stereo"
   //             }
   //         }
   //     }),
   // )?;

    let gst_output_command =  [
        "gst-launch-1.0 -v ",
        "rtpptdemux name=demux ",
        &format!("tcpclientsrc host={IP} port={OUTPUT_VIDEO_PORT} ! \"application/x-rtp-stream\" ! rtpstreamdepay ! queue ! demux. "),
        "demux.src_96 ! \"application/x-rtp,media=video,clock-rate=90000,encoding-name=H264\" ! queue ! rtph264depay ! decodebin ! videoconvert ! autovideosink ",
  //      "demux.src_97 ! \"application/x-rtp,media=audio,clock-rate=48000,encoding-name=OPUS\" ! queue ! rtpopusdepay ! decodebin ! audioconvert ! autoaudiosink ",
    ].concat();
    Command::new("bash")
        .arg("-c")
        .arg(gst_output_command)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    std::thread::sleep(Duration::from_millis(500));

    info!("[example] Start pipeline");
    common::post("start", &json!({}))?;

    Ok(())
}
