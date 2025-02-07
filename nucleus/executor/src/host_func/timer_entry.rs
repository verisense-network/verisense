use chrono::{DateTime, Utc};
use core::fmt;
use std::cmp::Reverse;
use std::hash::{Hash, Hasher};
use std::{cmp::Ordering, collections::BinaryHeap, hash::DefaultHasher};
use vrs_primitives::NucleusId;

#[derive(Clone, Hash)]
pub struct TimerEntry {
    pub nucleus_id: NucleusId,
    pub timestamp: DateTime<Utc>,
    pub func_name: String,
    pub triggered_time: Option<DateTime<Utc>>,
    pub func_params: Vec<u8>,
}

pub(crate) struct TimerQueue {
    heap: BinaryHeap<Reverse<TimerEntry>>,
}

impl TimerEntry {
    pub fn new(
        nucleus_id: NucleusId,
        timestamp: DateTime<Utc>,
        func_name: String,
        func_params: Vec<u8>,
    ) -> Self {
        TimerEntry {
            nucleus_id,
            timestamp,
            func_name,
            triggered_time: None,
            func_params,
        }
    }

    pub fn hash_u64(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl fmt::Debug for TimerEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TimerEntry")
            .field("timestamp", &self.timestamp)
            .field("triggered_time", &self.triggered_time)
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
        self.heap.push(Reverse(entry));
    }

    pub fn pop(&mut self) -> Option<TimerEntry> {
        self.heap.pop().map(|Reverse(entry)| entry)
    }
    pub fn _peek(&self) -> Option<&TimerEntry> {
        self.heap.peek().map(|Reverse(entry)| entry)
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use chrono::Utc;
//     use std::time::Duration;

//     #[test]
//     fn test_timer_queue() {
//         let mut timer_queue = TimerQueue::new();
//         let entry = TimerEntry::new(
//             NucleusId::new([0; 32]),
//             vec![CallerInfo {
//                 func: "test".to_string(),
//                 params: vec![1, 2, 3],
//                 thread_id: 1,
//                 caller_type: CallerType::Entry,
//             }],
//             Utc::now() + Duration::from_secs(1),
//             "test".to_string(),
//             vec![1, 2, 3],
//         );
//         timer_queue.push(entry.clone());
//         let entry = TimerEntry::new(
//             NucleusId::new([0; 32]),
//             vec![CallerInfo {
//                 func: "test".to_string(),
//                 params: vec![3, 2, 1],
//                 thread_id: 1,
//                 caller_type: CallerType::Entry,
//             }],
//             Utc::now() + Duration::from_secs(2),
//             "test".to_string(),
//             vec![3, 2, 1],
//         );
//         timer_queue.push(entry.clone());
//         let entry = TimerEntry::new(
//             NucleusId::new([0; 32]),
//             vec![CallerInfo {
//                 func: "test".to_string(),
//                 params: vec![3, 2, 1],
//                 thread_id: 1,
//                 caller_type: CallerType::Entry,
//             }],
//             Utc::now() + Duration::from_secs(3),
//             "test".to_string(),
//             vec![3, 2, 1],
//         );
//         timer_queue.push(entry.clone());
//         assert_eq!(timer_queue.pop().unwrap().func_params, vec![1, 2, 3]);
//     }
// }
