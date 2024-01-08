import Docs from "@site/pages/api/generated/renderer-WebRenderer.md"

# Web Renderer

Represents an instance of a website opened with Chromimum embeded inside the compositor. Used by a [`WebView` component](../components/WebView). Only one `WebView` component can use a specific instance at a time. \
Before the web renderer can be used, you need to make sure that compositor version which includes `"web renderer feature"` is used, and the web renderer is enabled in the [init request](../routes#init).

<Docs />
