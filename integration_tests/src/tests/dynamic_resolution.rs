use std::{thread, time::Duration};

use anyhow::Result;
use serde_json::json;

use crate::{
    compare_video_dumps, input_dump_from_disk, split_rtp_packet_dump, CommunicationProtocol, CompositorInstance, OutputReceiver, PacketSender, VideoValidationConfig
};

#[test]
pub fn dynamic_resolution_input() -> Result<()> {
    const OUTPUT_DUMP_FILE: &str = "dynamic_resolution.rtp";
    let instance = CompositorInstance::start();
    let input_port = instance.get_port();
    let output_port = instance.get_port();

    instance.send_request(
        "output/output_1/register",
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
                        "type": "input_stream",
                        "input_id": "input_1",
                    }
                }
            },
        }),
    )?;

    let output_receiver = OutputReceiver::start(output_port, CommunicationProtocol::Tcp)?;

    instance.send_request(
        "output/output_1/unregister",
        json!({
            "schedule_time_ms": 69_000,
        }),
    )?;

    instance.send_request(
        "input/input_1/register",
        json!({
            "type": "rtp_stream",
            "transport_protocol": "tcp_server",
            "port": input_port,
            "video": {
                "decoder": "ffmpeg_h264"
            },
            "offset_ms": 0,
            "required": true
        }),
    )?;

    let mut input_1_sender = PacketSender::new(CommunicationProtocol::Tcp, input_port)?;
    let input_1_dump = input_dump_from_disk("dynamic_resolution.rtp")?;

    let (input_1_first_part, _input_1_second_part) =
        split_rtp_packet_dump(input_1_dump, Duration::from_secs(70))?;

    instance.send_request("start", json!({}))?;
    input_1_sender.send(&input_1_first_part)?;

    let new_output_dump = output_receiver.wait_for_output()?;

    compare_video_dumps(
        OUTPUT_DUMP_FILE,
        &new_output_dump,
        VideoValidationConfig {
            validation_intervals: vec![Duration::from_millis(62_000)..Duration::from_millis(65_000)],
            ..Default::default()
        },
    )?;

    Ok(())
}

