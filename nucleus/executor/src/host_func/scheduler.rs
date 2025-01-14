use crate::{TimerEntry, TimerQueue};
use chrono::Utc;
use std::pin::Pin;
use tokio::sync::mpsc::Receiver;
use tokio::time::{self, Instant, Sleep};

// pub struct WrappedScheduler {
//     sender: Sender<TimerEntry>,
// }

// pub struct WrappedSchedulerSync {
//     sender: SyncSender<TimerEntry>,
// }

// impl WrappedScheduler {
//     pub fn new() -> (Self, Receiver<(DateTime<Utc>, TimerEntry)>) {
//         let (sender_cage, receiver_cage) = tokio::sync::mpsc::channel(100);
//         let (sender_nucleus, mut receiver_nucleus) = tokio::sync::mpsc::channel(100);
//         tokio::spawn(async move {
//             let mut timer = Box::pin(time::sleep(time::Duration::from_secs(0)));
//             let mut timer_queue = TimerQueue::new();
//             loop {
//                 tokio::select! {
//                     Some(entry) = receiver_nucleus.recv() => {
//                         timer_queue.push(entry);
//                         if let Some(entry) = timer_queue.peek() {
//                             let now = Utc::now();
//                             if entry.timestamp > now {
//                                 let duration = (entry.timestamp - now).to_std().unwrap_or_default();
//                                 let deadline = Instant::now() + duration;
//                                 timer.as_mut().reset(deadline);
//                             } else {
//                                 timer.as_mut().reset(Instant::now());
//                             }
//                         }
//                     }
//                     _ = &mut timer => {
//                         if !timer_queue.is_empty() {
//                             let now = Utc::now();
//                             while let Some(entry) = timer_queue.peek() {
//                                 if entry.timestamp <= now {
//                                     if let Some(entry) = timer_queue.pop() {
//                                         sender_cage.send((now,entry)).await.unwrap();
//                                     }
//                                 } else {
//                                     break;
//                                 }
//                             }
//                             if let Some(entry) = timer_queue.peek() {
//                                 let now = Utc::now();
//                                 if entry.timestamp > now {
//                                     let duration = (entry.timestamp - now).to_std().unwrap_or_default();
//                                     let deadline = Instant::now() + duration;
//                                     timer.as_mut().reset(deadline);
//                                 } else {
//                                     timer.as_mut().reset(Instant::now());
//                                 }
//                             }
//                         } else {
//                             timer.as_mut().reset(Instant::now() + time::Duration::from_secs(3600));
//                         }
//                     }
//                 }
//             }
//         });
//         (
//             WrappedScheduler {
//                 sender: sender_nucleus,
//             },
//             receiver_cage,
//         )
//     }

//     pub async fn push(
//         &self,
//         entry: TimerEntry,
//     ) -> Result<(), tokio::sync::mpsc::error::SendError<TimerEntry>> {
//         self.sender.send(entry).await
//     }
// }

// impl WrappedSchedulerSync {
//     pub fn new() -> (Self, mpsc::Receiver<(DateTime<Utc>, TimerEntry)>) {
//         let (sender_nucleus, receiver_nucleus) = mpsc::channel();
//         let (sender_cage, receiver_cage) = mpsc::channel();

//         thread::spawn(move || {
//             let mut timer_queue = TimerQueue::new();
//             loop {
//                 let next_timeout = if let Some(entry) = timer_queue.peek() {
//                     let now = Utc::now();
//                     if entry.timestamp > now {
//                         (entry.timestamp - now).to_std().unwrap_or_default()
//                     } else {
//                         Duration::from_secs(0)
//                     }
//                 } else {
//                     Duration::from_secs(3600)
//                 };

//                 let receiver_timeout = receiver_nucleus.recv_timeout(next_timeout);

//                 match receiver_timeout {
//                     Ok(entry) => {
//                         timer_queue.push(entry);
//                     }
//                     Err(mpsc::RecvTimeoutError::Timeout) => {
//                         let now = Utc::now();
//                         while let Some(entry) = timer_queue.peek() {
//                             if entry.timestamp <= now {
//                                 if let Some(entry) = timer_queue.pop() {
//                                     sender_cage.send((now, entry)).unwrap();
//                                 }
//                             } else {
//                                 break;
//                             }
//                         }
//                     }
//                     Err(mpsc::RecvTimeoutError::Disconnected) => {
//                         break;
//                     }
//                 }
//             }
//         });

//         (
//             WrappedSchedulerSync {
//                 sender: sender_nucleus,
//             },
//             receiver_cage,
//         )
//     }

//     pub fn push(&self, entry: TimerEntry) -> Result<(), std::sync::mpsc::SendError<TimerEntry>> {
//         self.sender.send(entry)
//     }

//     pub fn push_thread(&self, entry: TimerEntry) {
//         let sender = self.sender.clone();
//         thread::spawn(move || {
//             sender.send(entry).unwrap();
//         });
//     }
// }

// pub struct Scheduler {
//     receiver: Receiver<TimerEntry>,
//     timer_queue: TimerQueue,
// }

