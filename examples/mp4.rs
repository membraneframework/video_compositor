use anyhow::Result;
use log::{error, info};
use serde_json::json;
use std::{thread, time::Duration};
use video_compositor::{logger, server, types::Resolution};

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
    logger::init_logger();

    thread::spawn(|| {
        if let Err(err) = start_example_client_code() {
            error!("{err}")
        }
    });

    server::run();
}

fn start_example_client_code() -> Result<()> {
    info!("[example] Start listening on output port.");
    start_ffplay(IP, OUTPUT_VIDEO_PORT, Some(OUTPUT_AUDIO_PORT))?;
    start_websocket_thread();

    info!("[example] Send register input request.");
    common::post(&json!({
        "type": "register",
        "entity_type": "mp4",
        "input_id": "input_1",
        "url": BUNNY_URL
    }))?;

    let shader_source = include_str!("./silly.wgsl");
    info!("[example] Register shader transform");
    common::post(&json!({
        "type": "register",
        "entity_type": "shader",
        "shader_id": "shader_example_1",
        "source": shader_source,
    }))?;

    info!("[example] Send register output video request.");
    common::post(&json!({
        "type": "register",
        "entity_type": "output_stream",
        "output_id": "output_1",
        "port": OUTPUT_VIDEO_PORT,
        "ip": IP,
        "video": {
            "resolution": {
                "width": VIDEO_RESOLUTION.width,
                "height": VIDEO_RESOLUTION.height,
            },
            "encoder_preset": "medium",
            "initial": {
                "id": "input_1",
                "type": "input_stream",
                "input_id": "input_1",
            }
        }
    }))?;

    info!("[example] Send register output audio request.");
    common::post(&json!({
        "type": "register",
        "entity_type": "output_stream",
        "output_id": "output_2",
        "port": OUTPUT_AUDIO_PORT,
        "ip": IP,
        "audio": {
            "initial": {
                "inputs": [
                    {"input_id": "input_1"}
                ]
            },
            "channels": "stereo"
        }
    }))?;

    std::thread::sleep(Duration::from_millis(500));

    info!("[example] Start pipeline");
    common::post(&json!({
        "type": "start",
    }))?;

    Ok(())
}
