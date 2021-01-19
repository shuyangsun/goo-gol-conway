#[derive(Debug, Clone, PartialEq)]
pub enum ConwayState {
    Alive,
    Dead,
}

const CONWAY_STATE_ALIVE_CHAR: char = '0';
const CONWAY_STATE_DEAD_CHAR: char = ' ';

impl std::convert::Into<char> for ConwayState {
    fn into(self) -> char {
        match self {
            ConwayState::Alive => CONWAY_STATE_ALIVE_CHAR,
            ConwayState::Dead => CONWAY_STATE_DEAD_CHAR,
        }
    }
}

impl std::convert::From<char> for ConwayState {
    fn from(ch: char) -> Self {
        match ch {
            CONWAY_STATE_ALIVE_CHAR => ConwayState::Alive,
            _ => ConwayState::Dead,
        }
    }
}
