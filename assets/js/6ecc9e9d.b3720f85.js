"use strict";(self.webpackChunkcompositor_live=self.webpackChunkcompositor_live||[]).push([[8646,580,7598],{6239:(e,n,i)=>{i.r(n),i.d(n,{assets:()=>a,contentTitle:()=>l,default:()=>u,frontMatter:()=>d,metadata:()=>c,toc:()=>h});var t=i(5893),o=i(1151),s=i(5660),r=i(9342);const d={sidebar_position:2,hide_table_of_contents:!0},l="View",c={id:"api/components/View",title:"View",description:"View is the compositor's core layout mechanism. Its role is analogous to the",source:"@site/pages/api/components/View.md",sourceDirName:"api/components",slug:"/api/components/View",permalink:"/docs/api/components/View",draft:!1,unlisted:!1,tags:[],version:"current",sidebarPosition:2,frontMatter:{sidebar_position:2,hide_table_of_contents:!0},sidebar:"sidebar",previous:{title:"InputStream",permalink:"/docs/api/components/InputStream"},next:{title:"Rescaler",permalink:"/docs/api/components/Rescaler"}},a={},h=[{value:"Absolute positioning",id:"absolute-positioning",level:3},{value:"Static positioning",id:"static-positioning",level:3},{value:"For <code>direction=row</code>:",id:"for-directionrow",level:4},{value:"For <code>direction=column</code>:",id:"for-directioncolumn",level:4},{value:"Transitions",id:"transitions",level:3}];function p(e){const n={code:"code",h1:"h1",h3:"h3",h4:"h4",li:"li",p:"p",ul:"ul",...(0,o.a)(),...e.components};return(0,t.jsxs)(t.Fragment,{children:[(0,t.jsx)(n.h1,{id:"view",children:"View"}),"\n",(0,t.jsxs)(n.p,{children:[(0,t.jsx)(n.code,{children:"View"})," is the compositor's core layout mechanism. Its role is analogous to the\n",(0,t.jsx)(n.code,{children:"<div>"})," tag in HTML. It provides a container with basic styling that can be further composed."]}),"\n",(0,t.jsx)(n.h3,{id:"absolute-positioning",children:"Absolute positioning"}),"\n",(0,t.jsx)(r.default,{}),"\n",(0,t.jsxs)(n.ul,{children:["\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:"View"})," supports absolute positioning for its child components."]}),"\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:"View"})," can be absolutely positioned relative to its parent if the parent component supports it."]}),"\n"]}),"\n",(0,t.jsx)(n.h3,{id:"static-positioning",children:"Static positioning"}),"\n",(0,t.jsxs)(n.p,{children:["When children of a ",(0,t.jsx)(n.code,{children:"View"})," component have a static position, they are placed next to each other."]}),"\n",(0,t.jsxs)(n.h4,{id:"for-directionrow",children:["For ",(0,t.jsx)(n.code,{children:"direction=row"}),":"]}),"\n",(0,t.jsxs)(n.p,{children:["Children of a ",(0,t.jsx)(n.code,{children:"View"})," component form a row, with items aligned to the top. The size of each child will be calculated in the following way:"]}),"\n",(0,t.jsxs)(n.ul,{children:["\n",(0,t.jsxs)(n.li,{children:["If the ",(0,t.jsx)(n.code,{children:"width"})," or ",(0,t.jsx)(n.code,{children:"height"})," of a child component is defined, then those values take priority."]}),"\n",(0,t.jsxs)(n.li,{children:["If the ",(0,t.jsx)(n.code,{children:"height"})," is not defined, the component will have the same ",(0,t.jsx)(n.code,{children:"height"})," as its parent."]}),"\n",(0,t.jsxs)(n.li,{children:["If the ",(0,t.jsx)(n.code,{children:"width"})," is not defined, we calculate the sum ",(0,t.jsx)(n.code,{children:"width"})," of all components with that value defined.","\n",(0,t.jsxs)(n.ul,{children:["\n",(0,t.jsxs)(n.li,{children:["If it is larger than the parent's ",(0,t.jsx)(n.code,{children:"width"}),", then the ",(0,t.jsx)(n.code,{children:"width"})," of the rest of the components is zero."]}),"\n",(0,t.jsxs)(n.li,{children:["If it is smaller than the parent's ",(0,t.jsx)(n.code,{children:"width"}),", calculate the difference and divide the resulting value equally between all children with unknown widths."]}),"\n"]}),"\n"]}),"\n"]}),"\n",(0,t.jsxs)(n.h4,{id:"for-directioncolumn",children:["For ",(0,t.jsx)(n.code,{children:"direction=column"}),":"]}),"\n",(0,t.jsxs)(n.p,{children:["Analogous to the ",(0,t.jsx)(n.code,{children:"direction=row"})," case, but children form a column instead, with items aligned to the left."]}),"\n",(0,t.jsx)(n.h3,{id:"transitions",children:"Transitions"}),"\n",(0,t.jsxs)(n.p,{children:["On the scene update, a ",(0,t.jsx)(n.code,{children:"View"})," component will animate between the original state and the new one if the ",(0,t.jsx)(n.code,{children:"transition"})," field is defined. Both the original and the new scene need to define a component with the same ",(0,t.jsx)(n.code,{children:"id"}),". Currently, only some of the fields support animated transitions:"]}),"\n",(0,t.jsxs)(n.ul,{children:["\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:"width"})," / ",(0,t.jsx)(n.code,{children:"height"})," - Only supported within the same positioning mode. If the positioning mode changes between the old scene and the new one, the transition will not work."]}),"\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:"bottom"})," / ",(0,t.jsx)(n.code,{children:"top"})," / ",(0,t.jsx)(n.code,{children:"left"})," / ",(0,t.jsx)(n.code,{children:"right"})," / ",(0,t.jsx)(n.code,{children:"rotation"})," - Only supports transition when changing a value of the same field. If the old scene defines a ",(0,t.jsx)(n.code,{children:"left"})," field and the new one does not, the transition will not work."]}),"\n"]}),"\n",(0,t.jsx)(s.default,{})]})}function u(e={}){const{wrapper:n}={...(0,o.a)(),...e.components};return n?(0,t.jsx)(n,{...e,children:(0,t.jsx)(p,{...e})}):p(e)}},5660:(e,n,i)=>{i.r(n),i.d(n,{assets:()=>l,contentTitle:()=>r,default:()=>h,frontMatter:()=>s,metadata:()=>d,toc:()=>c});var t=i(5893),o=i(1151);const s={},r=void 0,d={id:"api/generated/component-View",title:"component-View",description:"View",source:"@site/pages/api/generated/component-View.md",sourceDirName:"api/generated",slug:"/api/generated/component-View",permalink:"/docs/api/generated/component-View",draft:!1,unlisted:!1,tags:[],version:"current",frontMatter:{}},l={},c=[{value:"View",id:"view",level:2},{value:"Properties",id:"properties",level:4},{value:"Transition",id:"transition",level:2},{value:"Properties",id:"properties-1",level:4},{value:"EasingFunction",id:"easingfunction",level:2}];function a(e){const n={a:"a",admonition:"admonition",code:"code",h2:"h2",h4:"h4",li:"li",p:"p",pre:"pre",strong:"strong",ul:"ul",...(0,o.a)(),...e.components};return(0,t.jsxs)(t.Fragment,{children:[(0,t.jsx)(n.h2,{id:"view",children:"View"}),"\n",(0,t.jsx)(n.pre,{children:(0,t.jsx)(n.code,{className:"language-typescript",children:'type View = {\n  id?: string;\n  children?: Component[];\n  width?: f32;\n  height?: f32;\n  direction?: "row" | "column";\n  top?: f32;\n  left?: f32;\n  bottom?: f32;\n  right?: f32;\n  rotation?: f32;\n  transition?: Transition;\n  overflow?: "visible" | "hidden" | "fit";\n  background_color_rgba?: string;\n}\n'})}),"\n",(0,t.jsx)(n.h4,{id:"properties",children:"Properties"}),"\n",(0,t.jsxs)(n.ul,{children:["\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:"id"})," - Id of a component."]}),"\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:"children"})," - List of component's children."]}),"\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:"width"})," - Width of a component in pixels. Required when using absolute positioning."]}),"\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:"height"})," - Height of a component in pixels. Required when using absolute positioning."]}),"\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:"direction"})," - Direction defines how static children are positioned inside a View component.","\n",(0,t.jsxs)(n.ul,{children:["\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:'"row"'})," - Children positioned from left to right."]}),"\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:'"column"'})," - Children positioned from top to bottom."]}),"\n"]}),"\n"]}),"\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:"top"})," - Distance in pixels between this component's top edge and its parent's top edge. If this field is defined, then the component will ignore a layout defined by its parent."]}),"\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:"left"})," - Distance in pixels between this component's left edge and its parent's left edge. If this field is defined, this element will be absolutely positioned, instead of being laid out by its parent."]}),"\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:"bottom"})," - Distance in pixels between the bottom edge of this component and the bottom edge of its parent. If this field is defined, this element will be absolutely positioned, instead of being laid out by its parent."]}),"\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:"right"})," - Distance in pixels between this component's right edge and its parent's right edge. If this field is defined, this element will be absolutely positioned, instead of being laid out by its parent."]}),"\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:"rotation"})," - Rotation of a component in degrees. If this field is defined, this element will be absolutely positioned, instead of being laid out by its parent."]}),"\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:"transition"})," - Defines how this component will behave during a scene update. This will only have an effect if the previous scene already contained a View component with the same id."]}),"\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:"overflow"})," - (",(0,t.jsxs)(n.strong,{children:["default=",(0,t.jsx)(n.code,{children:'"hidden"'})]}),") Controls what happens to content that is too big to fit into an area.","\n",(0,t.jsxs)(n.ul,{children:["\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:'"visible"'})," - Render everything, including content that extends beyond their parent."]}),"\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:'"hidden"'})," - Render only parts of the children that are inside their parent area."]}),"\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:'"fit"'})," - If children components are too big to fit inside the parent, resize everything inside to fit.\nComponents that have unknown sizes will be treated as if they had a size 0 when calculating scaling factor.","\n",(0,t.jsx)("br",{}),"\n",(0,t.jsx)("br",{}),"\n",(0,t.jsx)(n.admonition,{type:"warning",children:(0,t.jsx)(n.p,{children:"This will resize everything inside, even absolutely positioned elements. For example, if you have an element in the bottom right corner and the content will be rescaled by a factor 0.5x, then that component will end up in the middle of its parent"})}),"\n"]}),"\n"]}),"\n"]}),"\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:"background_color_rgba"})," - (",(0,t.jsxs)(n.strong,{children:["default=",(0,t.jsx)(n.code,{children:'"#00000000"'})]}),") Background color in a ",(0,t.jsx)(n.code,{children:'"#RRGGBBAA"'})," format."]}),"\n"]}),"\n",(0,t.jsx)(n.h2,{id:"transition",children:"Transition"}),"\n",(0,t.jsx)(n.pre,{children:(0,t.jsx)(n.code,{className:"language-typescript",children:"type Transition = {\n  duration_ms: f64;\n  easing_function?: EasingFunction;\n}\n"})}),"\n",(0,t.jsx)(n.h4,{id:"properties-1",children:"Properties"}),"\n",(0,t.jsxs)(n.ul,{children:["\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:"duration_ms"})," - Duration of a transition in milliseconds."]}),"\n",(0,t.jsxs)(n.li,{children:[(0,t.jsx)(n.code,{children:"easing_function"})," - (",(0,t.jsxs)(n.strong,{children:["default=",(0,t.jsx)(n.code,{children:'"linear"'})]}),") Easing function to be used for the transition."]}),"\n"]}),"\n",(0,t.jsx)(n.h2,{id:"easingfunction",children:"EasingFunction"}),"\n",(0,t.jsx)(n.pre,{children:(0,t.jsx)(n.code,{className:"language-typescript",children:'type EasingFunction = \n  | { function_name: "linear" }\n  | { function_name: "bounce" }\n  | {\n      function_name: "cubic_bezier";\n      points: [f64, f64, f64, f64];\n    }\n'})}),"\n",(0,t.jsx)(n.p,{children:"Easing functions are used to interpolate between two values over time."}),"\n",(0,t.jsxs)(n.p,{children:["Custom easing functions can be implemented with cubic B\xe9zier. The control points are defined with ",(0,t.jsx)(n.code,{children:"points"})," field by providing four numerical values: ",(0,t.jsx)(n.code,{children:"x1"}),", ",(0,t.jsx)(n.code,{children:"y1"}),", ",(0,t.jsx)(n.code,{children:"x2"})," and ",(0,t.jsx)(n.code,{children:"y2"}),". The ",(0,t.jsx)(n.code,{children:"x1"})," and ",(0,t.jsx)(n.code,{children:"x2"})," values have to be in the range ",(0,t.jsx)(n.code,{children:"[0; 1]"}),". The cubic B\xe9zier result is clamped to the range ",(0,t.jsx)(n.code,{children:"[0; 1]"}),". You can find example control point configurations ",(0,t.jsx)(n.a,{href:"https://easings.net/",children:"here"}),"."]})]})}function h(e={}){const{wrapper:n}={...(0,o.a)(),...e.components};return n?(0,t.jsx)(n,{...e,children:(0,t.jsx)(a,{...e})}):a(e)}},9342:(e,n,i)=>{i.r(n),i.d(n,{assets:()=>l,contentTitle:()=>r,default:()=>h,frontMatter:()=>s,metadata:()=>d,toc:()=>c});var t=i(5893),o=i(1151);const s={},r=void 0,d={id:"common/absolute-position",title:"absolute-position",description:"A component is absolutely positioned if it defines fields like top, left, right, bottom, or rotation.",source:"@site/pages/common/absolute-position.md",sourceDirName:"common",slug:"/common/absolute-position",permalink:"/docs/common/absolute-position",draft:!1,unlisted:!1,tags:[],version:"current",frontMatter:{}},l={},c=[];function a(e){const n={p:"p",...(0,o.a)(),...e.components};return(0,t.jsx)(n.p,{children:"A component is absolutely positioned if it defines fields like top, left, right, bottom, or rotation.\nThose fields define the component's position relative to its parent. However, to respect those values,\nthe parent component has to be a layout component that supports absolute positioning."})}function h(e={}){const{wrapper:n}={...(0,o.a)(),...e.components};return n?(0,t.jsx)(n,{...e,children:(0,t.jsx)(a,{...e})}):a(e)}},1151:(e,n,i)=>{i.d(n,{Z:()=>d,a:()=>r});var t=i(7294);const o={},s=t.createContext(o);function r(e){const n=t.useContext(s);return t.useMemo((function(){return"function"==typeof e?e(n):{...n,...e}}),[n,e])}function d(e){let n;return n=e.disableParentContext?"function"==typeof e.components?e.components(o):e.components||o:r(e.components),t.createElement(s.Provider,{value:n},e.children)}}}]);