use crate::{
    host_func::{TimerEntry, TimerQueue},
    mem,
    runtime::{ComponentProvider, ContextAware, Runtime},
};
use chrono::Utc;
use std::{pin::Pin, sync::Arc, task::Poll};
use tokio::sync::{
    mpsc::{UnboundedReceiver, UnboundedSender},
    Mutex,
};
use wasmtime::{Caller, Engine, FuncType, Val, ValType};

impl ComponentProvider<PendingTimerQueue> for Runtime {
    fn get_component(&self) -> std::sync::Arc<PendingTimerQueue> {
        self.timer_register.clone()
    }
}

pub(crate) fn register_timer_signature(engine: &Engine) -> FuncType {
    FuncType::new(
        &engine,
        [
            ValType::I32,
            ValType::I32,
            ValType::I32,
            ValType::I32,
            ValType::I32,
        ],
        [ValType::I32],
    )
}

pub(crate) fn now_timestamp_signature(engine: &Engine) -> FuncType {
    FuncType::new(&engine, [], [ValType::I32])
}

pub(crate) fn now_timestamp(
    mut _caller: Caller<'_, Runtime>,
    _params: &[Val],
    results: &mut [Val],
) -> anyhow::Result<()> {
    results[0] = Val::I32(Utc::now().timestamp() as i32);
    Ok(())
}

pub(crate) fn register_timer<R>(
    mut caller: Caller<'_, R>,
    params: &[Val],
    results: &mut [Val],
) -> anyhow::Result<()>
where
    R: ContextAware + ComponentProvider<PendingTimerQueue>,
{
    if caller.data().read_only() {
        results[0] = Val::I32(1);
        return Ok(());
    }
    if let [Val::I32(delay), Val::I32(func_ptr), Val::I32(func_len), Val::I32(params_ptr), Val::I32(params_len)] =
        params
    {
        results[0] = Val::I32(3);
        let func_params = mem::read_bytes_from_memory(&mut caller, *params_ptr, *params_len)?;
        let func_name = crate::mem::read_bytes_from_memory(&mut caller, *func_ptr, *func_len)?;
        let timestamp = Utc::now() + std::time::Duration::from_secs(*delay as u64);

        let entry = TimerEntry {
            nucleus_id: caller.data().get_nucleus_id(),
            timestamp,
            func_name: String::from_utf8(func_name)?,
            triggered_time: None,
            func_params,
        };
        let timer = caller.data_mut().get_component();
        timer.push(entry);
        results[0] = Val::I32(0);
    } else {
        results[0] = Val::I32(2);
    }
    Ok(())
}

pub struct PendingTimerQueue {
    pending_timer: Arc<std::sync::Mutex<TimerQueue>>,
}

impl PendingTimerQueue {
    pub fn new() -> Self {
        PendingTimerQueue {
            pending_timer: Arc::new(std::sync::Mutex::new(TimerQueue::new())),
        }
    }

    pub fn push(&self, entry: TimerEntry) {
        let mut pending_timer = self.pending_timer.lock().unwrap();
        pending_timer.push(entry);
    }

    pub fn pop(&self) -> Option<TimerEntry> {
        let mut pending_timer = self.pending_timer.lock().unwrap();
        pending_timer.pop()
    }

    pub fn is_empty(&self) -> bool {
        let pending_timer = self.pending_timer.lock().unwrap();
        pending_timer.is_empty()
    }
}

impl ComponentProvider<SchedulerAsync> for Runtime {
    fn get_component(&self) -> std::sync::Arc<SchedulerAsync> {
        self.timer_scheduler.clone()
    }
}

#[derive(Clone, Debug)]
pub struct SchedulerAsync {
    tx: UnboundedSender<TimerEntry>,
    rx: Arc<Mutex<UnboundedReceiver<TimerEntry>>>,
}

impl SchedulerAsync {
    pub fn new() -> Self {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        SchedulerAsync {
            tx,
            rx: Arc::new(Mutex::new(rx)),
        }
    }

    pub async fn pop(&self) -> Option<TimerEntry> {
        let mut rx = self.rx.lock().await;
        rx.recv().await
    }

    pub fn push(&self, entry: TimerEntry) {
        let tx = self.tx.clone();
        std::thread::spawn(move || {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                let now = Utc::now();
                if entry.timestamp > now {
                    let delay = (entry.timestamp - now)
                        .to_std()
                        .unwrap_or_else(|_| tokio::time::Duration::from_secs(0));
                    tokio::time::sleep(delay).await;
                }
                let mut entry = entry;
                entry.triggered_time = Some(Utc::now());
                if let Err(e) = tx.send(entry) {
                    eprintln!("Failed to send entry: {}", e);
                }
            });
        });
    }
}

impl futures::Stream for SchedulerAsync {
    type Item = TimerEntry;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let rx = self.rx.try_lock();
        match rx {
            Ok(mut rx) => rx.poll_recv(cx),
            Err(_) => {
                return Poll::Pending;
            }
        }
    }
}
