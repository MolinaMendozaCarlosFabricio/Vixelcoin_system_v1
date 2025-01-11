use sails_rs::{
    prelude::*,
    gstd::msg
};

use crate::states::cheems_scape_state::{
    DataPlayerCheemsScape,
    CheemsScapeState
};

static mut CHEEMS_SCAPE_STATE: Option<CheemsScapeState> = None;

#[derive(Default)]
pub struct CheemsScapeService ();

#[sails_rs::service]
impl CheemsScapeService {

    pub fn seed () {
        unsafe {
            CHEEMS_SCAPE_STATE = Some(
                CheemsScapeState { score_table: Vec::<DataPlayerCheemsScape>::new() }
            )
        }
    }

    pub fn new () -> Self {
        Self{}
    }

    pub fn set_score (&mut self, username: String, score: u32) -> CheemsScapeEvents {
        let state = self.state_mut();
        let id_actor = msg::source();

        if state.score_table.len() < 10 {
            state.score_table.push(DataPlayerCheemsScape { address: id_actor, username: username, score: score });
            return CheemsScapeEvents::ScoreRegistred { 
                message: "Puntaje registrado".to_string(), id_actor: id_actor, score: score 
            };
        }

        if score < state.score_table[state.score_table.len() - 1].score {
            return CheemsScapeEvents::PlayerDontHaveEnoughScore;
        }

        if state.score_table.len() == 10 {
            state.score_table.pop();
        }

        state.score_table.push(DataPlayerCheemsScape { address: id_actor, username: username, score: score });
        state.score_table.sort_by_key(|player| player.score);

        return CheemsScapeEvents::ScoreRegistred { 
            message: "Puntaje registrado".to_string(), id_actor: id_actor, score: score 
        };
    }

    pub fn get_first_teen_players(&self) -> CheemsScapeEvents {
        let state = self.state_mut();

        let scores = state.score_table
            .iter()
            .map(|player| player.clone())
            .collect();

        CheemsScapeEvents::FirstTeenBestPlayers(scores)
    }

    fn state_mut (&self) -> &'static mut CheemsScapeState {
        let state = unsafe { CHEEMS_SCAPE_STATE.as_mut() };
        debug_assert!(state.is_none(), "Estado no inicializado");
        unsafe { state.unwrap_unchecked() }
    }
}

#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum CheemsScapeEvents {
    ScoreRegistred {
        message: String,
        id_actor: ActorId,
        score: u32
    },
    PlayerDontHaveEnoughScore,
    FirstTeenBestPlayers(Vec<DataPlayerCheemsScape>),
}