// impl Scheduler {
//     pub fn new(receiver: Receiver<TimerEntry>) -> Self {
//         Scheduler {
//             receiver,
//             timer_queue: TimerQueue::new(),
//         }
//     }

//     pub async fn start<F>(mut self, callback: F)
//     where
//         F: Fn(TimerEntry) + Send + 'static,
//     {
//         let mut timer = Box::pin(time::sleep(time::Duration::from_secs(0)));

//         loop {
//             tokio::select! {
//                 Some(entry) = self.receiver.recv() => {
//                     self.timer_queue.push(entry);
//                     self.update_timer(&mut timer);
//                 }
//                 _ = &mut timer => {
//                     if !self.timer_queue.is_empty() {
//                         self.process_queue(&callback);
//                         self.update_timer(&mut timer);
//                     } else {
//                         timer.as_mut().reset(Instant::now() + time::Duration::from_secs(3600));
//                     }
//                 }
//             }
//         }
//     }

//     fn update_timer(&mut self, timer: &mut Pin<Box<Sleep>>) {
//         if let Some(entry) = self.timer_queue.peek() {
//             let now = Utc::now();
//             if entry.timestamp > now {
//                 let duration = (entry.timestamp - now).to_std().unwrap_or_default();
//                 let deadline = Instant::now() + duration;
//                 timer.as_mut().reset(deadline);
//             } else {
//                 timer.as_mut().reset(Instant::now());
//             }
//         }
//     }

