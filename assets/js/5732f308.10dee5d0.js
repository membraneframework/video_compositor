"use strict";(self.webpackChunkcompositor_live=self.webpackChunkcompositor_live||[]).push([[821,7721],{660:(e,n,t)=>{t.r(n),t.d(n,{assets:()=>c,contentTitle:()=>o,default:()=>x,frontMatter:()=>d,metadata:()=>l,toc:()=>h});var i=t(5893),s=t(1151),r=t(9820);const d={sidebar_position:5,hide_table_of_contents:!0},o="Text",l={id:"api/components/Text",title:"Text",description:"A component for rendering text.",source:"@site/pages/api/components/Text.md",sourceDirName:"api/components",slug:"/api/components/Text",permalink:"/docs/api/components/Text",draft:!1,unlisted:!1,tags:[],version:"current",sidebarPosition:5,frontMatter:{sidebar_position:5,hide_table_of_contents:!0},sidebar:"sidebar",previous:{title:"Tiles",permalink:"/docs/api/components/Tiles"},next:{title:"Shader",permalink:"/docs/api/components/Shader"}},c={},h=[];function a(e){const n={h1:"h1",p:"p",...(0,s.a)(),...e.components};return(0,i.jsxs)(i.Fragment,{children:[(0,i.jsx)(n.h1,{id:"text",children:"Text"}),"\n",(0,i.jsx)(n.p,{children:"A component for rendering text."}),"\n",(0,i.jsx)(r.default,{})]})}function x(e={}){const{wrapper:n}={...(0,s.a)(),...e.components};return n?(0,i.jsx)(n,{...e,children:(0,i.jsx)(a,{...e})}):a(e)}},9820:(e,n,t)=>{t.r(n),t.d(n,{assets:()=>l,contentTitle:()=>d,default:()=>a,frontMatter:()=>r,metadata:()=>o,toc:()=>c});var i=t(5893),s=t(1151);const r={},d=void 0,o={id:"api/generated/component-Text",title:"component-Text",description:"Text",source:"@site/pages/api/generated/component-Text.md",sourceDirName:"api/generated",slug:"/api/generated/component-Text",permalink:"/docs/api/generated/component-Text",draft:!1,unlisted:!1,tags:[],version:"current",frontMatter:{}},l={},c=[{value:"Text",id:"text",level:2},{value:"Properties",id:"properties",level:4}];function h(e){const n={a:"a",code:"code",h2:"h2",h4:"h4",li:"li",pre:"pre",strong:"strong",ul:"ul",...(0,s.a)(),...e.components};return(0,i.jsxs)(i.Fragment,{children:[(0,i.jsx)(n.h2,{id:"text",children:"Text"}),"\n",(0,i.jsx)(n.pre,{children:(0,i.jsx)(n.code,{className:"language-typescript",children:'type Text = {\n  id?: string;\n  text: string;\n  width?: f32;\n  height?: f32;\n  max_width?: f32;\n  max_height?: f32;\n  font_size: f32;\n  line_height?: f32;\n  color_rgba?: string;\n  background_color_rgba?: string;\n  font_family?: string;\n  style?: "normal" | "italic" | "oblique";\n  align?: "left" | "right" | "justified" | "center";\n  wrap?: "none" | "glyph" | "word";\n  weight?: \n    | "thin"\n    | "extra_light"\n    | "light"\n    | "normal"\n    | "medium"\n    | "semi_bold"\n    | "bold"\n    | "extra_bold"\n    | "black";\n}\n'})}),"\n",(0,i.jsx)(n.h4,{id:"properties",children:"Properties"}),"\n",(0,i.jsxs)(n.ul,{children:["\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.code,{children:"id"})," - Id of a component."]}),"\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.code,{children:"text"})," - Text that will be rendered."]}),"\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.code,{children:"width"})," - Width of a texture that text will be rendered on. If not provided, the resulting texture will be sized based on the defined text but limited to ",(0,i.jsx)(n.code,{children:"max_width"})," value."]}),"\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.code,{children:"height"})," - Height of a texture that text will be rendered on. If not provided, the resulting texture will be sized based on the defined text but limited to ",(0,i.jsx)(n.code,{children:"max_height"})," value.\nIt's an error to provide ",(0,i.jsx)(n.code,{children:"height"})," if ",(0,i.jsx)(n.code,{children:"width"})," is not defined."]}),"\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.code,{children:"max_width"})," - (",(0,i.jsxs)(n.strong,{children:["default=",(0,i.jsx)(n.code,{children:"7682"})]}),") Maximal ",(0,i.jsx)(n.code,{children:"width"}),". Limits the width of the texture that the text will be rendered on. Value is ignored if ",(0,i.jsx)(n.code,{children:"width"})," is defined."]}),"\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.code,{children:"max_height"})," - (",(0,i.jsxs)(n.strong,{children:["default=",(0,i.jsx)(n.code,{children:"4320"})]}),") Maximal ",(0,i.jsx)(n.code,{children:"height"}),". Limits the height of the texture that the text will be rendered on. Value is ignored if height is defined."]}),"\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.code,{children:"font_size"})," - Font size in pixels."]}),"\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.code,{children:"line_height"})," - Distance between lines in pixels. Defaults to the value of the ",(0,i.jsx)(n.code,{children:"font_size"})," property."]}),"\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.code,{children:"color_rgba"})," - (",(0,i.jsxs)(n.strong,{children:["default=",(0,i.jsx)(n.code,{children:'"#FFFFFFFF"'})]}),") Font color in ",(0,i.jsx)(n.code,{children:"#RRGGBBAA"})," format."]}),"\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.code,{children:"background_color_rgba"})," - (",(0,i.jsxs)(n.strong,{children:["default=",(0,i.jsx)(n.code,{children:'"#00000000"'})]}),") Background color in ",(0,i.jsx)(n.code,{children:"#RRGGBBAA"})," format."]}),"\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.code,{children:"font_family"})," - (",(0,i.jsxs)(n.strong,{children:["default=",(0,i.jsx)(n.code,{children:'"Verdana"'})]}),") Font family. Provide ",(0,i.jsx)(n.a,{href:"https://www.w3.org/TR/2018/REC-css-fonts-3-20180920/#family-name-value",children:"family-name"}),' for a specific font. "generic-family" values like e.g. "sans-serif" will not work.']}),"\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.code,{children:"style"})," - (",(0,i.jsxs)(n.strong,{children:["default=",(0,i.jsx)(n.code,{children:'"normal"'})]}),") Font style. The selected font needs to support the specified style."]}),"\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.code,{children:"align"})," - (",(0,i.jsxs)(n.strong,{children:["default=",(0,i.jsx)(n.code,{children:'"left"'})]}),") Text align."]}),"\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.code,{children:"wrap"})," - (",(0,i.jsxs)(n.strong,{children:["default=",(0,i.jsx)(n.code,{children:'"none"'})]}),") Text wrapping options.","\n",(0,i.jsxs)(n.ul,{children:["\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.code,{children:'"none"'})," - Disable text wrapping. Text that does not fit inside the texture will be cut off."]}),"\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.code,{children:'"glyph"'})," - Wraps at a glyph level."]}),"\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.code,{children:'"word"'})," - Wraps at a word level. Prevent splitting words when wrapping."]}),"\n"]}),"\n"]}),"\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.code,{children:"weight"})," - (",(0,i.jsxs)(n.strong,{children:["default=",(0,i.jsx)(n.code,{children:'"normal"'})]}),") Font weight. The selected font needs to support the specified weight. Font weight, based on the ",(0,i.jsx)(n.a,{href:"https://learn.microsoft.com/en-gb/typography/opentype/spec/os2#usweightclass",children:"OpenType specification"}),".","\n",(0,i.jsxs)(n.ul,{children:["\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.code,{children:'"thin"'})," - Weight 100."]}),"\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.code,{children:'"extra_light"'})," - Weight 200."]}),"\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.code,{children:'"light"'})," - Weight 300."]}),"\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.code,{children:'"normal"'})," - Weight 400."]}),"\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.code,{children:'"medium"'})," - Weight 500."]}),"\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.code,{children:'"semi_bold"'})," - Weight 600."]}),"\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.code,{children:'"bold"'})," - Weight 700."]}),"\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.code,{children:'"extra_bold"'})," - Weight 800."]}),"\n",(0,i.jsxs)(n.li,{children:[(0,i.jsx)(n.code,{children:'"black"'})," - Weight 900."]}),"\n"]}),"\n"]}),"\n"]})]})}function a(e={}){const{wrapper:n}={...(0,s.a)(),...e.components};return n?(0,i.jsx)(n,{...e,children:(0,i.jsx)(h,{...e})}):h(e)}},1151:(e,n,t)=>{t.d(n,{Z:()=>o,a:()=>d});var i=t(7294);const s={},r=i.createContext(s);function d(e){const n=i.useContext(r);return i.useMemo((function(){return"function"==typeof e?e(n):{...n,...e}}),[n,e])}function o(e){let n;return n=e.disableParentContext?"function"==typeof e.components?e.components(s):e.components||s:d(e.components),i.createElement(r.Provider,{value:n},e.children)}}}]);