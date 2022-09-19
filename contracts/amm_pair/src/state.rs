use cosmwasm_std::{Api, CanonicalAddr, StdResult, Storage, Addr};
use cosmwasm_storage::{singleton, singleton_read, Singleton, ReadonlySingleton, Bucket, ReadonlyBucket, bucket, bucket_read};
use serde::{Deserialize, Serialize};
use shadeswap_shared::{msg::amm_pair::TradeHistory, core::{ContractLink, TokenPair, CustomFee, ViewingKey}};

pub const PAGINATION_LIMIT: u8 = 30;
pub static CONFIG: &[u8] = b"config";
pub static TRADE_COUNT: &[u8] = b"tradecount";
pub static TRADE_HISTORY: &[u8] = b"trade_history";
pub static WHITELIST: &[u8] = b"whitelist";
pub const BLOCK_SIZE: usize = 256;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Config {
    pub factory_contract: ContractLink,
    pub lp_token: ContractLink,
    pub staking_contract: Option<ContractLink>,
    pub pair: TokenPair,
    pub viewing_key: ViewingKey,
    pub custom_fee: Option<CustomFee>
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum DirectionType {
    Buy,
    Sell,
    Unknown,
}

pub fn config_w(storage: &mut dyn Storage) -> Singleton<Config> {
    singleton(storage, CONFIG)
}

pub fn config_r(storage: &dyn Storage) -> ReadonlySingleton<Config> {
    singleton_read(storage, CONFIG)
}

pub fn trade_count_w(storage: &mut dyn Storage) -> Singleton<u64> {
    singleton(storage, CONFIG)
}

pub fn trade_count_r(storage: &dyn Storage) -> ReadonlySingleton<u64> {
    singleton_read(storage, CONFIG)
}


pub fn whitelist_w(storage: &mut dyn Storage) -> Singleton<Vec<Addr>> {
    singleton(storage, CONFIG)
}

pub fn whitelist_r(storage: &dyn Storage) -> ReadonlySingleton<Vec<Addr>> {
    singleton_read(storage, CONFIG)
}

pub fn trade_history_w(storage: &mut dyn Storage) -> Bucket<TradeHistory> {
    bucket(storage, CONFIG)
}

pub fn trade_history_r(storage: &dyn Storage) -> ReadonlyBucket<TradeHistory> {
    bucket_read(storage, CONFIG)
}
