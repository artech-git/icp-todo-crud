use candid::Deserialize;
use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};

// Take uuid to be a str for now
pub type UID = String;

// Todo text alias
pub type TodoText = String;

// fundamental todo task
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct TodoUnit {
    pub uid: UID,
    pub todo_text: TodoText,
    pub start_time: u64, // unix time format
}

const MAX_VALUE_SIZE: u32 = 100;

impl Storable for TodoUnit {
    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        let encoded_bytes = Encode!(self).unwrap_or(vec![]);
        std::borrow::Cow::Owned(encoded_bytes)
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        // Is this naked unwrap safe or not! 🤔
        Decode!(&bytes, TodoUnit).unwrap()
    }
}
