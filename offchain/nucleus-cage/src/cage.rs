use crate::nucleus::{Gluon, Nucleus};
use dashmap::DashMap;
use futures::{
    prelude::*,
    task::{Context, Poll},
};
use std::collections::HashMap;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct CageParameters {}

pub struct NucleusCage {
    sender: Sender<Gluon>,
    tunnel: Vec<Gluon>,
    /// forward
    nuclei: DashMap<String, (u64,)>,
}

impl NucleusCage {}

pub fn start_nucleus_cage(
    params: CageParameters,
) -> anyhow::Result<impl Future<Output = ()> + Send> {
    Ok(MonitorHostStorageTask {})
}

pub struct MonitorHostStorageTask {}

impl Future for MonitorHostStorageTask {
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 1. recover from storage
        // 2. monitor storage change event
        Poll::Ready(())
    }
}
