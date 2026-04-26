#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, Env};

#[test]
fn vote_tracks_each_option_independently() {
    let env = Env::default();
    let contract_id = env.register(LivePollContract, ());
    let client = LivePollContractClient::new(&env, &contract_id);
    let option_a = symbol_short!("OptionA");
    let option_b = symbol_short!("OptionB");

    assert_eq!(client.get_votes(&option_a), 0);
    assert_eq!(client.get_votes(&option_b), 0);

    client.vote(&option_a);
    client.vote(&option_a);
    client.vote(&option_b);

    assert_eq!(client.get_votes(&option_a), 2);
    assert_eq!(client.get_votes(&option_b), 1);
}
