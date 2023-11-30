"use strict";(self.webpackChunkcompositor_live=self.webpackChunkcompositor_live||[]).push([[780],{8169:(e,n,t)=>{t.r(n),t.d(n,{assets:()=>a,contentTitle:()=>r,default:()=>p,frontMatter:()=>s,metadata:()=>l,toc:()=>d});var i=t(5893),o=t(1151);const s={},r="Rescaler",l={id:"api/components/Rescaler",title:"Rescaler",description:"Properties",source:"@site/pages/api/components/Rescaler.md",sourceDirName:"api/components",slug:"/api/components/Rescaler",permalink:"/docs/api/components/Rescaler",draft:!1,unlisted:!1,tags:[],version:"current",frontMatter:{},sidebar:"sidebar",previous:{title:"InputStream",permalink:"/docs/api/components/InputStream"},next:{title:"Shader",permalink:"/docs/api/components/Shader"}},a={},d=[{value:"Properties",id:"properties",level:4},{value:"Transition",id:"transition",level:2}];function c(e){const n={code:"code",h1:"h1",h2:"h2",h4:"h4",li:"li",pre:"pre",ul:"ul",...(0,o.a)(),...e.components};return(0,i.jsxs)(i.Fragment,{children:[(0,i.jsx)(n.h1,{id:"rescaler",children:"Rescaler"}),"\n",(0,i.jsx)(n.pre,{children:(0,i.jsx)(n.code,{className:"language-typescript",children:'type Rescaler = {\n  type: "rescaler",\n  id?: string,\n  child: Component,\n  mode?: "fit" | "fill",\n  horizontal_align?: "left" | "right" | "justified" | "center",\n  vertical_align?: "top" | "center" | "bottom" | "justified",\n  width?: f32,\n  height?: f32,\n  top?: f32,\n  left?: f32,\n  bottom?: f32,\n  right?: f32,\n  rotation?: f32,\n  transition?: Transition,\n}\n'})}),"\n",(0,i.jsx)(n.h4,{id:"properties",children:"Properties"}),"\n",(0,i.jsxs)(n.ul,{children:["\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.code,{children:"width"})," - Width of a component in pixels. Required when using absolute positioning."]}),"\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.code,{children:"height"})," - Height of a component in pixels. Required when using absolute positioning."]}),"\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.code,{children:"top"})," - Distance between the top edge of this component and the top edge of its parent. If this field is defined, then component will ignore a layout defined by its parent."]}),"\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.code,{children:"left"})," - Distance between the left edge of this component and the left edge of its parent. If this field is defined, this element will be absolutely positioned, instead of being laid out by it's parent."]}),"\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.code,{children:"bottom"})," - Distance between the bottom edge of this component and the bottom edge of its parent. If this field is defined, this element will be absolutely positioned, instead of being laid out by it's parent."]}),"\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.code,{children:"right"})," - Distance between the right edge of this component and the right edge of its parent. If this field is defined, this element will be absolutely positioned, instead of being laid out by it's parent."]}),"\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.code,{children:"rotation"})," - Rotation of a component in degrees. If this field is defined, this element will be absolutely positioned, instead of being laid out by it's parent."]}),"\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.code,{children:"transition"})," - Defines how this component will behave during a scene update. This will only have an effect if previous scene already contained a View component with the same id."]}),"\n"]}),"\n",(0,i.jsx)(n.h2,{id:"transition",children:"Transition"}),"\n",(0,i.jsx)(n.pre,{children:(0,i.jsx)(n.code,{className:"language-typescript",children:"type Transition = {\n  duration_ms: f64,\n}\n"})})]})}function p(e={}){const{wrapper:n}={...(0,o.a)(),...e.components};return n?(0,i.jsx)(n,{...e,children:(0,i.jsx)(c,{...e})}):c(e)}},1151:(e,n,t)=>{t.d(n,{Z:()=>l,a:()=>r});var i=t(7294);const o={},s=i.createContext(o);function r(e){const n=i.useContext(s);return i.useMemo((function(){return"function"==typeof e?e(n):{...n,...e}}),[n,e])}function l(e){let n;return n=e.disableParentContext?"function"==typeof e.components?e.components(o):e.components||o:r(e.components),i.createElement(s.Provider,{value:n},e.children)}}}]);