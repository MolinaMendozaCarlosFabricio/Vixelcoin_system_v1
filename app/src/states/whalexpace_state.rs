use sails_rs::prelude::*;

#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct DataPlayerWhaleXPace {
    pub address: ActorId,
    pub username: String,
    pub score: u32
}

#[derive(Default)]
pub struct WhaleXPaceState {
    pub score_table: Vec<DataPlayerWhaleXPace>
}

impl WhaleXPaceState {
    pub fn new () -> Self {
        let temp = Self::default();
        temp
    }
}