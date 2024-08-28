use std::{cmp::Ordering, collections::BinaryHeap, rc::Rc};

use chrono::{DateTime, Utc};
#[derive(Debug, Clone)]
pub enum CallerType {
    Post,
    Get,
}
#[derive(Debug, Clone)]
pub struct CallerInfo {
    pub func: String,
    pub thread_id: u64,
    pub caller_type: CallerType,
}
pub struct TimerEntry {
    pub caller_info: Vec<CallerInfo>,
    pub timestamp: DateTime<Utc>,
    pub func_name: String,
    pub func_params: Vec<u8>,
}
pub(crate) struct TimerQueue {
    heap: BinaryHeap<TimerEntry>,
}

impl PartialEq for TimerEntry {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
            && self.func_name == other.func_name
            && self.func_params == other.func_params
    }
}

impl PartialOrd for TimerEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TimerEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}
impl Eq for TimerEntry {}

impl TimerQueue {
    pub fn new() -> Self {
        TimerQueue {
            heap: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, entry: TimerEntry) {
        self.heap.push(entry);
    }

    pub fn pop(&mut self) -> Option<TimerEntry> {
        self.heap.pop()
    }
}
