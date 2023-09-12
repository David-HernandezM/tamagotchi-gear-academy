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

const HUNGER_PER_BLOCK: u64 = 1;
const ENERGY_PER_BLOCK: u64 = 2;
const BOREDOM_PER_BLOCK: u64 = 2;
const FILL_PER_SLEEP: u64 = 1000;
const FILL_PER_FEED: u64 = 1000;
const FILL_PER_ENTERTAINMENT: u64 = 1000;

static mut TAMAGOTCHI: Option<Tamagotchi> = None;

fn state_mut() -> &'static mut Tamagotchi {
    let state = unsafe { TAMAGOTCHI.as_mut() };

    debug_assert!(state.is_some(), "state isn't initialized");

    unsafe { state.unwrap_unchecked() }
}

fn state_ref() -> &'static Tamagotchi {
    let state = unsafe { TAMAGOTCHI.as_ref() };

    debug_assert!(state.is_some(), "state isn't initialized");

    unsafe { state.unwrap_unchecked() }
}

#[no_mangle]
unsafe extern "C" fn init() {
    let tamagotchi_name: String = msg::load().expect("Error in init message");
    let new_tamagotchi: Tamagotchi = Tamagotchi {
        name: tamagotchi_name,
        date_of_birth: exec::block_timestamp(),
        owner: msg::source(),
        fed: 5000,
        fed_block: 0,
        entertained: 5000,
        entertained_block: 0,
        rested: 5000,
        rested_block: 0
    };
    TAMAGOTCHI = Some(new_tamagotchi);
    msg::reply("successful initialization!", 0).expect("error in reply");
}

#[no_mangle]
unsafe extern "C" fn handle() {
    let type_message: TmgAction = msg::load().expect("error in load message");
    let tamagotchi = state_mut();

    let blocksHeight = blocks_height();

    match type_message {
        TmgAction::Name => {
            let tamagotchiName: TmgEvent = TmgEvent::Name(String::from(&tamagotchi.name));
            msg::reply(tamagotchiName, 0).expect("Error in sending tamagotchi name");
        },
        TmgAction::Age => {
            let tamagotchiAge: u64 = exec::block_timestamp();
            tamagotchi.date_of_birth = tamagotchiAge;
            msg::reply(TmgEvent::Age(tamagotchiAge), 0).expect("Errorin sending tamagotchi age");
        },
        TmgAction::Feed => {
            tamagotchi.fed = tamagotchi_fed_updated(blocksHeight);
            tamagotchi.fed_block = blocksHeight;
            msg::reply(TmgEvent::Fed, 0).expect("Error sending tamagotchi variant 'Fed'");
        },
        TmgAction::Play => {
            tamagotchi.entertained = tamagotchi_entertained_updated(blocksHeight);
            tamagotchi.entertained_block = blocksHeight;
            msg::reply(TmgEvent::Entertained, 0).expect("Error sending tamagotchi variant 'Entertained'");  
        },
        TmgAction::Sleep => {
            tamagotchi.rested = tamagotchi_rested_updated(blocksHeight);
            tamagotchi.rested_block = blocksHeight;
            msg::reply(TmgEvent::Slept, 0).expect("Error sending tamagotchi variant 'Slept'");  
        }
    }
}

#[no_mangle]
extern "C" fn state() {
    let blocksHeight = blocks_height();
    let tamagotchi = state_ref();
    let state = Tamagotchi {
        name: String::from(&tamagotchi.name),
        date_of_birth: tamagotchi.date_of_birth,
        owner: tamagotchi.owner.clone(),
        fed: tamagotchi_total_fed(blocksHeight),
        fed_block: tamagotchi.fed_block,
        entertained: tamagotchi_total_entertained(blocksHeight),
        entertained_block: tamagotchi.entertained_block,
        rested: tamagotchi_total_rested(blocksHeight),
        rested_block: tamagotchi.rested_block
    };
    msg::reply(tamagotchi, 0).expect("Failed to share state");
}

fn blocks_height() -> u64 {
    u64::from(exec::block_height())
}

fn tamagotchi_total_fed(blocksHeight: u64) -> u64 {
    let tamagotchi = state_ref();
    let tamagotchi_hungry = (blocksHeight - tamagotchi.fed_block) * HUNGER_PER_BLOCK;
    

    if tamagotchi.fed > tamagotchi_hungry {
        tamagotchi.fed - tamagotchi_hungry
    } else {
        0
    }
}

fn tamagotchi_total_entertained(blocksHeight: u64) -> u64 {
    let tamagotchi = state_ref();
    let tamagotchi_bored = (blocksHeight - tamagotchi.entertained_block) * BOREDOM_PER_BLOCK;
    
    if tamagotchi.entertained > tamagotchi_bored {
        tamagotchi.entertained - tamagotchi_bored
    } else {
        0
    }
}

fn tamagotchi_total_rested(blocksHeight: u64) -> u64 {
    let tamagotchi = state_ref();
    let tamagotchi_tired = (blocksHeight - tamagotchi.rested_block) * ENERGY_PER_BLOCK;

    if tamagotchi.rested > tamagotchi_tired {
        tamagotchi.rested - tamagotchi_tired
    } else {
        0
    }
}

fn tamagotchi_fed_updated(blocksHeight: u64) -> u64 {
    let tamagotchi = state_ref();
    let actual_fed = tamagotchi_total_fed(blocksHeight);
    let total_fed = actual_fed + FILL_PER_FEED;

    if total_fed <= 10_000 {
        total_fed
    } else {
        10_000
    }
}

fn tamagotchi_entertained_updated(blocksHeight: u64) -> u64 {
    let tamagotchi = state_ref();
    let actual_entertained = tamagotchi_total_entertained(blocksHeight);
    let total_entertained = actual_entertained + FILL_PER_ENTERTAINMENT;

    if total_entertained <= 10_000 {
        total_entertained
    } else {
        10_000
    }
}

fn tamagotchi_rested_updated(blocksHeight: u64) -> u64 {
    let tamagotchi = state_ref();
    let actual_rested = tamagotchi_total_rested(blocksHeight);
    let total_rested = actual_rested + FILL_PER_SLEEP;

    if total_rested <= 10_000 {
        total_rested
    } else {
        0
    }
}