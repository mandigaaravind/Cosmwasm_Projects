use cosmwasm_std::{Addr, Storage, Coin};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton,Bucket, ReadonlyBucket,bucket, bucket_read,};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

static CONFIG_KEY: &[u8] = b"config";
pub static NAME_RESOLVER_KEY: &[u8] = b"nameresolver";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub arbiter: Addr,
    pub source: Addr,
    pub maxlimit: Option<Coin>,
    pub minlimit: Option<Coin>,
}



pub fn config(storage: &mut dyn Storage) -> Singleton<State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<State> {
    singleton_read(storage, CONFIG_KEY)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GameRecord {
    pub owner: Addr,
    pub sum_prediction : u8,
    pub sum_actual : u8,
    pub sum_dice1 :u8,
    pub sum_dice2 : u8,
    pub is_winner : bool,
    pub entry_fee : Option<Coin>,
}

pub fn resolver(storage: &mut dyn Storage) -> Bucket<GameRecord> {
    bucket(storage, NAME_RESOLVER_KEY)
}

pub fn resolver_read(storage: &dyn Storage) -> ReadonlyBucket<GameRecord> {
    bucket_read(storage, NAME_RESOLVER_KEY)
}

