use std::{collections::HashMap, sync::Arc, time::Duration};

use compositor_common::{scene::NodeId, Frame};

#[derive(Debug)]
pub struct FrameSet<Id>
where
    Id: From<NodeId>,
{
    pub frames: HashMap<Id, Arc<Frame>>,
    pub pts: Duration,
}

impl<Id> FrameSet<Id>
where
    Id: From<NodeId>,
{
    pub fn new(pts: Duration) -> Self {
        FrameSet {
            frames: HashMap::new(),
            pts,
        }
    }
}