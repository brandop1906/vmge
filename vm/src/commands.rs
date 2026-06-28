#[derive(Debug)]
pub enum Command {
    PlaceCharacter { x: i16, y: i16, character_id: u8 },
    SetPartyLeader { character_id: u8 },
    Message { window_id: u8, message_id: u16 },
    WindowOpen { x: i16, y: i16, width: i16, height: i16, window_id: u8},
    WindowClose { window_id: u8 },
    SetSolid { character_id: u8, enabled: bool },
}