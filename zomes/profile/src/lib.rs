mod entries;
mod handler;
use entries::*;
use hdk::prelude::*;
mod error;
mod validation;
use hc_utils::WrappedAgentPubKey;

entry_defs![Profile::entry_def()];

#[hdk_extern]
fn update_my_profile(profile_input: ProfileInput) -> ExternResult<Profile> {
    Ok(handler::__update_my_profile(profile_input)?)
}

#[hdk_extern]
fn get_my_profile(_: ()) -> ExternResult<Profile> {
    Ok(handler::__get_my_profile()?)
}

#[hdk_extern]
fn get_profile(agent_address: WrappedAgentPubKey) -> ExternResult<Profile> {
    Ok(handler::__get_profile(AgentPubKey::from(agent_address))?)
}

#[hdk_extern]
fn validate_create_entry_profile(
    entry: ValidateData,
) -> ExternResult<ValidateCallbackResult> {
    Ok(validation::__validate_profile(entry)?)
}

#[hdk_extern]
fn validate_update_entry_profile(
    entry: ValidateData,
) -> ExternResult<ValidateCallbackResult> {
    Ok(validation::__validate_profile(entry)?)
}
