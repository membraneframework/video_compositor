use anyhow::Result;
use compositor_common::Framerate;
use compositor_pipeline::Pipeline;
use log::{error, info};
use serde_json::json;
use signal_hook::{consts, iterator::Signals};
use std::{process::Command, sync::Arc, thread, time::Duration};
use video_compositor::{http, state::State};

use crate::common::write_example_sdp_file;

#[path = "./common/common.rs"]
mod common;

fn main() {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );
    ffmpeg_next::format::network::init();
    let pipeline = Arc::new(Pipeline::new(Framerate(30)));
    pipeline.start();
    let state = Arc::new(State::new(pipeline));

    thread::spawn(|| {
        if let Err(err) = start_example_client_code() {
            error!("{err}")
        }
    });

    http::Server::new(8001, state).start();

    let mut signals = Signals::new([consts::SIGINT]).unwrap();
    signals.forever().next();
}

fn start_example_client_code() -> Result<()> {
    thread::sleep(Duration::from_secs(2));

    info!("[example] Sending init request.");
    common::post(&json!({
        "type": "init",
    }))?;

    info!("[example] Start listening on output port.");
    let output_sdp = write_example_sdp_file(8002)?;
    Command::new("ffplay")
        .args(["-protocol_whitelist", "file,rtp,udp", &output_sdp])
        .spawn()?;

    info!("[example] Send register output request.");
    common::post(&json!({
        "type": "register_output",
        "id": "output 1",
        "port": 8002,
        "resolution": {
            "width": 3840,
            "height": 2160,
        },
        "encoder_settings": {
            "preset": "ultrafast"
        }
    }))?;

    info!("[example] Send register input request.");
    common::post(&json!({
        "type": "register_input",
        "id": "input 1",
        "port": 8004
    }))?;

    info!("[example] Register shader transform");
    common::post(&json!({
        "type": "register_transformation",
        "key": "example shader",
        "transform": {
            "type": "shader",
            "source": "some source or other data"
        }
    }))?;

    info!("[example] Register web renderer transform");
    common::post(&json!({
        "type": "register_transformation",
        "key": "example website",
        "transform": {
            "type": "web_renderer",
            "url": "http://some-website", // or other way of providing source
        }
    }))?;

    info!("[example] Update scene");
    common::post(&json!({
        "type": "update_scene",
        "inputs": [
            {
                "input_id": "input 1",
                "resolution": { "width": 3840, "height": 2160 },
            }
        ],
        "transforms": [
           {
               "node_id": "side-by-side",
               "type": "shader",
               "shader_id": "example shader",
               "shader_params": {
                   "example": {"type": "string", "value": "param"},
               },
               "input_pads": [
                   "input 1",
               ],
               "resolution": { "width": 3840, "height": 2160 },
           },
           {
               "node_id": "add-overlay",
               "type": "web_renderer",
               "renderer_id": "example website",
               "input_pads": [
                   "side-by-side",
               ],
               "resolution": { "width": 3840, "height": 2160 },
           }
        ],
        "outputs": [
            {
                "output_id": "output 1",
                "input_pad": "add-overlay"
            }
        ]
    }))?;

    Command::new("ffmpeg")
        .args([
            "-re",
            "-f",
            "lavfi",
            "-i",
            "testsrc=s=3840x2160:r=30,format=yuv420p",
            "-c:v",
            "libx264",
            "-f",
            "rtp",
            "rtp://127.0.0.1:8004?rtcpport=8004",
        ])
        .spawn()?;
    Ok(())
}