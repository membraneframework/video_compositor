use std::time::Duration;

use crate::{
    compare_video_dumps, input_dump_from_disk, CommunicationProtocol, CompositorInstance,
    OutputReceiver, PacketSender, VideoValidationConfig,
};
use anyhow::Result;
use serde_json::json;

/// Schedules an output update.
///
/// Show `input_1` for 2 seconds.
/// Show `input_1` and `input_2` side by side (transition with animation) for 18 seconds.
#[test]
pub fn schedule_update() -> Result<()> {
    const OUTPUT_DUMP_FILE: &str = "schedule_update_output.rtp";
    let instance = CompositorInstance::start();
    let input_1_port = instance.get_port();
    let input_2_port = instance.get_port();
    let output_port = instance.get_port();

    instance.send_request(
        "output/output_with_animation/register",
        json!({
            "type": "rtp_stream",
            "transport_protocol": "tcp_server",
            "port": output_port,
            "video": {
                "resolution": {
                    "width": 640,
                    "height": 360,
                },
                "encoder": {
                    "type": "ffmpeg_h264",
                    "preset": "ultrafast"
                },
                "initial": {
                    "root": {
                        "type": "tiles",
                        "id": "tiles_1",
                        "padding": 3,
                        "background_color_rgba": "#DDDDDDFF",
                        "transition": {
                            "duration_ms": 500,
                            "easing_function": {
                                "function_name": "bounce"
                            }
                        },
                        "children": [
                            {
                                "type": "input_stream",
                                "input_id": "input_1_anim",
                            },
                        ],
                    }
                }
            },
        }),
    )?;

    instance.send_request(
        "output/output_with_animation/update",
        json!({
            "video": {
                "root": {
                    "type": "tiles",
                    "id": "tiles_1",
                    "padding": 3,
                    "background_color_rgba": "#DDDDDDFF",
                    "transition": {
                        "duration_ms": 500,
                        "easing_function": {
                            "function_name": "bounce"
                        }
                    },
                    "children": [
                        {
                            "type": "input_stream",
                            "input_id": "input_1_anim",
                        },
                        {
                            "type": "input_stream",
                            "input_id": "input_2_anim",
                        },
                    ],
                }
            },
            "schedule_time_ms": 2000
        }),
    )?;

    instance.send_request(
        "output/output_with_animation/unregister",
        json!({
            "schedule_time_ms": 20000,
        }),
    )?;

    let output_receiver = OutputReceiver::start(output_port, CommunicationProtocol::Tcp)?;

    instance.send_request(
        "input/input_1_anim/register",
        json!({
            "type": "rtp_stream",
            "transport_protocol": "udp",
            "port": input_1_port,
            "video": {
                "decoder": "ffmpeg_h264"
            },
        }),
    )?;

    instance.send_request(
        "input/input_2_anim/register",
        json!({
            "type": "rtp_stream",
            "transport_protocol": "tcp_server",
            "port": input_2_port,
            "video": {
                "decoder": "ffmpeg_h264"
            },
        }),
    )?;

    instance.send_request("start", json!({}))?;

    let mut input_1_sender = PacketSender::new(CommunicationProtocol::Udp, input_1_port)?;
    let mut input_2_sender = PacketSender::new(CommunicationProtocol::Tcp, input_2_port)?;
    let input_1_dump = input_dump_from_disk("8_colors_input_video.rtp")?;
    let input_2_dump = input_dump_from_disk("8_colors_input_reversed_video.rtp")?;

    input_1_sender.send(&input_1_dump)?;
    input_2_sender.send(&input_2_dump)?;

    let new_output_dump = output_receiver.wait_for_output()?;

    compare_video_dumps(
        OUTPUT_DUMP_FILE,
        &new_output_dump,
        VideoValidationConfig {
            validation_intervals: vec![Duration::from_millis(500)..Duration::from_millis(3500)],
            ..Default::default()
        },
    )?;

    Ok(())
}
