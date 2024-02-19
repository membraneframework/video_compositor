"use strict";(self.webpackChunkcompositor_live=self.webpackChunkcompositor_live||[]).push([[692],{4011:(e,r,n)=>{n.r(r),n.d(r,{assets:()=>c,contentTitle:()=>o,default:()=>p,frontMatter:()=>s,metadata:()=>d,toc:()=>l});var t=n(5893),i=n(1151);const s={},o=void 0,d={id:"api/generated/renderer-RtpInputStream",title:"renderer-RtpInputStream",description:"RtpInputStream",source:"@site/pages/api/generated/renderer-RtpInputStream.md",sourceDirName:"api/generated",slug:"/api/generated/renderer-RtpInputStream",permalink:"/docs/api/generated/renderer-RtpInputStream",draft:!1,unlisted:!1,tags:[],version:"current",frontMatter:{}},c={},l=[{value:"RtpInputStream",id:"rtpinputstream",level:2},{value:"Properties",id:"properties",level:4},{value:"Port",id:"port",level:2},{value:"Video",id:"video",level:2},{value:"Properties",id:"properties-1",level:4},{value:"Audio",id:"audio",level:2},{value:"Properties",id:"properties-2",level:4}];function a(e){const r={a:"a",code:"code",h2:"h2",h4:"h4",li:"li",p:"p",pre:"pre",strong:"strong",ul:"ul",...(0,i.a)(),...e.components};return(0,t.jsxs)(t.Fragment,{children:[(0,t.jsx)(r.h2,{id:"rtpinputstream",children:"RtpInputStream"}),"\n",(0,t.jsx)(r.pre,{children:(0,t.jsx)(r.code,{className:"language-typescript",children:'type RtpInputStream = {\n  input_id: string;\n  port: Port;\n  transport_protocol?: "udp" | "tcp_server";\n  video?: Video;\n  audio?: Audio;\n  required?: bool;\n  offset_ms?: f64;\n}\n'})}),"\n",(0,t.jsxs)(r.p,{children:["Parameters for an input stream from RTP source. At least one of ",(0,t.jsx)(r.code,{children:"video"})," and ",(0,t.jsx)(r.code,{children:"audio"})," has to be defined."]}),"\n",(0,t.jsx)(r.h4,{id:"properties",children:"Properties"}),"\n",(0,t.jsxs)(r.ul,{children:["\n",(0,t.jsxs)(r.li,{children:[(0,t.jsx)(r.code,{children:"input_id"})," - An identifier for the input stream."]}),"\n",(0,t.jsxs)(r.li,{children:[(0,t.jsx)(r.code,{children:"port"})," - UDP port or port range on which the compositor should listen for the stream."]}),"\n",(0,t.jsxs)(r.li,{children:[(0,t.jsx)(r.code,{children:"transport_protocol"})," - Transport protocol.","\n",(0,t.jsxs)(r.ul,{children:["\n",(0,t.jsxs)(r.li,{children:[(0,t.jsx)(r.code,{children:'"udp"'})," - UDP protocol."]}),"\n",(0,t.jsxs)(r.li,{children:[(0,t.jsx)(r.code,{children:'"tcp_server"'})," - TCP protocol where LiveCompositor is a server side of the connection."]}),"\n"]}),"\n"]}),"\n",(0,t.jsxs)(r.li,{children:[(0,t.jsx)(r.code,{children:"video"})," - Parameters of a video source included in the RTP stream."]}),"\n",(0,t.jsxs)(r.li,{children:[(0,t.jsx)(r.code,{children:"audio"})," - Parameters of an audio source included in the RTP stream."]}),"\n",(0,t.jsxs)(r.li,{children:[(0,t.jsx)(r.code,{children:"required"})," - (",(0,t.jsxs)(r.strong,{children:["default=",(0,t.jsx)(r.code,{children:"false"})]}),") If input is required and the stream is not delivered on time, then LiveCompositor will delay producing output frames."]}),"\n",(0,t.jsxs)(r.li,{children:[(0,t.jsx)(r.code,{children:"offset_ms"})," - Offset in milliseconds relative to the pipeline start (start request). If offset is not defined then stream is synchronized based on the first frames delivery time."]}),"\n"]}),"\n",(0,t.jsx)(r.h2,{id:"port",children:"Port"}),"\n",(0,t.jsx)(r.pre,{children:(0,t.jsx)(r.code,{className:"language-typescript",children:"type Port = string | u16\n"})}),"\n",(0,t.jsx)(r.h2,{id:"video",children:"Video"}),"\n",(0,t.jsx)(r.pre,{children:(0,t.jsx)(r.code,{className:"language-typescript",children:'type Video = {\n  codec?: "h264";\n  rtp_payload_type?: u8;\n}\n'})}),"\n",(0,t.jsx)(r.h4,{id:"properties-1",children:"Properties"}),"\n",(0,t.jsxs)(r.ul,{children:["\n",(0,t.jsxs)(r.li,{children:[(0,t.jsx)(r.code,{children:"codec"})," - (",(0,t.jsxs)(r.strong,{children:["default=",(0,t.jsx)(r.code,{children:'"h264"'})]}),") Video codec.","\n",(0,t.jsxs)(r.ul,{children:["\n",(0,t.jsxs)(r.li,{children:[(0,t.jsx)(r.code,{children:'"h264"'})," - H264 video."]}),"\n"]}),"\n"]}),"\n",(0,t.jsxs)(r.li,{children:[(0,t.jsx)(r.code,{children:"rtp_payload_type"})," - (",(0,t.jsxs)(r.strong,{children:["default=",(0,t.jsx)(r.code,{children:"96"})]}),") Value of payload type field in received RTP packets.\nPackets with different payload type won't be treated as video and included in composing. Values should be in [0, 64] or [96, 255]. Values in range [65, 95] can't be used. For more information, see ",(0,t.jsx)(r.a,{href:"https://datatracker.ietf.org/doc/html/rfc5761#section-4",children:"RFC"})," Packets with different payload type won't be treated as video and included in composing."]}),"\n"]}),"\n",(0,t.jsx)(r.h2,{id:"audio",children:"Audio"}),"\n",(0,t.jsx)(r.pre,{children:(0,t.jsx)(r.code,{className:"language-typescript",children:'type Audio = {\n  codec?: "opus";\n  sample_rate: u32;\n  channels: "mono" | "stereo";\n  rtp_payload_type?: u8;\n  forward_error_correction?: bool;\n}\n'})}),"\n",(0,t.jsx)(r.h4,{id:"properties-2",children:"Properties"}),"\n",(0,t.jsxs)(r.ul,{children:["\n",(0,t.jsxs)(r.li,{children:[(0,t.jsx)(r.code,{children:"codec"})," - (",(0,t.jsxs)(r.strong,{children:["default=",(0,t.jsx)(r.code,{children:'"opus"'})]}),") Audio codec.","\n",(0,t.jsxs)(r.ul,{children:["\n",(0,t.jsxs)(r.li,{children:[(0,t.jsx)(r.code,{children:'"opus"'})," - Opus audio."]}),"\n"]}),"\n"]}),"\n",(0,t.jsxs)(r.li,{children:[(0,t.jsx)(r.code,{children:"sample_rate"})," - Sample rate. If the specified sample rate doesn't match real sample rate, audio won't be mixed properly."]}),"\n",(0,t.jsxs)(r.li,{children:[(0,t.jsx)(r.code,{children:"channels"})," - Audio channels.","\n",(0,t.jsxs)(r.ul,{children:["\n",(0,t.jsxs)(r.li,{children:[(0,t.jsx)(r.code,{children:'"mono"'})," - Mono audio (single channel)."]}),"\n",(0,t.jsxs)(r.li,{children:[(0,t.jsx)(r.code,{children:'"stereo"'})," - Stereo audio (two channels)."]}),"\n"]}),"\n"]}),"\n",(0,t.jsxs)(r.li,{children:[(0,t.jsx)(r.code,{children:"rtp_payload_type"})," - (",(0,t.jsxs)(r.strong,{children:["default=",(0,t.jsx)(r.code,{children:"97"})]}),") Value of payload type field in received RTP packets.\nPackets with different payload type won't be treated as audio and included in mixing. Values should be in range [0, 64] or [96, 255]. Values in range [65, 95] can't be used. For more information, check out ",(0,t.jsx)(r.a,{href:"https://datatracker.ietf.org/doc/html/rfc5761#section-4",children:"RFC"}),"."]}),"\n",(0,t.jsxs)(r.li,{children:[(0,t.jsx)(r.code,{children:"forward_error_correction"})," - (",(0,t.jsxs)(r.strong,{children:["default=",(0,t.jsx)(r.code,{children:"false"})]}),") Specifies whether the stream uses forward error correction. It's specific for Opus codec. For more information, check out ",(0,t.jsx)(r.a,{href:"https://datatracker.ietf.org/doc/html/rfc6716#section-2.1.7",children:"RFC"}),"."]}),"\n"]})]})}function p(e={}){const{wrapper:r}={...(0,i.a)(),...e.components};return r?(0,t.jsx)(r,{...e,children:(0,t.jsx)(a,{...e})}):a(e)}},1151:(e,r,n)=>{n.d(r,{Z:()=>d,a:()=>o});var t=n(7294);const i={},s=t.createContext(i);function o(e){const r=t.useContext(s);return t.useMemo((function(){return"function"==typeof e?e(r):{...r,...e}}),[r,e])}function d(e){let r;return r=e.disableParentContext?"function"==typeof e.components?e.components(i):e.components||i:o(e.components),t.createElement(s.Provider,{value:r},e.children)}}}]);