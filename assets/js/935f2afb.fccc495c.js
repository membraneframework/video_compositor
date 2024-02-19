"use strict";(self.webpackChunkcompositor_live=self.webpackChunkcompositor_live||[]).push([[53],{1109:e=>{e.exports=JSON.parse('{"pluginId":"default","version":"current","label":"Next","banner":null,"badge":false,"noIndex":false,"className":"docs-version-current","isLast":true,"docsSidebars":{"sidebar":[{"type":"link","label":"Introduction","href":"/docs/intro","docId":"intro","unlisted":false},{"label":"Get started","type":"category","items":[{"type":"link","label":"Elixir","href":"/docs/get-started/elixir","docId":"get-started/elixir","unlisted":false},{"type":"link","label":"Node.js","href":"/docs/get-started/node","docId":"get-started/node","unlisted":false}],"collapsed":true,"collapsible":true,"href":"/docs/get-started"},{"type":"category","label":"Concepts","collapsible":false,"items":[{"type":"link","label":"Component","href":"/docs/concept/component","docId":"concept/component","unlisted":false},{"type":"link","label":"Layouts","href":"/docs/concept/layouts","docId":"concept/layouts","unlisted":false},{"type":"link","label":"Shaders","href":"/docs/concept/shaders","docId":"concept/shaders","unlisted":false}],"collapsed":false},{"type":"category","label":"Deployment","collapsible":true,"items":[{"type":"link","label":"Configuration","href":"/docs/deployment/configuration","docId":"deployment/configuration","unlisted":false}],"collapsed":true,"href":"/docs/category/deployment"},{"type":"category","label":"API Reference","collapsible":false,"items":[{"type":"link","label":"HTTP Routes","href":"/docs/api/routes","docId":"api/routes","unlisted":false},{"type":"category","label":"Components","collapsible":false,"description":"Basic blocks used to define a scene.","items":[{"type":"link","label":"InputStream","href":"/docs/api/components/InputStream","docId":"api/components/InputStream","unlisted":false},{"type":"link","label":"View","href":"/docs/api/components/View","docId":"api/components/View","unlisted":false},{"type":"link","label":"Rescaler","href":"/docs/api/components/Rescaler","docId":"api/components/Rescaler","unlisted":false},{"type":"link","label":"Tiles","href":"/docs/api/components/Tiles","docId":"api/components/Tiles","unlisted":false},{"type":"link","label":"Text","href":"/docs/api/components/Text","docId":"api/components/Text","unlisted":false},{"type":"link","label":"Shader","href":"/docs/api/components/Shader","docId":"api/components/Shader","unlisted":false},{"type":"link","label":"Image","href":"/docs/api/components/Image","docId":"api/components/Image","unlisted":false},{"type":"link","label":"WebView","href":"/docs/api/components/WebView","docId":"api/components/WebView","unlisted":false}],"collapsed":false},{"type":"category","label":"Renderers","collapsible":false,"description":"Resources that need to be registered first before they can be used.","items":[{"type":"link","label":"Shader","href":"/docs/api/renderers/shader","docId":"api/renderers/shader","unlisted":false},{"type":"link","label":"Image","href":"/docs/api/renderers/image","docId":"api/renderers/image","unlisted":false},{"type":"link","label":"Web Renderer","href":"/docs/api/renderers/web","docId":"api/renderers/web","unlisted":false}],"collapsed":false},{"type":"category","label":"Inputs","collapsible":false,"description":"Elements that deliver media from external sources.","items":[{"type":"link","label":"RTP","href":"/docs/api/inputs/rtp","docId":"api/inputs/rtp","unlisted":false},{"type":"link","label":"MP4","href":"/docs/api/inputs/mp4","docId":"api/inputs/mp4","unlisted":false}],"collapsed":false}],"collapsed":false,"href":"/docs/category/api-reference"}]},"docs":{"api/components/Image":{"id":"api/components/Image","title":"Image","description":"A component for rendering images.","sidebar":"sidebar"},"api/components/InputStream":{"id":"api/components/InputStream","title":"InputStream","description":"InputStream represents an incoming RTP stream.","sidebar":"sidebar"},"api/components/Rescaler":{"id":"api/components/Rescaler","title":"Rescaler","description":"Rescaler is a layout component responsible for rescaling other components.","sidebar":"sidebar"},"api/components/Shader":{"id":"api/components/Shader","title":"Shader","description":"Shader applies transformation defined via WGSL shader on its children. Learn more.","sidebar":"sidebar"},"api/components/Text":{"id":"api/components/Text","title":"Text","description":"A component for rendering text.","sidebar":"sidebar"},"api/components/Tiles":{"id":"api/components/Tiles","title":"Tiles","description":"Tiles is a layout component that places all the child components next to each other while maximizing the use of available space. The component divides its area into multiple rectangles/tiles, one for each child component. All of those rectangles are the same size and do not overlap over each other.","sidebar":"sidebar"},"api/components/View":{"id":"api/components/View","title":"View","description":"View is the compositor\'s core layout mechanism. Its role is analogous to the","sidebar":"sidebar"},"api/components/WebView":{"id":"api/components/WebView","title":"WebView","description":"WebView renders a website using Chromium engine embedded inside the compositor.","sidebar":"sidebar"},"api/generated/component-Image":{"id":"api/generated/component-Image","title":"component-Image","description":"Image"},"api/generated/component-InputStream":{"id":"api/generated/component-InputStream","title":"component-InputStream","description":"InputStream"},"api/generated/component-Rescaler":{"id":"api/generated/component-Rescaler","title":"component-Rescaler","description":"Rescaler"},"api/generated/component-Text":{"id":"api/generated/component-Text","title":"component-Text","description":"Text"},"api/generated/component-Tiles":{"id":"api/generated/component-Tiles","title":"component-Tiles","description":"Tiles"},"api/generated/component-View":{"id":"api/generated/component-View","title":"component-View","description":"View"},"api/generated/component-WebView":{"id":"api/generated/component-WebView","title":"component-WebView","description":"WebView"},"api/generated/renderer-Mp4":{"id":"api/generated/renderer-Mp4","title":"renderer-Mp4","description":"Mp4"},"api/generated/renderer-RtpInputStream":{"id":"api/generated/renderer-RtpInputStream","title":"renderer-RtpInputStream","description":"RtpInputStream"},"api/generated/renderer-Shader":{"id":"api/generated/renderer-Shader","title":"renderer-Shader","description":"Shader"},"api/generated/renderer-WebRenderer":{"id":"api/generated/renderer-WebRenderer","title":"renderer-WebRenderer","description":"WebRenderer"},"api/inputs/mp4":{"id":"api/inputs/mp4","title":"MP4","description":"An input type that allows the compositor to read static MP4 files.","sidebar":"sidebar"},"api/inputs/rtp":{"id":"api/inputs/rtp","title":"RTP","description":"An input type that allows streaming video and audio to the compositor over RTP.","sidebar":"sidebar"},"api/renderers/image":{"id":"api/renderers/image","title":"Image","description":"Represents an image asset uploaded to the compositor. Used by a Image component.","sidebar":"sidebar"},"api/renderers/shader":{"id":"api/renderers/shader","title":"Shader","description":"Represents compiled shader. Used by a Shader component.","sidebar":"sidebar"},"api/renderers/web":{"id":"api/renderers/web","title":"Web Renderer","description":"Represents an instance of a website opened with Chromimum embeded inside the compositor. Used by a WebView component. Only one WebView component can use a specific instance at a time.","sidebar":"sidebar"},"api/routes":{"id":"api/routes","title":"Routes","description":"API routes to configure the compositor.","sidebar":"sidebar"},"common/absolute-position":{"id":"common/absolute-position","title":"absolute-position","description":"A component is absolutely positioned if it defines fields like top, left, right, bottom, or rotation."},"concept/component":{"id":"concept/component","title":"Component","description":"A component is a basic block used to define how video streams are composed.","sidebar":"sidebar"},"concept/layouts":{"id":"concept/layouts","title":"Layouts","description":"Layout components define the size, position, and simple styling of other components.","sidebar":"sidebar"},"concept/shaders":{"id":"concept/shaders","title":"Shaders","description":"Shaders are small programs that we send to a GPU to perform some computation for us. They are used extensively in the video compositor. All builtin transformation are implemented as shaders under the hood. It is also possible to create render nodes that run a custom shader on their input. Since video compositor is implemented using wgpu, the shaders have to be written in WGSL (WebGPU Shading Language). They also have to fulfill some custom requirements that allow them to be run by the video compositor.","sidebar":"sidebar"},"deployment/configuration":{"id":"deployment/configuration","title":"Configuration","description":"Environment variables","sidebar":"sidebar"},"get-started":{"id":"get-started","title":"Get started","description":"To familiarize yourself with a compositor you can start with examples directory. It includes example applications that use ffmpeg and ffplay to simulate compositor inputs and outputs. For a more detailed explanation of some of the terms used in this documentation, you can check this page.","sidebar":"sidebar"},"get-started/elixir":{"id":"get-started/elixir","title":"Elixir","description":"See Membrane Live Compositor plugin for more.","sidebar":"sidebar"},"get-started/node":{"id":"get-started/node","title":"Node.js","description":"See github.com/membraneframework-labs/rtconvideocompositorworkshops for example usage.","sidebar":"sidebar"},"intro":{"id":"intro","title":"Introduction","description":"Live compositor is an application for real-time video processing/transforming/composing, providing simple, language-agnostic API for live video rendering. It targets real-time use cases, like video conferencing, live-streaming, or broadcasting (e.g. with WebRTC / HLS / RTMP).","sidebar":"sidebar"}}}')}}]);