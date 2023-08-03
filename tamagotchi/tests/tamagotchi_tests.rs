use gtest::{ Log, System, Program };
use program_io::{ TmgAction, TmgEvent };

#[test]
fn tamagotchi_test_init() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    let res = program.send(2, String::from("DavidHM"));

    let  expected_init_log = Log::builder()
        .dest(2)
        .payload("successful initialization!");

    assert!(res.contains(&expected_init_log));
    assert!(!res.main_failed());
}

#[test]
fn check_correct_name() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    let res = program.send(2, String::from("david"));
    let  expected_name_log = Log::builder()
        .dest(2)
        .payload(TmgEvent::Name(String::from("david")));

    let res = program.send(
        2,
        TmgAction::Name
    );

    assert!(res.contains(&expected_name_log));

}