#![no_std]

use gmeta::{InOut, Metadata, Out};
use gstd::{prelude::*, ActorId, MessageId};
use wordle_io::Event;
pub struct GameSessionMetadata;

impl Metadata for GameSessionMetadata {
    type Init = InOut<ActorId, SessionEvent>;
    type Handle = InOut<SessionAction, SessionEvent>;
    type Reply = InOut<Event, SessionEvent>;
    type Others = ();
    type Signal = ();
    type State = Out<State>;
}

#[derive(Encode, Decode, TypeInfo, Clone, Debug)]
pub enum SessionAction {
    StartGame,
    CheckWord { word: String },
    CheckGameStatus { user: ActorId },
}

#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq)]
pub enum SessionEvent {
    Initialized,
    GameStarted,
    WordChecked {
        correct_positions: Vec<u8>,
        contained_in_word: Vec<u8>,
    },
    GameOver {
        result: SessionResult,
    },
}

#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq)]
pub enum SessionStatus {
    StartGameWaiting,
    StartGameSent,
    CheckWordWaiting,
    CheckWordSent,
    ReplyReceived(SessionEvent),
}

#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq)]
pub enum SessionResult {
    Ongoing,
    Win,
    Lose,
}

#[derive(Encode, Decode, TypeInfo, Clone, Debug)]
pub struct Session {
    pub start_block: u32,
    pub check_count: u8,
    pub msg_ids: (MessageId, MessageId),
    pub status: SessionStatus,
    pub result: SessionResult,
}

#[derive(Encode, Decode, TypeInfo, Clone)]
pub struct GameSessionState {
    pub wordle_program: ActorId,
    pub user_to_session: Vec<(ActorId, Session)>,
}

#[derive(Encode, Decode, TypeInfo)]
pub struct State {
    pub wordle_program: ActorId,
    pub user_to_session: Vec<(ActorId, Session)>,
}
