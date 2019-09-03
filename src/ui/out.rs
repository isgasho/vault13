use super::*;

use crate::game::object;
use crate::graphics::EPoint;

/// Output event for signaling widget-specific events to callee.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OutEvent {
    pub source: Handle,
    pub data: OutEventData,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OutEventData {
}