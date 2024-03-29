"use strict";(self.webpackChunkcompositor_live=self.webpackChunkcompositor_live||[]).push([[7508],{5237:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>c,contentTitle:()=>d,default:()=>p,frontMatter:()=>i,metadata:()=>o,toc:()=>l});var r=n(5893),s=n(1151);const i={description:"API routes to configure the compositor."},d="Routes",o={id:"api/routes",title:"Routes",description:"API routes to configure the compositor.",source:"@site/pages/api/routes.md",sourceDirName:"api",slug:"/api/routes",permalink:"/docs/api/routes",draft:!1,unlisted:!1,tags:[],version:"current",frontMatter:{description:"API routes to configure the compositor."},sidebar:"sidebar",previous:{title:"API Reference",permalink:"/docs/category/api-reference"},next:{title:"Events",permalink:"/docs/api/events"}},c={},l=[{value:"Endpoint <code>POST /--/api</code>",id:"endpoint-post---api",level:2},{value:"Start",id:"start",level:3},{value:"Update output",id:"update-output",level:3},{value:"Register input stream",id:"register-input-stream",level:3},{value:"Register output stream",id:"register-output-stream",level:3},{value:"Register renderer",id:"register-renderer",level:3},{value:"Unregister request",id:"unregister-request",level:3},{value:"Endpoint <code>GET /status</code>",id:"endpoint-get-status",level:2},{value:"WebSocket endpoint <code>/--/ws</code>",id:"websocket-endpoint---ws",level:2}];function a(e){const t={a:"a",code:"code",h1:"h1",h2:"h2",h3:"h3",hr:"hr",li:"li",p:"p",pre:"pre",strong:"strong",ul:"ul",...(0,s.a)(),...e.components};return(0,r.jsxs)(r.Fragment,{children:[(0,r.jsx)(t.h1,{id:"routes",children:"Routes"}),"\n",(0,r.jsxs)(t.p,{children:["API is served by default on the port 8081. Different port can be configured using ",(0,r.jsx)(t.a,{href:"../deployment/configuration#live_compositor_api_port",children:(0,r.jsx)(t.code,{children:"LIVE_COMPOSITOR_API_PORT"})})," environment variable."]}),"\n",(0,r.jsxs)(t.h2,{id:"endpoint-post---api",children:["Endpoint ",(0,r.jsx)(t.code,{children:"POST /--/api"})]}),"\n",(0,r.jsx)(t.p,{children:"Main endpoint for configuring the compositor server."}),"\n",(0,r.jsx)(t.h3,{id:"start",children:"Start"}),"\n",(0,r.jsx)(t.pre,{children:(0,r.jsx)(t.code,{className:"language-typescript",children:'type Start = {\n  type: "start";\n}\n'})}),"\n",(0,r.jsx)(t.p,{children:"Starts the processing pipeline. If outputs are registered and defined in the scene then the compositor will start to send the RTP stream."}),"\n",(0,r.jsx)(t.hr,{}),"\n",(0,r.jsx)(t.h3,{id:"update-output",children:"Update output"}),"\n",(0,r.jsx)(t.pre,{children:(0,r.jsx)(t.code,{className:"language-typescript",children:'type UpdateOutput = {\n  type: "update_output";\n  output_id: string;\n  video?: Component;\n  audio?: {\n    inputs: AudioInput[];\n  };\n  schedule_time_ms?: number;\n}\n\ntype AudioInput = {\n  input_id: InputId;\n  volume?: number;\n}\n'})}),"\n",(0,r.jsxs)(t.ul,{children:["\n",(0,r.jsxs)(t.li,{children:[(0,r.jsx)(t.code,{children:"output_id"})," - Id of an already registered output stream. See ",(0,r.jsx)(t.a,{href:"./routes#register-output-stream",children:(0,r.jsx)(t.code,{children:"RegisterOutputStream"})}),"."]}),"\n",(0,r.jsxs)(t.li,{children:[(0,r.jsx)(t.code,{children:"video"})," - Root of a component tree/scene that should be rendered for the output. ",(0,r.jsx)(t.a,{href:"../concept/component",children:"Learn more"})]}),"\n",(0,r.jsxs)(t.li,{children:[(0,r.jsx)(t.code,{children:"audio"})," - Parameters for mixing input audio streams."]}),"\n",(0,r.jsxs)(t.li,{children:[(0,r.jsx)(t.code,{children:"audio.inputs[].input_id"})," - Input id."]}),"\n",(0,r.jsxs)(t.li,{children:[(0,r.jsx)(t.code,{children:"audio.inputs[].volume"})," - (",(0,r.jsxs)(t.strong,{children:["default=",(0,r.jsx)(t.code,{children:"1.0"})]}),") Float in ",(0,r.jsx)(t.code,{children:"[0, 1]"})," range representing volume."]}),"\n",(0,r.jsxs)(t.li,{children:[(0,r.jsx)(t.code,{children:"schedule_time_ms"})," - Time in milliseconds when this request should be applied. Value ",(0,r.jsx)(t.code,{children:"0"})," represents time of ",(0,r.jsx)(t.a,{href:"#start",children:"the start request"}),"."]}),"\n"]}),"\n",(0,r.jsx)(t.hr,{}),"\n",(0,r.jsx)(t.h3,{id:"register-input-stream",children:"Register input stream"}),"\n",(0,r.jsx)(t.pre,{children:(0,r.jsx)(t.code,{className:"language-typescript",children:'type RegisterInputStream = {\n  type: "register";\n  entity_type: "rtp_input_stream" | "mp4";\n  ... // input specific options\n}\n'})}),"\n",(0,r.jsx)(t.p,{children:"Register external source that can be used as a compositor input. See inputs documentation to learn more."}),"\n",(0,r.jsxs)(t.ul,{children:["\n",(0,r.jsx)(t.li,{children:(0,r.jsx)(t.a,{href:"/docs/api/inputs/rtp",children:"RTP"})}),"\n",(0,r.jsx)(t.li,{children:(0,r.jsx)(t.a,{href:"/docs/api/inputs/mp4",children:"MP4"})}),"\n"]}),"\n",(0,r.jsx)(t.hr,{}),"\n",(0,r.jsx)(t.h3,{id:"register-output-stream",children:"Register output stream"}),"\n",(0,r.jsx)(t.pre,{children:(0,r.jsx)(t.code,{className:"language-typescript",children:'type RegisterOutputStream = {\n  type: "register";\n  entity_type: "output_stream";\n  ...\n}\n'})}),"\n",(0,r.jsx)(t.p,{children:"Register external destination that can be used as a compositor output. See outputs documentation to learn more."}),"\n",(0,r.jsxs)(t.ul,{children:["\n",(0,r.jsx)(t.li,{children:(0,r.jsx)(t.a,{href:"/docs/api/outputs/rtp",children:"RTP"})}),"\n"]}),"\n",(0,r.jsx)(t.hr,{}),"\n",(0,r.jsx)(t.h3,{id:"register-renderer",children:"Register renderer"}),"\n",(0,r.jsx)(t.pre,{children:(0,r.jsx)(t.code,{className:"language-typescript",children:'type RegisterRenderer = {\n  type: "register";\n  entity_type: "shader" | "web_renderer" | "image";\n  ... // renderer specific options\n}\n'})}),"\n",(0,r.jsx)(t.p,{children:"See renderers documentation to learn more."}),"\n",(0,r.jsxs)(t.ul,{children:["\n",(0,r.jsx)(t.li,{children:(0,r.jsx)(t.a,{href:"./renderers/image",children:"Image"})}),"\n",(0,r.jsx)(t.li,{children:(0,r.jsx)(t.a,{href:"./renderers/shader",children:"Shader"})}),"\n",(0,r.jsx)(t.li,{children:(0,r.jsx)(t.a,{href:"./renderers/web",children:"WebRenderer"})}),"\n"]}),"\n",(0,r.jsx)(t.hr,{}),"\n",(0,r.jsx)(t.h3,{id:"unregister-request",children:"Unregister request"}),"\n",(0,r.jsx)(t.pre,{children:(0,r.jsx)(t.code,{className:"language-typescript",children:'type Unregister =\n  | {\n    type: "unregister";\n    entity_type: "input_stream";\n    input_id: string;\n    schedule_time_ms: number;\n  }\n  | {\n    type: "unregister";\n    entity_type: "output_stream";\n    output_id: string;\n    schedule_time_ms: number;\n  }\n  | { type: "unregister"; entity_type: "shader"; shader_id: string }\n  | { type: "unregister"; entity_type: "image"; image_id: string }\n  | { type: "unregister"; entity_type: "web_renderer"; instance_id: string }\n'})}),"\n",(0,r.jsxs)(t.p,{children:["Removes entities previously registered with ",(0,r.jsx)(t.a,{href:"#register-input-stream",children:"register input"}),", ",(0,r.jsx)(t.a,{href:"#register-output-stream",children:"register output"})," or ",(0,r.jsx)(t.a,{href:"#register-renderer",children:"register renderer"})," requests."]}),"\n",(0,r.jsxs)(t.ul,{children:["\n",(0,r.jsxs)(t.li,{children:[(0,r.jsx)(t.code,{children:"schedule_time_ms"})," - Time in milliseconds when this request should be applied. Value ",(0,r.jsx)(t.code,{children:"0"})," represents time of ",(0,r.jsx)(t.a,{href:"#start",children:"the start request"}),"."]}),"\n"]}),"\n",(0,r.jsxs)(t.h2,{id:"endpoint-get-status",children:["Endpoint ",(0,r.jsx)(t.code,{children:"GET /status"})]}),"\n",(0,r.jsx)(t.pre,{children:(0,r.jsx)(t.code,{className:"language-typescript",children:"type Response = {\n  instance_id: string\n}\n"})}),"\n",(0,r.jsxs)(t.p,{children:["Status/health check endpoint. Returns ",(0,r.jsx)(t.code,{children:"200 OK"}),"."]}),"\n",(0,r.jsxs)(t.ul,{children:["\n",(0,r.jsxs)(t.li,{children:[(0,r.jsx)(t.code,{children:"instance_id"})," - ID that can be provided using ",(0,r.jsx)(t.code,{children:"LIVE_COMPOSITOR_INSTANCE_ID"})," environment variable. Defaults to random value in the format ",(0,r.jsx)(t.code,{children:"live_compositor_{RANDOM_VALUE}"}),"."]}),"\n"]}),"\n",(0,r.jsxs)(t.h2,{id:"websocket-endpoint---ws",children:["WebSocket endpoint ",(0,r.jsx)(t.code,{children:"/--/ws"})]}),"\n",(0,r.jsxs)(t.p,{children:["Establish WebSocket connection to listen for LiveCompositor events. List of supported events and their descriptions can be found ",(0,r.jsx)(t.a,{href:"/docs/api/events",children:"here"}),"."]})]})}function p(e={}){const{wrapper:t}={...(0,s.a)(),...e.components};return t?(0,r.jsx)(t,{...e,children:(0,r.jsx)(a,{...e})}):a(e)}},1151:(e,t,n)=>{n.d(t,{Z:()=>o,a:()=>d});var r=n(7294);const s={},i=r.createContext(s);function d(e){const t=r.useContext(i);return r.useMemo((function(){return"function"==typeof e?e(t):{...t,...e}}),[t,e])}function o(e){let t;return t=e.disableParentContext?"function"==typeof e.components?e.components(s):e.components||s:d(e.components),r.createElement(i.Provider,{value:t},e.children)}}}]);