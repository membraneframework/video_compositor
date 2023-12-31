use std::time::Duration;

use crate::{
    scene::{
        layout::StatefulLayoutComponent, Overflow, Position, Size, StatefulComponent,
        ViewChildrenDirection,
    },
    transformations::layout::{Crop, LayoutContent, NestedLayout},
};

use super::ViewComponentParam;

#[derive(Debug)]
struct StaticChildLayoutOpts {
    width: Option<f32>,
    height: Option<f32>,
    /// Offset inside parent component (position where next static child should start).
    static_offset: f32,
    /// Define size(width or height) of a static component if it's not
    /// already defined explicitly.
    /// For direction=row defines width of a static component
    /// For direction=column defines height of a static component
    static_child_size: f32,
    parent_size: Size,
}

impl ViewComponentParam {
    pub(super) fn layout(
        &self,
        size: Size,
        children: &mut [StatefulComponent],
        pts: Duration,
    ) -> NestedLayout {
        let static_child_size = self.static_child_size(size, children, pts);
        let (scale, crop) = match self.overflow {
            Overflow::Visible => (1.0, None),
            Overflow::Hidden => (
                1.0,
                Some(Crop {
                    top: 0.0,
                    left: 0.0,
                    width: size.width,
                    height: size.height,
                }),
            ),
            Overflow::Fit => (
                self.scale_factor_for_overflow_fit(size, children, pts),
                None,
            ),
        };

        // offset along x or y direction (depends on self.direction) where next
        // child component should be placed
        let mut static_offset = 0.0;

        let children: Vec<_> = children
            .iter_mut()
            .map(|child| {
                let position = match child {
                    StatefulComponent::Layout(layout) => layout.position(pts),
                    ref non_layout_component => Position::Static {
                        width: non_layout_component.width(pts),
                        height: non_layout_component.height(pts),
                    },
                };
                match position {
                    Position::Static { width, height } => {
                        let (layout, updated_static_offset) = self.layout_static_child(
                            child,
                            StaticChildLayoutOpts {
                                width,
                                height,
                                static_offset,
                                static_child_size,
                                parent_size: size,
                            },
                            pts,
                        );

                        static_offset = updated_static_offset;
                        layout
                    }
                    Position::Absolute(position) => {
                        StatefulLayoutComponent::layout_absolute_position_child(
                            child, position, size, pts,
                        )
                    }
                }
            })
            .collect();
        NestedLayout {
            top: 0.0,
            left: 0.0,
            width: size.width,
            height: size.height,
            rotation_degrees: 0.0,
            scale_x: scale,
            scale_y: scale,
            crop,
            content: LayoutContent::Color(self.background_color),
            child_nodes_count: children.iter().map(|l| l.child_nodes_count).sum(),
            children,
        }
    }

    fn layout_static_child(
        &self,
        child: &mut StatefulComponent,
        opts: StaticChildLayoutOpts,
        pts: Duration,
    ) -> (NestedLayout, f32) {
        let mut static_offset = opts.static_offset;
        let (top, left, width, height) = match self.direction {
            ViewChildrenDirection::Row => {
                let width = opts.width.unwrap_or(opts.static_child_size);
                let height = opts.height.unwrap_or(opts.parent_size.height);
                let top = 0.0;
                let left = static_offset;
                static_offset += width;
                (top as f32, left, width, height)
            }
            ViewChildrenDirection::Column => {
                let height = opts.height.unwrap_or(opts.static_child_size);
                let width = opts.width.unwrap_or(opts.parent_size.width);
                let top = static_offset;
                let left = 0.0;
                static_offset += height;
                (top, left as f32, width, height)
            }
        };
        let layout = match child {
            StatefulComponent::Layout(layout_component) => {
                let children_layouts = layout_component.layout(Size { width, height }, pts);
                NestedLayout {
                    top,
                    left,
                    width,
                    height,
                    rotation_degrees: 0.0,
                    scale_x: 1.0,
                    scale_y: 1.0,
                    crop: None,
                    content: LayoutContent::None,
                    child_nodes_count: children_layouts.child_nodes_count,
                    children: vec![children_layouts],
                }
            }
            _ => NestedLayout {
                top,
                left,
                width,
                height,
                rotation_degrees: 0.0,
                scale_x: 1.0,
                scale_y: 1.0,
                crop: None,
                content: StatefulLayoutComponent::layout_content(child, 0),
                child_nodes_count: 1,
                children: vec![],
            },
        };
        (layout, static_offset)
    }

    /// Calculate a size of a static child component that does not have it explicitly defined.
    /// Returned value represents width if the direction is `ViewChildrenDirection::Row` or
    /// height if the direction is `ViewChildrenDirection::Column`.
    fn static_child_size(&self, size: Size, children: &[StatefulComponent], pts: Duration) -> f32 {
        let max_size = match self.direction {
            super::ViewChildrenDirection::Row => size.width,
            super::ViewChildrenDirection::Column => size.height,
        };

        let children_with_unknown_size_count = Self::static_children_iter(children, pts)
            .filter(|child| match self.direction {
                ViewChildrenDirection::Row => child.width(pts).is_none(),
                ViewChildrenDirection::Column => child.height(pts).is_none(),
            })
            .count();
        let static_children_sum = self.sum_static_children_sizes(children, pts);

        if children_with_unknown_size_count == 0 {
            return 0.0; // if there is no dynamically sized children then this value does not matter
        }
        f32::max(
            0.0,
            (max_size - static_children_sum) / children_with_unknown_size_count as f32,
        )
    }

    fn scale_factor_for_overflow_fit(
        &self,
        size: Size,
        children: &[StatefulComponent],
        pts: Duration,
    ) -> f32 {
        let sum_size = self
            .sum_static_children_sizes(children, pts)
            .max(0.000000001); // avoid division by 0
        let (max_size, max_alternative_size) = match self.direction {
            super::ViewChildrenDirection::Row => (size.width, size.height),
            super::ViewChildrenDirection::Column => (size.height, size.width),
        };
        let max_alternative_size_for_child = Self::static_children_iter(children, pts)
            .map(|child| match self.direction {
                ViewChildrenDirection::Row => child.height(pts).unwrap_or(0.0),
                ViewChildrenDirection::Column => child.width(pts).unwrap_or(0.0),
            })
            .max_by(|a, b| f32::partial_cmp(a, b).unwrap()) // will panic if comparing NaN
            .unwrap_or(0.0)
            .max(0.000000001); // avoid division by 0

        f32::min(
            1.0,
            f32::min(
                max_size / sum_size,
                max_alternative_size / max_alternative_size_for_child,
            ),
        )
    }

    fn sum_static_children_sizes(&self, children: &[StatefulComponent], pts: Duration) -> f32 {
        let size_accessor = match self.direction {
            super::ViewChildrenDirection::Row => StatefulComponent::width,
            super::ViewChildrenDirection::Column => StatefulComponent::height,
        };

        Self::static_children_iter(children, pts)
            .map(|component| size_accessor(component, pts).unwrap_or(0.0))
            .sum()
    }

    fn static_children_iter(
        children: &[StatefulComponent],
        pts: Duration,
    ) -> impl Iterator<Item = &StatefulComponent> {
        children.iter().filter(move |child| match child {
            StatefulComponent::Layout(layout) => match layout.position(pts) {
                super::Position::Static { .. } => true,
                super::Position::Absolute(_) => false,
            },
            _ => true,
        })
    }
}
