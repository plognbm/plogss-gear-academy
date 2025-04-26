use game_session_io::*;
use gtest::{Program, System};

const USER: u64 = 5;
const BALANCE: u128 = 500000000000000;

#[test]
fn test_start_game() {
    let system = System::new();
    system.init_logger();
    system.mint_to(USER, BALANCE);
    let game_session_program = Program::from_file(
        &system,
        "../target/wasm32-unknown-unknown/debug/game_session.opt.wasm",
    );
    let wordle_program = Program::from_file(
        &system,
        "../target/wasm32-unknown-unknown/debug/wordle.opt.wasm",
    );
    wordle_program.send_bytes(USER, []);
    system.run_next_block();

    game_session_program.send(USER, wordle_program.id());
    system.run_next_block();

    game_session_program.send(USER, SessionAction::StartGame);
    system.run_next_block();

    // Check game state
    let state: State = game_session_program
        .read_state("")
        .expect("Failed to read state");
    assert_eq!(state.user_to_session.len(), 1);
    assert_eq!(
        state.user_to_session[0].1.status,
        SessionStatus::CheckWordWaiting
    );
}

#[test]
fn test_check_word() {
    let system = System::new();
    system.init_logger();
    system.mint_to(USER, BALANCE);
    let game_session_program = Program::from_file(
        &system,
        "../target/wasm32-unknown-unknown/debug/game_session.opt.wasm",
    );
    let wordle_program = Program::from_file(
        &system,
        "../target/wasm32-unknown-unknown/debug/wordle.opt.wasm",
    );
    wordle_program.send_bytes(USER, []);
    system.run_next_block();


    game_session_program.send(USER, wordle_program.id());
    system.run_next_block();

    game_session_program.send(USER,SessionAction::StartGame);
    system.run_next_block();

    // Check word
    game_session_program.send(
        USER,
        SessionAction::CheckWord {
            word: "wwwww".to_string(),
        },
    );
    system.run_next_block();

    // Check game state
    let state: State = game_session_program.read_state("").expect("Failed to read state");
    assert_eq!(state.user_to_session[0].1.check_count, 1);
    assert_eq!(
        state.user_to_session[0].1.status,
        SessionStatus::CheckWordWaiting
    );
}

#[test]
fn test_game_timeout() {
    let system = System::new();
    system.init_logger();
    system.mint_to(USER, BALANCE);
    let game_session_program = Program::from_file(
        &system,
        "../target/wasm32-unknown-unknown/debug/game_session.opt.wasm",
    );
    let wordle_program = Program::from_file(
        &system,
        "../target/wasm32-unknown-unknown/debug/wordle.opt.wasm",
    );
    wordle_program.send_bytes(USER, []);
    system.run_next_block();

    game_session_program.send(USER, wordle_program.id());
    system.run_next_block();

    game_session_program.send(USER,SessionAction::StartGame);
    system.run_next_block();

    // Advance time by 11 minutes
    system.run_to_block(220);

    // Check game status
    game_session_program.send(USER, SessionAction::CheckGameStatus { user: USER.into() });
    system.run_next_block();

    // Check game state
    let state: State = game_session_program.read_state("").expect("Failed to read state");
    assert_eq!(state.user_to_session[0].1.result, SessionResult::Lose);
}

#[test]
fn test_game_win() {
    let system = System::new();
    system.init_logger();
    system.mint_to(USER, BALANCE);
    let game_session_program = Program::from_file(
        &system,
        "../target/wasm32-unknown-unknown/debug/game_session.opt.wasm",
    );
    let wordle_program = Program::from_file(
        &system,
        "../target/wasm32-unknown-unknown/debug/wordle.opt.wasm",
    );
    wordle_program.send_bytes(USER, []);
    system.run_next_block();

    game_session_program.send(USER, wordle_program.id());
    system.run_next_block();

    game_session_program.send(USER,SessionAction::StartGame);
    system.run_next_block();

    // Check word (assuming the secret word is "house")
    game_session_program.send(
        USER,
        SessionAction::CheckWord {
            word: "house".to_string(),
        },
    );
    system.run_next_block();

    game_session_program.send(
        USER,
        SessionAction::CheckWord {
            word: "human".to_string(),
        },
    );
    system.run_next_block();

    game_session_program.send(
        USER,
        SessionAction::CheckWord {
            word: "horse".to_string(),
        },
    );
    system.run_next_block();

    // Check game state
    let state: State = game_session_program.read_state("").expect("Failed to read state");
    assert_eq!(state.user_to_session[0].1.result, SessionResult::Win);
}

#[test]
fn test_game_lose() {
    let system = System::new();
    system.init_logger();
    system.mint_to(USER, BALANCE);
    let game_session_program = Program::from_file(
        &system,
        "../target/wasm32-unknown-unknown/debug/game_session.opt.wasm",
    );
    let wordle_program = Program::from_file(
        &system,
        "../target/wasm32-unknown-unknown/debug/wordle.opt.wasm",
    );
    wordle_program.send_bytes(USER, []);
    system.run_next_block();

    game_session_program.send(USER, wordle_program.id());
    system.run_next_block();

    game_session_program.send(USER,SessionAction::StartGame);
    system.run_next_block();

    // Make 6 incorrect guesses
    for _ in 0..6 {
        game_session_program.send(
            USER,
            SessionAction::CheckWord {
                word: "wrong".to_string(),
            },
        );
        system.run_next_block();
    }

    // Check game state
    let state: State = game_session_program.read_state("").expect("Failed to read state");
    assert_eq!(state.user_to_session[0].1.result, SessionResult::Lose);
}
