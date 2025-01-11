use sails_rs::prelude::*;

#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct DataPlayerCheemsScape {
    pub address: ActorId,
    pub username: String,
    pub score: u32
}

#[derive(Default)]
pub struct CheemsScapeState {
    pub score_table: Vec<DataPlayerCheemsScape>
}

impl CheemsScapeState {
    
    pub fn new () -> Self {
        let temp = Self::default();
        temp
    }
    
}