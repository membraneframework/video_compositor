"use strict";(self.webpackChunkcompositor_live=self.webpackChunkcompositor_live||[]).push([[2676],{3393:(e,n,r)=>{r.r(n),r.d(n,{assets:()=>c,contentTitle:()=>o,default:()=>u,frontMatter:()=>d,metadata:()=>s,toc:()=>a});var t=r(5893),i=r(1151);const d={},o=void 0,s={id:"api/generated/renderer-WebRenderer",title:"renderer-WebRenderer",description:"WebRenderer",source:"@site/pages/api/generated/renderer-WebRenderer.md",sourceDirName:"api/generated",slug:"/api/generated/renderer-WebRenderer",permalink:"/docs/api/generated/renderer-WebRenderer",draft:!1,unlisted:!1,tags:[],version:"current",frontMatter:{}},c={},a=[{value:"WebRenderer",id:"webrenderer",level:2},{value:"Properties",id:"properties",level:4}];function l(e){const n={admonition:"admonition",code:"code",h2:"h2",h4:"h4",li:"li",p:"p",pre:"pre",ul:"ul",...(0,i.a)(),...e.components};return(0,t.jsxs)(t.Fragment,{children:[(0,t.jsx)(n.h2,{id:"webrenderer",children:"WebRenderer"}),"\n",(0,t.jsx)(n.pre,{children:(0,t.jsx)(n.code,{className:"language-typescript",children:'type WebRenderer = {\n  url: string;\n  resolution: {\n    width: u32;\n    height: u32;\n  };\n  embedding_method?: \n    | "chromium_embedding"\n    | "native_embedding_over_content"\n    | "native_embedding_under_content";\n}\n'})}),"\n",(0,t.jsx)(n.h4,{id:"properties",children:"Properties"}),"\n",(0,t.jsxs)(n.ul,{children:["\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:"url"})," - Url of a website that you want to render."]}),"\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:"resolution"})," - Resolution."]}),"\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:"embedding_method"})," - Mechanism used to render input frames on the website.","\n",(0,t.jsxs)(n.ul,{children:["\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:'"chromium_embedding"'})," - Pass raw input frames as JS buffers so they can be rendered, for example, using a ",(0,t.jsx)(n.code,{children:"<canvas>"})," component.","\n",(0,t.jsx)("br",{}),"\n",(0,t.jsx)("br",{}),"\n",(0,t.jsx)(n.admonition,{type:"warning",children:(0,t.jsx)(n.p,{children:"This method might have a significant performance impact, especially for a large number of inputs."})}),"\n"]}),"\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:'"native_embedding_over_content"'})," - Render a website without any inputs and overlay them over the website content."]}),"\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:'"native_embedding_under_content"'})," - Render a website without any inputs and overlay them under the website content."]}),"\n"]}),"\n"]}),"\n"]})]})}function u(e={}){const{wrapper:n}={...(0,i.a)(),...e.components};return n?(0,t.jsx)(n,{...e,children:(0,t.jsx)(l,{...e})}):l(e)}},1151:(e,n,r)=>{r.d(n,{Z:()=>s,a:()=>o});var t=r(7294);const i={},d=t.createContext(i);function o(e){const n=t.useContext(d);return t.useMemo((function(){return"function"==typeof e?e(n):{...n,...e}}),[n,e])}function s(e){let n;return n=e.disableParentContext?"function"==typeof e.components?e.components(i):e.components||i:o(e.components),t.createElement(d.Provider,{value:n},e.children)}}}]);