use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use mysimplecounter::msg::{GameStateResponse, HandleMsg, InitMsg, QueryMsg};
use mysimplecounter::state::State;

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InitMsg), &out_dir);
    export_schema(&schema_for!(HandleMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(State), &out_dir);
    export_schema(&schema_for!(GameStateResponse), &out_dir);
}
/*

[
  {
    "id": 1,
    "creator": "secret1wah7fzr4af6vwn98k2y9xe9ru2zz5aw7uy09t4",
    "data_hash": "DEDE2D8652D4F39CE954542E3FAC1CA8C37796E255CD53D8397C48C4235128F4"
  }
]




*/
