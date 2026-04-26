#![no_std]
use soroban_sdk::{contract, contractevent, contractimpl, Env, Symbol};

#[contract]
pub struct LivePollContract;

#[contractevent(topics = ["voted"], data_format = "single-value")]
pub struct VoteCast {
    #[topic]
    pub option: Symbol,
    pub votes: u32,
}

#[contractimpl]
impl LivePollContract {
    // Cast a vote for a specific option (e.g., "OptionA" or "OptionB")
    pub fn vote(env: Env, option: Symbol) {
        // Get current votes, default to 0 if none exist
        let current_votes: u32 = env.storage().instance().get(&option).unwrap_or(0);
        let new_votes = current_votes + 1;
        
        // Save the new vote count
        env.storage().instance().set(&option, &new_votes);

        // Emit an event so the frontend can stream live tallies from Soroban.
        VoteCast {
            option,
            votes: new_votes,
        }
        .publish(&env);
    }

    // Read the current vote count for an option
    pub fn get_votes(env: Env, option: Symbol) -> u32 {
        env.storage().instance().get(&option).unwrap_or(0)
    }
}

#[cfg(test)]
mod test;
