use codec::{Decode, Encode};
use vrs_primitives::NucleusId;

#[derive(Clone, Debug, Encode, Decode)]
pub struct FetchEventsRequest {}

#[derive(Clone, Debug, Encode, Decode)]
pub struct FetchEventsResponse {}

#[derive(Clone, Debug, Encode, Decode)]
pub struct MonadDeliverRequest {}

#[derive(Clone, Debug, Encode, Decode)]
pub struct MonadDeliverResponse {}

#[derive(Clone, Debug, Encode, Decode)]
pub struct GluonForwardRequest {
    pub nucleus_id: NucleusId,
    pub payload: Vec<u8>,
}

#[derive(Clone, Debug, Encode, Decode)]
pub struct GluonForwardResponse {}
