#![no_std]

use gear_lib::non_fungible_token::{
    state::NFTQueryReply,
    token::{Token, TokenId},
};
use gmeta::{metawasm, Metadata};
use gstd::{ActorId, Vec, prelude::*};
use program_io::ProgramMetadata;

#[metawasm]
pub mod metafns {
    pub type State = <ProgramMetadata as Metadata>::State;

    pub fn data(state: State) -> String {
        let tamagotchi_data: String = format!("{} - {}", state.name, state.date_of_birth);
        tamagotchi_data
    }
}