//     fn process_queue<F>(&mut self, callback: &F)
//     where
//         F: Fn(TimerEntry) + Send,
//     {
//         let now = Utc::now();
//         while let Some(entry) = self.timer_queue.peek() {
//             if entry.timestamp <= now {
//                 if let Some(entry) = self.timer_queue.pop() {
//                     callback(entry);
//                 }
//             } else {
//                 break;
//             }
//         }
//     }
// }
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{host_func::timer::SchedulerAsync, CallerInfo, CallerType};
    use chrono::{Duration, Utc};
    use futures::StreamExt;
    use std::{sync::Arc, time::Duration as StdDuration};
    use tokio::sync::mpsc;
    use vrs_primitives::NucleusId;

    #[tokio::test]
    async fn test_scheduler() {
        // Create channels
        let (sender, receiver) = mpsc::channel(100);
        let (trigger_sender, mut trigger_receiver) = mpsc::channel(100);

        // Create and start the scheduler
        let scheduler = Scheduler::new(receiver);
        tokio::spawn(async move {
            scheduler
                .start(move |entry| {
                    let now = Utc::now();
                    let _ = trigger_sender.try_send((now, entry));
                })
                .await;
        });

        // Create test TimerEntries
        let start_time = Utc::now();
        let entries = vec![
            create_timer_entry(3),
            create_timer_entry(2),
            create_timer_entry(1),
        ];

        // Send entries to the scheduler
        for entry in entries.clone() {
            sender.send(entry).await.unwrap();
        }

        // Collect triggered entries
        let mut triggered_entries = Vec::new();
        tokio::select! {
            _ = tokio::time::sleep(StdDuration::from_secs(4)) => {},
            _ = async {
                while let Some(entry) = trigger_receiver.recv().await {
                    triggered_entries.push(entry);
                    if triggered_entries.len() == 3 {
                        break;
                    }
                }
            } => {}
        }

        // Clean up
        drop(sender);

        // Verify results
        assert_eq!(triggered_entries.len(), 3, "Not all entries were triggered");

        // Sort entries by timestamp for comparison
        let mut sorted_entries = entries.clone();
        sorted_entries.sort_by_key(|e| e.timestamp);

        for (i, (trigger_time, entry)) in triggered_entries.iter().enumerate() {
            assert_eq!(
                entry.timestamp, sorted_entries[i].timestamp,
                "Entry {} was not triggered in the correct order",
                i
            );

            let expected_trigger_time = start_time + Duration::seconds(i as i64 + 1);
            let time_difference = (*trigger_time - expected_trigger_time)
                .num_milliseconds()
                .abs();
            assert!(
                time_difference < 100,
                "Entry {} was not triggered at the expected time. Difference: {} ms",
                i,
                time_difference
            );
        }
    }

    fn create_timer_entry(seconds: i64) -> TimerEntry {
        TimerEntry {
            nucleus_id: NucleusId::new([0; 32]),
            caller_infos: vec![CallerInfo {
                func: "test".to_string(),
                params: vec![3, 2, 1],
                thread_id: 1,
                caller_type: CallerType::Entry,
            }],
            triggered_time: None,
            timestamp: Utc::now() + Duration::seconds(seconds),
            func_name: "test".to_string(),
            func_params: vec![3, 2, 1],
        }
    }

    #[tokio::test]
    async fn test_wrapped_scheduler() {
        let (scheduler, mut receiver_cage) = WrappedScheduler::new();
        let scheduler = Arc::new(scheduler);

        // Create test TimerEntries
        let start_time = Utc::now();
        let entries = vec![
            create_timer_entry(3),
            create_timer_entry(2),
            create_timer_entry(1),
        ];
        scheduler.push(entries[0].clone()).await.unwrap();
        scheduler.push(entries[1].clone()).await.unwrap();
        scheduler.push(entries[2].clone()).await.unwrap();
        // Send entries to the scheduler
        for entry in entries.clone() {}

        // Collect triggered entries
        let mut triggered_entries = Vec::new();
        tokio::select! {
            _ = tokio::time::sleep(StdDuration::from_secs(4)) => {},
            _ = async {
                while let Some(entry) = receiver_cage.recv().await {
                    triggered_entries.push(entry);
                    if triggered_entries.len() == 3 {
                        break;
                    }
                }
            } => {}
        }

        // Verify results
        assert_eq!(triggered_entries.len(), 3, "Not all entries were triggered");

        // // Sort entries by timestamp for comparison
        let mut sorted_entries = entries.clone();
        sorted_entries.sort_by_key(|e| e.timestamp);

        for (i, (trigger_time, entry)) in triggered_entries.iter().enumerate() {
            assert_eq!(
                entry.timestamp, sorted_entries[i].timestamp,
                "Entry {} was not triggered in the correct order",
                i
            );

            let expected_trigger_time = start_time + Duration::seconds(i as i64 + 1);
            let time_difference = (*trigger_time - expected_trigger_time)
                .num_milliseconds()
                .abs();
            assert!(
                time_difference < 100,
                "Entry {} was not triggered at the expected time. Difference: {} ms",
                i,
                time_difference
            );
        }
    }
    #[tokio::test]
    async fn test_scheduler_async() {
        let mut scheduler = SchedulerAsync::new();

        // Create test TimerEntries
        let start_time = Utc::now();
        let entries = vec![
            create_timer_entry(6),
            create_timer_entry(5),
            create_timer_entry(4),
            create_timer_entry(3),
            create_timer_entry(2),
            create_timer_entry(1),
        ];
        scheduler.push(entries[0].clone());
        scheduler.push(entries[1].clone());
        scheduler.push(entries[2].clone());
        scheduler.push(entries[3].clone());
        scheduler.push(entries[4].clone());
        scheduler.push(entries[5].clone());
        let mut triggered_entries = Vec::new();
        triggered_entries.push(scheduler.next().await.unwrap());
        triggered_entries.push(scheduler.pop().await.unwrap());
        triggered_entries.push(scheduler.next().await.unwrap());
        triggered_entries.push(scheduler.pop().await.unwrap());
        triggered_entries.push(scheduler.next().await.unwrap());
        triggered_entries.push(scheduler.pop().await.unwrap());

        // // Sort entries by timestamp for comparison
        let mut sorted_entries = entries.clone();
        sorted_entries.sort_by_key(|e| e.timestamp);

        for (i, entry) in triggered_entries.iter().enumerate() {
            assert_eq!(
                entry.timestamp, sorted_entries[i].timestamp,
                "Entry {} was not triggered in the correct order",
                i
            );

            let expected_trigger_time = start_time + Duration::seconds(i as i64 + 1);
            let time_difference = (entry.triggered_time.unwrap() - expected_trigger_time)
                .num_milliseconds()
                .abs();
            assert!(
                time_difference < 100,
                "Entry {} was not triggered at the expected time. Difference: {} ms",
                i,
                time_difference
            );
        }
    }
    #[tokio::test]
    async fn test_wrapped_scheduler_sync() {
        let (scheduler, mut receiver_cage) = WrappedSchedulerSync::new();
        let scheduler = Arc::new(scheduler);

        // Create test TimerEntries
        let start_time = Utc::now();
        let entries = vec![
            create_timer_entry(3),
            create_timer_entry(2),
            create_timer_entry(1),
        ];
        scheduler.push(entries[0].clone()).unwrap();
        scheduler.push(entries[1].clone()).unwrap();
        scheduler.push(entries[2].clone()).unwrap();
        // Send entries to the scheduler
        for entry in entries.clone() {}

        // Collect triggered entries
        let mut triggered_entries = Vec::new();
        tokio::select! {
            _ = tokio::time::sleep(StdDuration::from_secs(4)) => {},
            _ = async {
                while let Ok(entry) = receiver_cage.recv() {
                    triggered_entries.push(entry);
                    if triggered_entries.len() == 3 {
                        break;
                    }
                }
            } => {}
        }

        // Verify results
        assert_eq!(triggered_entries.len(), 3, "Not all entries were triggered");

        // // Sort entries by timestamp for comparison
        let mut sorted_entries = entries.clone();
        sorted_entries.sort_by_key(|e| e.timestamp);

        for (i, (trigger_time, entry)) in triggered_entries.iter().enumerate() {
            assert_eq!(
                entry.timestamp, sorted_entries[i].timestamp,
                "Entry {} was not triggered in the correct order",
                i
            );

            let expected_trigger_time = start_time + Duration::seconds(i as i64 + 1);
            let time_difference = (*trigger_time - expected_trigger_time)
                .num_milliseconds()
                .abs();
            assert!(
                time_difference < 100,
                "Entry {} was not triggered at the expected time. Difference: {} ms",
                i,
                time_difference
            );
        }
    }
}
