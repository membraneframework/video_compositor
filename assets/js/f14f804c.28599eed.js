"use strict";(self.webpackChunkcompositor_live=self.webpackChunkcompositor_live||[]).push([[8051],{9093:(e,o,n)=>{n.r(o),n.d(o,{assets:()=>d,contentTitle:()=>s,default:()=>c,frontMatter:()=>i,metadata:()=>a,toc:()=>l});var r=n(5893),t=n(1151);const i={},s="How to deliver input streams",a={id:"guides/deliver-input",title:"How to deliver input streams",description:"Live Compositor currently supports 2 input types:",source:"@site/pages/guides/deliver-input.md",sourceDirName:"guides",slug:"/guides/deliver-input",permalink:"/docs/guides/deliver-input",draft:!1,unlisted:!1,tags:[],version:"current",frontMatter:{},sidebar:"sidebar",previous:{title:"Simple scene",permalink:"/docs/guides/simple-scene"},next:{title:"How to receive output streams",permalink:"/docs/guides/receive-output"}},d={},l=[{value:"RTP over TCP vs UDP",id:"rtp-over-tcp-vs-udp",level:3},{value:"What to use to stream over RTP?",id:"what-to-use-to-stream-over-rtp",level:3},{value:"Membrane Framework",id:"membrane-framework",level:4},{value:"FFmpeg",id:"ffmpeg",level:4},{value:"GStreamer",id:"gstreamer",level:4}];function p(e){const o={code:"code",h1:"h1",h3:"h3",h4:"h4",li:"li",p:"p",pre:"pre",ul:"ul",...(0,t.a)(),...e.components};return(0,r.jsxs)(r.Fragment,{children:[(0,r.jsx)(o.h1,{id:"how-to-deliver-input-streams",children:"How to deliver input streams"}),"\n",(0,r.jsx)(o.p,{children:"Live Compositor currently supports 2 input types:"}),"\n",(0,r.jsxs)(o.ul,{children:["\n",(0,r.jsx)(o.li,{children:"RTP"}),"\n",(0,r.jsx)(o.li,{children:"MP4 (not supported in the Membrane Framework plugin)"}),"\n"]}),"\n",(0,r.jsx)(o.p,{children:"MP4 support is useful if you want to add some prerecorded assets, but for most streaming use cases RTP protocol will be a primary choice."}),"\n",(0,r.jsx)(o.p,{children:"Our RTP implementation supports the following codecs:"}),"\n",(0,r.jsxs)(o.ul,{children:["\n",(0,r.jsx)(o.li,{children:"H264 for video"}),"\n",(0,r.jsx)(o.li,{children:"AAC and Opus for audio (AAC is not supported via the Membrane Framework plugin)"}),"\n"]}),"\n",(0,r.jsx)(o.p,{children:"To deliver input from any other format you can use tools like FFmpeg, GStreamer or Membrane Framework to convert between RTP end the desired format."}),"\n",(0,r.jsx)(o.h3,{id:"rtp-over-tcp-vs-udp",children:"RTP over TCP vs UDP"}),"\n",(0,r.jsx)(o.p,{children:"RTP streams can be delivered over TCP or UDP. Depending on your use case, a different choice might make more sense, but in general, we recommend using TCP if possible."}),"\n",(0,r.jsx)(o.p,{children:"What to choose?"}),"\n",(0,r.jsxs)(o.ul,{children:["\n",(0,r.jsx)(o.li,{children:"If you are using the Membrane Framework plugin all communication already happens over TCP. Currently, we do not support any way to configure it."}),"\n",(0,r.jsx)(o.li,{children:"Some of the popular multimedia tools do not support RTP over TCP e.g. FFmpeg."}),"\n",(0,r.jsx)(o.li,{children:"UDP should only be used for communication on localhost. We do not support retransmission or packet reordering, so if you use it in an unreliable network it might lead to unexpected behavior."}),"\n",(0,r.jsx)(o.li,{children:"UDP does not have a congestion control, so if you are using any non-real-time sources for inputs (e.g. streaming file with FFmpeg over RTP) then if you don't throttle the input it might lead to high memory usage."}),"\n"]}),"\n",(0,r.jsx)(o.h3,{id:"what-to-use-to-stream-over-rtp",children:"What to use to stream over RTP?"}),"\n",(0,r.jsx)(o.h4,{id:"membrane-framework",children:"Membrane Framework"}),"\n",(0,r.jsxs)(o.p,{children:["If you are using the Membrane Framework plugin you do not need anything else. Just connect appropriate input pads to the ",(0,r.jsx)(o.code,{children:"LiveCompositor"})," bin."]}),"\n",(0,r.jsx)(o.h4,{id:"ffmpeg",children:"FFmpeg"}),"\n",(0,r.jsx)(o.p,{children:"FFmpeg does not support RTP over TCP, so you are limited to UDP only."}),"\n",(0,r.jsxs)(o.p,{children:["Stream an H264 video from an MP4 file (without transcoding) over RTP to ",(0,r.jsx)(o.code,{children:"127.0.0.1:9001"}),"."]}),"\n",(0,r.jsx)(o.pre,{children:(0,r.jsx)(o.code,{className:"language-bash",children:"ffmpeg -re -i path_to_file.mp4 -an -c:v copy \\\n    -f rtp -bsf:v h264_mp4toannexb rtp://127.0.0.1:9001?rtcpport=9001\n"})}),"\n",(0,r.jsxs)(o.ul,{children:["\n",(0,r.jsxs)(o.li,{children:[(0,r.jsx)(o.code,{children:"-re"})," - Limits speed of transfer to send data in real-time. Without this option entire file would be sent very quickly."]}),"\n",(0,r.jsxs)(o.li,{children:[(0,r.jsx)(o.code,{children:"-c:v copy"})," - Copy video without transcoding."]}),"\n",(0,r.jsxs)(o.li,{children:[(0,r.jsx)(o.code,{children:"-an"})," - Ignore audio stream."]}),"\n",(0,r.jsxs)(o.li,{children:[(0,r.jsx)(o.code,{children:"-bsf:v h264_mp4toannexb"})," - Convert H264 to AnnexB bitstream format (no transcoding is necessary)"]}),"\n"]}),"\n",(0,r.jsxs)(o.p,{children:["Stream a video from supported file formats (potentially with transcoding) over RTP to ",(0,r.jsx)(o.code,{children:"127.0.0.1:9001"})]}),"\n",(0,r.jsx)(o.pre,{children:(0,r.jsx)(o.code,{className:"language-bash",children:"ffmpeg -re -i path_to_file -an -c:v libx264 -f rtp rtp://127.0.0.1:9001?rtcpport=9001\n"})}),"\n",(0,r.jsxs)(o.p,{children:["Stream OPUS audio from supported file formats (potentially with transcoding) over RTP to ",(0,r.jsx)(o.code,{children:"127.0.0.1:9001"})]}),"\n",(0,r.jsx)(o.pre,{children:(0,r.jsx)(o.code,{className:"language-bash",children:"ffmpeg -re -i path_to_file -vn -c:a libopus -f rtp rtp://127.0.0.1:9001?rtcpport=9001\n"})}),"\n",(0,r.jsx)(o.h4,{id:"gstreamer",children:"GStreamer"}),"\n",(0,r.jsx)(o.p,{children:"Stream audio and video from a MP4 file over RTP TCP:"}),"\n",(0,r.jsxs)(o.ul,{children:["\n",(0,r.jsxs)(o.li,{children:["video to ",(0,r.jsx)(o.code,{children:"127.0.0.1:9001"})]}),"\n",(0,r.jsxs)(o.li,{children:["audio to ",(0,r.jsx)(o.code,{children:"127.0.0.1:9002"})]}),"\n"]}),"\n",(0,r.jsx)(o.pre,{children:(0,r.jsx)(o.code,{className:"language-bash",children:'gst-launch-1.0 filesrc location=path_to_file.mp4 ! qtdemux name=demux \\\n    ! demux.video_0 ! queue ! h264parse ! rtph264pay config-interval=1 \\\n    ! "application/x-rtp,payload=96" ! rtpstreampay ! tcpclientsink host=127.0.0.1 port=9001  \\\n    ! demux.audio_0 ! queue ! decodebin ! audioconvert ! audioresample ! opusenc ! rtpopuspay \\\n    ! "application/x-rtp,payload=97" ! rtpstreampay ! tcpclientsink host=127.0.0.1 port=9002\n'})}),"\n",(0,r.jsxs)(o.ul,{children:["\n",(0,r.jsxs)(o.li,{children:[(0,r.jsx)(o.code,{children:'"application/x-rtp,payload=97"'}),"/",(0,r.jsx)(o.code,{children:'"application/x-rtp,payload=97"'})," - Compositor detects audio stream based on payload type. It needs\nto be set to 96 for video and 97 for audio."]}),"\n",(0,r.jsxs)(o.li,{children:[(0,r.jsx)(o.code,{children:"decodebin ! audioconvert ! audioresample ! opusenc"})," - Transcode audio (most likely from AAC) from MP4 file into Opus. If you know\nwhat format is inside you can simplify this part."]}),"\n",(0,r.jsx)(o.li,{children:"Compositor supports multiplexing audio and video stream on the same port, but it is hard to create a GStreamer pipeline that demuxes\naudio and video tracks converts them to RTP and multiplex them on the same port."}),"\n"]}),"\n",(0,r.jsxs)(o.p,{children:["To stream over UDP replace ",(0,r.jsx)(o.code,{children:"rtpstreampay ! tcpclientsink host=127.0.0.1 port=9002"})," with ",(0,r.jsx)("nobr",{children:(0,r.jsx)(o.code,{children:"udpsink host=127.0.0.1 port=9002"})}),".\nAdditionally, you can use the same port for video and audio."]})]})}function c(e={}){const{wrapper:o}={...(0,t.a)(),...e.components};return o?(0,r.jsx)(o,{...e,children:(0,r.jsx)(p,{...e})}):p(e)}},1151:(e,o,n)=>{n.d(o,{Z:()=>a,a:()=>s});var r=n(7294);const t={},i=r.createContext(t);function s(e){const o=r.useContext(i);return r.useMemo((function(){return"function"==typeof e?e(o):{...o,...e}}),[o,e])}function a(e){let o;return o=e.disableParentContext?"function"==typeof e.components?e.components(t):e.components||t:s(e.components),r.createElement(i.Provider,{value:o},e.children)}}}]);