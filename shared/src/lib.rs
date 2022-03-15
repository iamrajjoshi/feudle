pub type PlayerId = u8;

pub enum StreamId {
    Heartbeat = 0,
    Event = 1,
}

// All packets are prepended by [magic_byte, message_type]
pub enum MessageType {
    Heartbeat = 0,         // [] perhaps a timestamp?
    JoinEvent = 1,         // []
    AssignIdEvent = 2,     // [assigned_id]
    ReadyEvent = 3,        // [id]
    StartEvent = 4,        // [word]
    GuessEvent = 5,        // [id, guess]
    FinishEvent = 6,       // [id, num_guesses]
    EndEvent = 7,          // [id_winner]
}

pub const MAGIC_BYTE: u8 = 42;
pub const MAX_PLAYERS: usize = 2;