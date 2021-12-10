use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{HumanAddr, StdResult, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub player_1: Option<HumanAddr>,
    pub player_1_move: Option<Moves>,

    pub player_2: Option<HumanAddr>,
    pub player_2_move: Option<Moves>,

    pub game_state: GameState,
    pub winner: Option<HumanAddr>,
    pub owner: HumanAddr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum Moves {
    Block,
    Paper,
    Scissors,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum GameState {
    Playing,
    Player1Win,
    Player2Win,
    Draw,
}

pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, CONFIG_KEY)
}

impl State {
    pub fn save<S: Storage>(&self, storage: &mut S) -> StdResult<()> {
        Singleton::new(storage, b"state").save(self)
    }
    pub fn load<S: Storage>(storage: &S) -> StdResult<State> {
        ReadonlySingleton::new(storage, b"state").load()
    }
}
