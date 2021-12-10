use cosmwasm_std::{
    debug_print, to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier,
    StdError, StdResult, Storage,
};

use crate::msg::{GameStateResponse, HandleMsg, InitMsg, QueryMsg};
use crate::state::{config, config_read, GameState, Moves, State};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    _msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State {
        game_state: crate::state::GameState::Playing,
        owner: env.message.sender,
        player_1: None,
        player_1_move: None,
        player_2: None,
        winner: None,
        player_2_move: None,
    };

    config(&mut deps.storage).save(&state)?;

    debug_print!("Contract was initialized by {}", &state.owner);

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::Join { player_move } => try_join(deps, env, player_move),
    }
}

pub fn try_join<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    player_move: Moves,
) -> StdResult<HandleResponse> {
    let mut state = State::load(&deps.storage)?;

    if state.player_1.is_none() {
        state.player_1 = Some(env.message.sender);
        state.player_1_move = Some(player_move);

        state.save(&mut deps.storage)?;
        Ok(HandleResponse::default())
    } else if state.player_2.is_none() {
        state.player_2 = Some(env.message.sender);
        state.player_2_move = Some(player_move);

        let result = match (
            &state.player_1_move.as_ref().unwrap(),
            &state.player_2_move.as_ref().unwrap(),
        ) {
            (Moves::Paper, Moves::Scissors) => GameState::Player2Win,
            (Moves::Block, Moves::Paper) => GameState::Player2Win,
            (Moves::Scissors, Moves::Block) => GameState::Player2Win,
            (Moves::Block, Moves::Scissors) => GameState::Player1Win,
            (Moves::Paper, Moves::Block) => GameState::Player1Win,
            (Moves::Scissors, Moves::Paper) => GameState::Player1Win,
            (Moves::Block, Moves::Block) => GameState::Draw,
            (Moves::Paper, Moves::Paper) => GameState::Draw,
            (Moves::Scissors, Moves::Scissors) => GameState::Draw,
        };
        state.game_state = result;

        state.winner = match &state.game_state {
            GameState::Player1Win => Some(state.player_2.as_ref().unwrap().clone()),
            GameState::Player2Win => Some(state.player_1.as_ref().unwrap().clone()),
            GameState::Draw => None,
            _ => None,
        };

        state.save(&mut deps.storage)?;

        Ok(HandleResponse::default())
    } else {
        return Err(StdError::generic_err("Room is full"));
    }
}
// pub fn save_player<S: Storage>(storage: &S, env: Env) -> StdResult<HandleResponse> {
//     config(storage).update(|mut state| {
//         state.player_1_move = Some(Moves::Block);

//         Ok(state)
//     });
//     Ok(HandleResponse::default())
// }

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetGameState {} => to_binary(&query_game_state(deps)?),
    }
}
fn query_game_state<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
) -> StdResult<GameStateResponse> {
    let state = config_read(&deps.storage).load()?;
    Ok(GameStateResponse {
        state: state.game_state,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(20, &[]);
        let msg = InitMsg {};
        let env = mock_env("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = init(&mut deps, env, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(&deps, QueryMsg::GetGameState {}).unwrap();
        let value: GameStateResponse = from_binary(&res).unwrap();
        println!("{:?}", value);
        assert_eq!(GameState::Playing, value.state);
    }
    #[test]
    fn play() {
        let mut deps = mock_dependencies(20, &coins(2, "earth"));

        let player_1_env = mock_env("player1", &coins(1000, "earth"));
        let player_2_env = mock_env("player2", &coins(1000, "earth"));

        //create a room, anyone can do so.
        let msg = InitMsg {};
        let env = mock_env("creator", &coins(1000, "earth"));
        let _res = init(&mut deps, env, msg);

        //player 1 joins room and sends his move
        let msg = HandleMsg::Join {
            player_move: Moves::Paper,
        };
        let _res = handle(&mut deps, player_1_env, msg);

        //player 2 joins room and sends his move
        let msg = HandleMsg::Join {
            player_move: Moves::Block,
        };
        let _res = handle(&mut deps, player_2_env, msg);
        //get the result and assert if wrong
        let res = query(&deps, QueryMsg::GetGameState {}).unwrap();
        let value: GameStateResponse = from_binary(&res).unwrap();
        assert_eq!(GameState::Player1Win, value.state);
    }
}
