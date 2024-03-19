"use strict";(self.webpackChunkcompositor_live=self.webpackChunkcompositor_live||[]).push([[6010],{7181:(e,n,t)=>{t.r(n),t.d(n,{assets:()=>l,contentTitle:()=>o,default:()=>u,frontMatter:()=>r,metadata:()=>d,toc:()=>c});var s=t(5893),i=t(1151);const r={},o="RTP",d={id:"api/outputs/rtp",title:"RTP",description:"An output type that allows streaming video and audio from the compositor over RTP.",source:"@site/pages/api/outputs/rtp.md",sourceDirName:"api/outputs",slug:"/api/outputs/rtp",permalink:"/docs/api/outputs/rtp",draft:!1,unlisted:!1,tags:[],version:"current",frontMatter:{},sidebar:"sidebar",previous:{title:"Web Renderer",permalink:"/docs/api/renderers/web"},next:{title:"RTP",permalink:"/docs/api/inputs/rtp"}},l={},c=[];function a(e){const n={a:"a",code:"code",h1:"h1",li:"li",p:"p",pre:"pre",strong:"strong",ul:"ul",...(0,i.a)(),...e.components};return(0,s.jsxs)(s.Fragment,{children:[(0,s.jsx)(n.h1,{id:"rtp",children:"RTP"}),"\n",(0,s.jsx)(n.p,{children:"An output type that allows streaming video and audio from the compositor over RTP."}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-typescript",children:'type RegisterOutputStream = {\n  output_id: string;\n  transport_protocol?: "udp" | "tcp_server";\n  port: u16;\n  ip?: string;\n  video?: Video;\n  audio?: Audio;\n}\n'})}),"\n",(0,s.jsx)(n.p,{children:"Register a new RTP output stream."}),"\n",(0,s.jsxs)(n.ul,{children:["\n",(0,s.jsxs)(n.li,{children:[(0,s.jsx)(n.code,{children:"output_id"})," - An identifier for the output stream. It can be used in the ",(0,s.jsx)(n.code,{children:"UpdateOutput"})," request to define what to render for the output stream."]}),"\n",(0,s.jsxs)(n.li,{children:[(0,s.jsx)(n.code,{children:"transport_protocol"})," - (",(0,s.jsxs)(n.strong,{children:["default=",(0,s.jsx)(n.code,{children:'"udp"'})]}),") Transport layer protocol that will be used to send RTP packets.","\n",(0,s.jsxs)(n.ul,{children:["\n",(0,s.jsxs)(n.li,{children:[(0,s.jsx)(n.code,{children:"udp"})," - UDP protocol."]}),"\n",(0,s.jsxs)(n.li,{children:[(0,s.jsx)(n.code,{children:"tcp_server"})," - TCP protocol where LiveCompositor is the server side of the connection."]}),"\n"]}),"\n"]}),"\n",(0,s.jsxs)(n.li,{children:[(0,s.jsx)(n.code,{children:"port"})," - Depends on the value of the ",(0,s.jsx)(n.code,{children:"transport_protocol"})," field:","\n",(0,s.jsxs)(n.ul,{children:["\n",(0,s.jsxs)(n.li,{children:[(0,s.jsx)(n.code,{children:"udp"})," - An UDP port number that RTP packets will be sent to."]}),"\n",(0,s.jsxs)(n.li,{children:[(0,s.jsx)(n.code,{children:"tcp_server"})," - A local TCP port number or a port range that LiveCompositor will listen for incoming connections."]}),"\n"]}),"\n"]}),"\n",(0,s.jsxs)(n.li,{children:[(0,s.jsx)(n.code,{children:"ip"})," - Only valid if ",(0,s.jsx)(n.code,{children:'transport_protocol="udp"'}),". IP address where RTP packets should be sent to."]}),"\n"]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-typescript",children:'type Video = {\n  resolution: { width: number; height: number };\n  encoder_preset?: VideoEncoderPreset;\n  send_eos_when?: EosCondition;\n  initial: Component;\n}\n\ntype VideoEncoderPreset =\n  | "ultrafast"\n  | "superfast"\n  | "veryfast"\n  | "faster"\n  | "fast"\n  | "medium"\n  | "slow"\n  | "slower"\n  | "veryslow"\n  | "placebo"\n\n'})}),"\n",(0,s.jsxs)(n.ul,{children:["\n",(0,s.jsxs)(n.li,{children:[(0,s.jsx)(n.code,{children:"resolution"})," - Output resolution in pixels."]}),"\n",(0,s.jsxs)(n.li,{children:[(0,s.jsx)(n.code,{children:"encoder_preset"})," - (",(0,s.jsxs)(n.strong,{children:["default=",(0,s.jsx)(n.code,{children:'"fast"'})]}),") Preset for an encoder. See ",(0,s.jsx)(n.code,{children:"FFmpeg"})," ",(0,s.jsx)(n.a,{href:"https://trac.ffmpeg.org/wiki/Encode/H.264#Preset",children:"docs"})," to learn more."]}),"\n",(0,s.jsxs)(n.li,{children:[(0,s.jsx)(n.code,{children:"send_eos_when"})," - Defines when output stream should end if some of the input streams are finished. If output includes both audio and video streams, then EOS needs to be sent on both."]}),"\n",(0,s.jsxs)(n.li,{children:[(0,s.jsx)(n.code,{children:"initial"})," - Root of a component tree/scene that should be rendered for the output. Use ",(0,s.jsxs)(n.a,{href:"/docs/api/routes#update-output",children:[(0,s.jsx)(n.code,{children:"update_output"})," request"]})," to update this value after registration. ",(0,s.jsx)(n.a,{href:"/docs/concept/component",children:"Learn more"}),"."]}),"\n"]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-typescript",children:'type Audio = {\n  channels: "stereo" | "mono";\n  forward_error_correction?: boolean;\n  encoder_preset?: AudioEncoderPreset;\n  send_eos_when?: EosCondition;\n  initial: {\n    inputs: AudioInput[];\n  };\n  mixing_strategy?: "sum_clip" | "sum_scale" \n}\n\ntype AudioInput = {\n  input_id: string;\n  volume?: number;\n}\n\ntype AudioEncoderPreset =\n  | "quality"\n  | "voip"\n  | "lowest_latency"\n\n'})}),"\n",(0,s.jsxs)(n.ul,{children:["\n",(0,s.jsxs)(n.li,{children:[(0,s.jsx)(n.code,{children:"channels"})," - Channel configuration for output audio."]}),"\n",(0,s.jsxs)(n.li,{children:[(0,s.jsx)(n.code,{children:"forward_error_correction"})," - (",(0,s.jsxs)(n.strong,{children:["default=",(0,s.jsx)(n.code,{children:"false"})]}),") Specifies whether the stream use forward error correction. It's specific for Opus codec. For more information, check out ",(0,s.jsx)(n.a,{href:"https://datatracker.ietf.org/doc/html/rfc6716#section-2.1.7",children:"RFC"}),"."]}),"\n",(0,s.jsxs)(n.li,{children:[(0,s.jsx)(n.code,{children:"encoder_preset"})," - (",(0,s.jsxs)(n.strong,{children:["default=",(0,s.jsx)(n.code,{children:'"voip"'})]}),") Preset for an encoder.","\n",(0,s.jsxs)(n.ul,{children:["\n",(0,s.jsxs)(n.li,{children:[(0,s.jsx)(n.code,{children:"quality"})," - Best for broadcast/high-fidelity application where the decoded audio should be as close as possible to the input."]}),"\n",(0,s.jsxs)(n.li,{children:[(0,s.jsx)(n.code,{children:"voip"})," - Best for most VoIP/videoconference applications where listening quality and intelligibility matter most."]}),"\n",(0,s.jsxs)(n.li,{children:[(0,s.jsx)(n.code,{children:"lowest_latency"})," - Only use when lowest-achievable latency is what matters most."]}),"\n"]}),"\n"]}),"\n",(0,s.jsxs)(n.li,{children:[(0,s.jsx)(n.code,{children:"send_eos_when"})," - Defines when output stream should end if some of the input streams are finished. If output includes both audio and video streams, then EOS needs to be sent on both."]}),"\n",(0,s.jsxs)(n.li,{children:[(0,s.jsx)(n.code,{children:"initial"})," - Initial configuration for audio mixer for this output. Use ",(0,s.jsxs)(n.a,{href:"/docs/api/routes#update-output",children:[(0,s.jsx)(n.code,{children:"update_output"})," request"]})," to update this value after registration."]}),"\n",(0,s.jsxs)(n.li,{children:[(0,s.jsx)(n.code,{children:"initial.inputs[].input_id"})," - Input ID."]}),"\n",(0,s.jsxs)(n.li,{children:[(0,s.jsx)(n.code,{children:"initial.inputs[].volume"})," - (",(0,s.jsxs)(n.strong,{children:["default=",(0,s.jsx)(n.code,{children:"1.0"})]}),") Float in ",(0,s.jsx)(n.code,{children:"[0, 1]"})," range representing volume."]}),"\n",(0,s.jsxs)(n.li,{children:[(0,s.jsx)(n.code,{children:"mixing_strategy"})," - (",(0,s.jsxs)(n.strong,{children:["default=",(0,s.jsx)(n.code,{children:"sum_clip"})]}),") Specifies how input samples should be mixed:","\n",(0,s.jsxs)(n.ul,{children:["\n",(0,s.jsxs)(n.li,{children:[(0,s.jsx)(n.code,{children:"sum_clip"})," - Firstly, input samples are summed. If the result sample is outside the i16 PCM range, it gets clipped."]}),"\n",(0,s.jsxs)(n.li,{children:[(0,s.jsx)(n.code,{children:"sum_scale"})," - Firstly, input samples are summed. If the result wave is outside the i16 PCM range, nearby samples are scaled down by factor, such that the summed wave is in the i16 PCM range."]}),"\n"]}),"\n"]}),"\n"]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-typescript",children:"type EosCondition = {\n  any_input?: bool;\n  all_inputs?: bool;\n  any_of?: InputId[];\n  all_of?: InputId[];\n}\n"})}),"\n",(0,s.jsx)(n.p,{children:"This type defines when end of an input stream should trigger end of the output stream. Only one of those fields can be set at the time."}),"\n",(0,s.jsx)(n.p,{children:"Unless specified otherwise the input stream is considered finished/ended when:"}),"\n",(0,s.jsxs)(n.ul,{children:["\n",(0,s.jsx)(n.li,{children:"TCP connection was dropped/closed."}),"\n",(0,s.jsxs)(n.li,{children:["RTCP Goodbye packet (",(0,s.jsx)(n.code,{children:"BYE"}),") was received."]}),"\n",(0,s.jsx)(n.li,{children:"Mp4 track has ended."}),"\n",(0,s.jsx)(n.li,{children:"Input was unregistered already (or never registered)."}),"\n"]}),"\n",(0,s.jsx)(n.p,{children:"Options:"}),"\n",(0,s.jsxs)(n.ul,{children:["\n",(0,s.jsxs)(n.li,{children:[(0,s.jsx)(n.code,{children:"any_of"})," - Terminate output stream if any of the input streams from the list are finished."]}),"\n",(0,s.jsxs)(n.li,{children:[(0,s.jsx)(n.code,{children:"all_of"})," - Terminate output stream if all the input streams from the list are finished."]}),"\n",(0,s.jsxs)(n.li,{children:[(0,s.jsx)(n.code,{children:"any_input"})," - Terminate output stream if any of the input streams ends. This includes streams added after the output was registered. In particular, output stream will ",(0,s.jsx)(n.strong,{children:"not be"})," terminated if no inputs were ever connected."]}),"\n",(0,s.jsxs)(n.li,{children:[(0,s.jsx)(n.code,{children:"all_inputs"})," - Terminate output stream if all the input streams finish. In particular, output stream will ",(0,s.jsx)(n.strong,{children:"be"})," terminated if no inputs were ever connected."]}),"\n"]})]})}function u(e={}){const{wrapper:n}={...(0,i.a)(),...e.components};return n?(0,s.jsx)(n,{...e,children:(0,s.jsx)(a,{...e})}):a(e)}},1151:(e,n,t)=>{t.d(n,{Z:()=>d,a:()=>o});var s=t(7294);const i={},r=s.createContext(i);function o(e){const n=s.useContext(r);return s.useMemo((function(){return"function"==typeof e?e(n):{...n,...e}}),[n,e])}function d(e){let n;return n=e.disableParentContext?"function"==typeof e.components?e.components(i):e.components||i:o(e.components),s.createElement(r.Provider,{value:n},e.children)}}}]);