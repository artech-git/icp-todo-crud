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

impl Storable for TodoUnit {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        // std::borrow::Cow::Owned(bincode::serialize(self).expect("Failed to serialize TodoUnit"))
        let encoded_bytes = Encode!(self).unwrap();
        std::borrow::Cow::Owned(encoded_bytes)
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        // bincode::deserialize(&bytes).expect("Failed to deserialize TodoUnit")
        Decode!(&bytes, TodoUnit).unwrap()
    }
}

// ic_cdk::export_candid!();
