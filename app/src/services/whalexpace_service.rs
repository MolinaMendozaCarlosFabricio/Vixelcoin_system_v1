use sails_rs::{
    gstd::msg::{self}, 
    prelude::*
};

use crate::states::whalexpace_state::{
    WhaleXPaceState,
    DataPlayerWhaleXPace
};

static mut WHALEXPACE_STATE: Option<WhaleXPaceState> = None;

#[derive(Default)]
pub struct WhaleXPaceService();

#[sails_rs::service]
impl WhaleXPaceService {
    pub fn seed (){
        unsafe {
            WHALEXPACE_STATE = Some(
                WhaleXPaceState{
                    score_table: Vec::<DataPlayerWhaleXPace>::new()
                }
            );
        }
    }

    pub fn new () -> Self {
        Self{}
    }

    pub fn set_score (&mut self, username: String, score: u32) -> WhaleXPaceEvents {
        let id_actor = msg::source();
        let state = Self::state_mut();

        if state.score_table.len() < 10 {
            state.score_table.push(DataPlayerWhaleXPace{ address: id_actor, username: username.clone(), score: score });
            return WhaleXPaceEvents::ScoreRegistred { 
                message: "Puntaje registrado".to_string(), id_actor: id_actor, score: score 
            };
        }

        if score < state.score_table[state.score_table.len() - 1].score && state.score_table.len() == 10 {
            return WhaleXPaceEvents::PlayerDontHaveEnoughScore;
        }

        if state.score_table.len() == 10 {
            state.score_table.pop();
        }

        state.score_table.push(DataPlayerWhaleXPace{ address: id_actor, username: username.clone(), score: score });
        state.score_table.sort_by_key(|user_data| user_data.score);

        return WhaleXPaceEvents::ScoreRegistred { 
            message: "Puntaje registrado".to_string(), id_actor: id_actor, score: score 
        };
    }

    pub fn get_first_teen_players (&self) -> WhaleXPaceEvents {
        let state = Self::state_mut();
        
        let scores = state.score_table
            .iter()
            .map(|user_data| user_data.clone())
            .collect();
        
        WhaleXPaceEvents::FirstTeenBestPlayers(scores)
    }

    pub fn state_mut() -> &'static mut WhaleXPaceState {
        let state = unsafe { WHALEXPACE_STATE.as_mut() };
        debug_assert!(state.is_none(), "El estado no ha sido inicializado");
        unsafe { state.unwrap_unchecked() }
    }
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum WhaleXPaceEvents {
    ScoreRegistred {
        message: String,
        id_actor: ActorId,
        score: u32
    },
    PlayerDontHaveEnoughScore,
    FirstTeenBestPlayers(Vec<DataPlayerWhaleXPace>),
}