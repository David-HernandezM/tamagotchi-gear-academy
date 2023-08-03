use gear_lib::non_fungible_token::{io::NFTTransfer, nft_core::*, state::*, token::*};
use gear_lib_derive::{NFTCore, NFTMetaState, NFTStateKeeper};
use gmeta::Metadata;
use gstd::{errors::Result as GstdResult, exec, msg, prelude::*, ActorId, MessageId};
use hashbrown::HashMap;
use primitive_types::{H256, U256};
use program_io::{
    ProgramMetadata,
    Tamagotchi,
    TmgAction,
    TmgEvent
};

static mut TAMAGOTCHI: Option<Tamagotchi> = None;

fn state_mut() -> &'static mut Tamagotchi {
    let state = unsafe { TAMAGOTCHI.as_mut() };

    debug_assert!(state.is_some(), "state isn't initialized");

    unsafe { state.unwrap_unchecked() }
}

#[no_mangle]
unsafe extern "C" fn init() {
    let tamagotchi_name: String = msg::load().expect("Error in init message");
    let new_tamagotchi: Tamagotchi = Tamagotchi {
        name: tamagotchi_name,
        date_of_birth: exec::block_timestamp()
    };
    TAMAGOTCHI = Some(new_tamagotchi);
    msg::reply("successful initialization!", 0).expect("error in reply");
}

#[no_mangle]
unsafe extern "C" fn handle() {
    let type_message: TmgAction = msg::load().expect("error in load message");
    let tamagotchi = TAMAGOTCHI
        .as_mut()
        .expect("The contract is no initialized");
    match type_message {
        TmgAction::Name => {
            let tamagotchiName: TmgEvent = TmgEvent::Name(String::from(&tamagotchi.name));
            msg::reply(tamagotchiName, 0).expect("Error in sending tamagotchi name");
        },
        TmgAction::Age => {
            let tamagotchiAge: u64 = exec::block_timestamp();
            tamagotchi.date_of_birth = tamagotchiAge;
            msg::reply(TmgEvent::Age(tamagotchiAge), 0).expect("Errorin sending tamagotchi age");
        }
    }
}

#[no_mangle]
extern "C" fn state() {
    let tamagotchi = state_mut();
    msg::reply(tamagotchi, 0).expect("Failed to share state");
}