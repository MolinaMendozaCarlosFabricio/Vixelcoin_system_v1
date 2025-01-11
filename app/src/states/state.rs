use sails_rs::{
    prelude::*,
    collections::HashMap
};

pub struct AcountInformation {
    pub username: String,
    pub balance_vixelcoins: u128,
}

#[derive(Default)]
pub struct VixelCoinSystemState {
    pub admins: Vec<ActorId>,
    pub users: HashMap<ActorId, AcountInformation>,
    pub amount_vixelcoins_total_in_the_system: Option<u128>,
    // pub value_of_vixelcoin_in_vara: Option<u128>
}

impl VixelCoinSystemState {
    pub fn new (new_admin: ActorId) -> Self {
        let mut temp = Self::default();
        temp.admins.push(new_admin);
        temp
    }
}