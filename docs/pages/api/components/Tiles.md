---
sidebar_position: 4
hide_table_of_contents: true
---
import Docs from "@site/pages/api/generated/component-Tiles.md"
import AbsolutePositionDefinition from "@site/pages/common/absolute-position.md"

# Tiles

`Tiles` is a layout component that places all the child components next to each other while maximizing the use of available space. The component divides its area into multiple rectangles/tiles, one for each child component. All of those rectangles are the same size and do not overlap over each other.

### Absolute positioning

<AbsolutePositionDefinition />

- `Tiles` **does not** support absolute positioning for its child components. All children will still be rendered, but all fields like `top`, `left`, `right`, `bottom`, and `rotation` will be ignored.
- `Tiles` **can not** be absolutely positioned relative to it's parent.

### Static positioning

The component calculates the number of rows and columns that children should be divided into. The result is based on:
- The size of the `Tiles` component.
- Aspect ratio of a single tile (`tile_aspect_ratio` field).
- Number of children components.

An optimal number of rows and columns should result in a layout that covers the biggest part of its area. Children components are placed based on their order, from left to right, and row-by-row from top to bottom.

When placing a child component inside a tile, the component might change its size.
- Non-layout component scales proportionally to fit inside the parent. If the aspect ratios of a child and its parent do not match, then the component will be centered vertically or horizontally.
- Layout component takes the `width` and `height` of a tile. It ignores its own `width`/`height` fields if they are defined.

### Transitions

The `Tiles` component currently does not support `width` / `height` changes in the same way that `View` or `Rescaler` support it. You can always wrap your `Tiles` component with a `View` component that have a transition.

Currently supported transitions:
- Adding a new child. All existing components move to their new position (with animation). At the end new component appears (without any animation).
- Removing existing child. At the start, the tile representing that child disappears (without any animation). All remaining components move to their new position (with animation).
- Changing order of child components.
- \[partial\] Responsiveness to size changes:
  - 


Operations like adding/removing/changing order of child components rely on specific way to identify components before scene update and after. Identity is resolved in a following way:
- If component has the `"id"` field then that its identifier.
- If component does not have the `"id"` field defined, then its identified based on order inside the `children` list. Only child components without an `"id"` are taken into account here. For example, if a component without and `"id"` was 1st before the update and now component with an id was added before it, then that now second component its still identified.

<Docs />
