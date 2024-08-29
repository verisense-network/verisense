use core::fmt;
use std::hash::{Hash, Hasher};
use std::{cmp::Ordering, collections::BinaryHeap, hash::DefaultHasher, rc::Rc};

use chrono::{DateTime, Utc};
#[derive(Debug, Clone, Hash)]
pub enum CallerType {
    Post,
    Get,
    Entry,
}
#[derive(Clone, Hash)]
pub struct CallerInfo {
    pub func: String,
    pub params: Vec<u8>,
    pub thread_id: u64,
    pub caller_type: CallerType,
}
impl CallerInfo {
    pub fn hash_u64(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}
impl fmt::Debug for CallerInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CallerInfo")
            .field("func", &self.func)
            .field("thread_id", &self.thread_id)
            .field("caller_type", &self.caller_type)
            .field("hash", &format!("{:016x}", self.hash_u64()))
            .finish()
    }
}

impl fmt::Display for CallerInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}\nHash: {:016x}", self, self.hash_u64())
    }
}

#[derive(Clone, Hash)]
pub struct TimerEntry {
    pub caller_infos: Vec<CallerInfo>,
    pub timestamp: DateTime<Utc>,
    pub func_name: String,
    pub func_params: Vec<u8>,
}
pub(crate) struct TimerQueue {
    heap: BinaryHeap<TimerEntry>,
}
impl TimerEntry {
    pub fn hash_u64(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl fmt::Debug for TimerEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TimerEntry")
            .field("caller_infos", &self.caller_infos)
            .field("timestamp", &self.timestamp)
            .field("func_name", &self.func_name)
            .field("func_params", &self.func_params)
            .field("hash", &format!("{:016x}", self.hash_u64()))
            .finish()
    }
}

impl fmt::Display for TimerEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}\nHash: {:016x}", self, self.hash_u64())
    }
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
        other.timestamp.cmp(&self.timestamp)
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
