use crate::cage::MonadringVerifyResult::{AllGood, Failed};
use anyhow::anyhow;
use codec::{Decode, Encode};
use sp_core::sr25519::Signature;
use std::sync::Arc;
use vrs_nucleus_executor::{state::B256, Event, Gluon, NucleusState, NucleusTunnel};
use vrs_primitives::{AccountId, NucleusId};

pub(crate) struct NucleusCage {
    pub(crate) nucleus_id: NucleusId,
    pub(crate) tunnel: NucleusTunnel,
    pub(crate) pending_requests: Vec<Gluon>,
    pub(crate) event_id: u64,
    pub(crate) state: Arc<NucleusState>,
}

pub enum MonadringVerifyResult {
    AllGood,
    Failed,
}

impl NucleusCage {
    pub(crate) fn validate_token(
        &self,
        self_account: &AccountId,
        token: &MonadringToken,
    ) -> MonadringVerifyResult {
        let event_id = self.event_id;
        if token.ring.is_empty() {
            return AllGood;
        }
        let item = token.ring.first().cloned().unwrap();
        let mut items = token.ring.clone();
        if item.source != *self_account {
            items.remove(0);
        }
        for item in items {
            if event_id + item.events.len() as u64 != item.last_event_id {
                return Failed;
            }
        }
        AllGood
    }

    pub(crate) fn pre_commit(&self, id: u64, msg: &[u8]) -> anyhow::Result<()> {
        self.state
            .put_user_data(&id.to_be_bytes(), msg)
            .map_err(|e| anyhow!(e))
    }

    pub fn execute_outer_events(&mut self, imports: Vec<Event>) -> Vec<Event> {
        // for `TimerRegister` and `HttpRequest`, we need to check its id
        for event in imports.clone() {
            self.event_id += 1;
            if let Err(e) = self.pre_commit(self.event_id, &event.encode()) {
                log::error!(
                    "couldn't save event {} of nucleus {}: {:?}",
                    self.event_id,
                    self.nucleus_id,
                    e
                );
            } else {
            }
        }
        imports
    }

    pub(crate) fn drain(&mut self, imports: Vec<Event>) -> Vec<Event> {
        let imports = self.execute_outer_events(imports);
        let mut new_events = imports;
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
                new_events.push(event);
            }
        }
        new_events
    }

    pub(crate) fn forward(&mut self, gluon: Gluon) {
        if matches!(gluon, Gluon::GetRequest { .. }) {
            let _ = self.tunnel.send((0, gluon));
        } else {
            self.pending_requests.push(gluon);
        }
    }
}

#[derive(Debug, Encode, Decode)]
pub struct QueryEventsResult {
    pub events: Vec<Event>,
}

#[derive(Debug, Decode, Encode)]
pub struct MonadringToken {
    pub nucleus_id: NucleusId,
    pub ring: Vec<MonadringTokenItem>,
}

impl MonadringToken {
    pub fn combine_events(&self, account_id: &AccountId) -> Vec<Event> {
        let mut v = vec![];
        for i in self.ring.iter() {
            if i.source == *account_id {
                continue;
            }
            v.append(&mut i.events.clone());
        }
        v
    }
}

#[derive(Debug, Clone, Decode, Encode)]
pub struct MonadringTokenItem {
    pub events: Vec<Event>,
    pub nucleus_state_root: B256,
    pub last_event_id: u64,
    pub source: AccountId,
    pub signature: Signature,
}
