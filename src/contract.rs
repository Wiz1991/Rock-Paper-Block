use cosmwasm_std::{
    debug_print, to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier,
    StdError, StdResult, Storage,
};

use crate::msg::{GameStateResponse, HandleMsg, InitMsg, QueryMsg};
use crate::state::{config, config_read, GameState, Moves, State};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State {
        owner_move: msg.owner_move,
        invitee: deps.api.canonical_address(&msg.invitee)?,
        game_state: crate::state::GameState::Playing,
        owner: env.message.sender,
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
        HandleMsg::Play { player_move } => try_play(deps, env, player_move),
    }
}

pub fn try_play<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    player_move: Moves,
) -> StdResult<HandleResponse> {
    let sender_adress_raw = deps.api.canonical_address(&env.message.sender)?;
    config(&mut deps.storage).update(|mut state| {
        if sender_adress_raw != state.invitee {
            return Err(StdError::Unauthorized { backtrace: None });
        }
        if state.game_state != GameState::Playing {
            return Err(StdError::GenericErr {
                msg: "Game is over.".to_string(),
                backtrace: None,
            });
        }

        let result = match (player_move, &state.owner_move) {
            (Moves::Paper, Moves::Scissors) => GameState::OwnerWin,
            (Moves::Block, Moves::Paper) => GameState::OwnerWin,
            (Moves::Block, Moves::Scissors) => GameState::InviteeWin,
            (Moves::Paper, Moves::Block) => GameState::InviteeWin,
            (Moves::Scissors, Moves::Paper) => GameState::InviteeWin,
            (Moves::Block, Moves::Block) => GameState::Draw,
            (Moves::Paper, Moves::Paper) => GameState::Draw,
            (Moves::Scissors, Moves::Block) => GameState::Draw,
            (Moves::Scissors, Moves::Scissors) => GameState::Draw,
        };
        state.game_state = result;

        Ok(state)
    })?;
    Ok(HandleResponse::default())
}

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
        let player_2_env = mock_env("player2", &coins(1000, "earth"));
        let msg = InitMsg {
            invitee: player_2_env.contract.address,
            owner_move: Moves::Block,
        };
        let env = mock_env("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = init(&mut deps, env, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(&deps, QueryMsg::GetGameState {}).unwrap();
        let value: GameStateResponse = from_binary(&res).unwrap();
        println!("{:?}",value);
        assert_eq!(GameState::Playing, value.state);
    }
    #[test]
    fn play() {
        let mut deps = mock_dependencies(20, &coins(2, "earth"));

        let player_2_env = mock_env("player2", &coins(1000, "earth"));

        let msg = InitMsg {
            invitee: player_2_env.contract.address.clone(),
            owner_move: Moves::Block,
        };
        println!("{:?}", msg);
        let env = mock_env("creator", &coins(1000, "earth"));
        let _res = init(&mut deps, env, msg);

        let msg = HandleMsg::Play {
            player_move: Moves::Paper,
        };
        let _res = handle(&mut deps, player_2_env, msg).unwrap();

        let res = query(&deps, QueryMsg::GetGameState {}).unwrap();
        let value: GameStateResponse = from_binary(&res).unwrap();
        assert_eq!(GameState::InviteeWin, value.state);
    }
}
