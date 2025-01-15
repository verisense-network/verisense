use codec::Encode;
use std::sync::Arc;
use vrs_nucleus_executor::{Event, Gluon, NucleusState, NucleusTunnel};
use vrs_primitives::NucleusId;

pub(crate) struct NucleusCage {
    pub(crate) nucleus_id: NucleusId,
    pub(crate) tunnel: NucleusTunnel,
    pub(crate) pending_requests: Vec<Gluon>,
    pub(crate) event_id: u64,
    pub(crate) state: Arc<NucleusState>,
}

impl NucleusCage {
    pub(crate) fn validate_token(&self) -> bool {
        true
    }

    pub(crate) fn pre_commit(&self, id: u64, msg: &[u8]) -> anyhow::Result<()> {
        // let handle = self.db.cf_handle("seq").unwrap();
        // self.db.put_cf(handle, &id.to_be_bytes(), msg)?;
        Ok(())
    }

    pub(crate) fn drain(&mut self, imports: Vec<Event>) {
        // TODO handle imports first
        // for `TimerRegister` and `HttpRequest`, we need to check its id
        let pipe = self.pending_requests.drain(..).collect::<Vec<_>>();
        for gluon in pipe.into_iter() {
            self.event_id += 1;
            let event = Event::from(&gluon);
            if let Err(e) = self.pre_commit(self.event_id, &event.encode()) {
                log::error!(
                    "couldn't save event {} of nucleus {}: {:?}",
                    self.event_id,
                    self.nucleus_id,
                    e
                );
                // TODO only reply request from rpc
                // if let Some(reply_to) = gluon.2 {
                //     let _ = reply_to.send(Err((-42000, "Event persistence failed.".to_string())));
                // }
            } else {
                let _ = self.tunnel.send((self.event_id, gluon));
            }
        }
    }

    pub(crate) fn forward(&mut self, gluon: Gluon) {
        if matches!(gluon, Gluon::GetRequest { .. }) {
            let _ = self.tunnel.send((0, gluon));
        } else {
            self.pending_requests.push(gluon);
        }
    }
}
