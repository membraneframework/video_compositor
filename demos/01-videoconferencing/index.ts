import { registerImage, registerInput, registerOutput, start, updateOutput, } from "../utils/api";
import { runCompositorExample } from "../utils/run";
import { gstStreamWebcam } from "../utils/gst";
import { sleepAsync } from "../utils/utils";
import { Component, Resolution } from "../types/api";
import { ffplayListenVideoAsync } from "../utils/ffmpeg";
import { randomInt } from "crypto";

const OUTPUT_RESOLUTION: Resolution = {
    width: 1920,
    height: 1080,
};

const INPUT_PORT = 8010;
const OUTPUT_PORT = randomInt(9000, 10000);
const IP = "127.0.0.1";

async function example() {
    await ffplayListenVideoAsync(IP, OUTPUT_PORT);
    await registerImage("background", {
        asset_type: "png",
        url: "https://i.ibb.co/h1wrkbg/membrane-background-fullhd.png"
    })

    await registerInput("input_1", {
        type: "rtp_stream",
        transport_protocol: "tcp_server",
        port: INPUT_PORT,
        video: {
            decoder: "ffmpeg_h264"
        }
    });

    await registerOutput("output_1", {
        type: "rtp_stream",
        ip: IP,
        port: OUTPUT_PORT,
        video: {
            resolution: OUTPUT_RESOLUTION,
            encoder: {
                type: "ffmpeg_h264",
                preset: "medium"
            },
            initial: {
                root: sceneWithInputs(0)
            }
        }
    });

    const inputs = [1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    inputs.forEach(async (input, index) =>
        await updateOutput("output_1", {
            video: {
                root: sceneWithInputs(input)
            },
            schedule_time_ms: 2000 * index
        })
    );

    await sleepAsync(2000);

    await start();
    gstStreamWebcam(IP, INPUT_PORT);
}


function sceneWithInputs(n: number): Component {
    const children: Array<Component> = Array.from({ length: n }, (_, i) => {
        const emptyView: Component = { type: "view" }
        const text: Component = {
            type: "text",
            text: `InputStream ${i} 🚀`,
            font_size: 25,
            align: "center",
            color_rgba: "#FFFFFFFF",
            background_color_rgba: "#FF0000FF",
            font_family: "Arial",
        };

        const inputStreamTile: Component = {
            type: "view",
            children: [
                {
                    type: "rescaler",
                    child: {
                        type: "input_stream",
                        input_id: "input_1"
                    }
                },
                {
                    type: "view",
                    height: 50,
                    bottom: 0,
                    left: 0,
                    children: [emptyView, text, emptyView]
                }
            ]
        };

        return inputStreamTile;
    });

    const tiles: Component = {
        type: "tiles",
        id: "tile",
        padding: 5,
        children: children,
        transition: {
            duration_ms: 700,
            easing_function: {
                function_name: "cubic_bezier",
                points: [0.35, 0.22, 0.1, 0.8]
            }
        }
    };

    const background: Component = {
        type: "image",
        image_id: "background"
    };

    return {
        type: "view",
        width: OUTPUT_RESOLUTION.width,
        height: OUTPUT_RESOLUTION.height,
        children: [
            {
                type: "view",
                width: OUTPUT_RESOLUTION.width,
                height: OUTPUT_RESOLUTION.height,
                top: 0,
                left: 0,
                children: [background]
            },
            {
                type: "view",
                width: OUTPUT_RESOLUTION.width,
                height: OUTPUT_RESOLUTION.height,
                top: 0,
                left: 0,
                children: [tiles]
            }
        ]
    }
}

runCompositorExample(example);
