use candid::Nat;
use ic_cdk::{println, query, update};
use todo_type::{TodoText, TodoUnit, UID};
// use std::borrow::Borrow;
use ic_stable_structures::btreemap::BTreeMap;
use ic_stable_structures::DefaultMemoryImpl;
use std::cell::RefCell;

pub mod app;
pub mod todo_type;
pub mod utils;


//TODO: work on ensuring memory metrics doesn't get overfilled   

// declare the storage region for todo in canister
thread_local! {
    static TODO_DATA: RefCell<BTreeMap<UID, TodoUnit, DefaultMemoryImpl>> =
        RefCell::new(BTreeMap::init(DefaultMemoryImpl::default()));

    static TODO_DATA_PAGE: RefCell<Nat> = RefCell::new(Nat::from(0_u32));
    // static TODO_PAGINATION: RefCell<Iter<'_, UID, TodoUnit, DefaultMemoryImpl>> = RefCell::new(TODO_DATA.with(|todo_data| todo_data.borrow().iter()));
}

/// Router for submitting a todo information.
#[update]
fn generate_new_todo(todo_text: TodoText) -> UID {
    //TODO: apply data size assertion checks: Result should be used somehow

    TODO_DATA.with(|todo_data| {
        // generate a uid of 32 bit sized
        let uid_string = utils::generate_random_string(32);

        let todo_unit = TodoUnit {
            uid: uid_string.clone(),
            todo_text: todo_text,
            start_time: ic_cdk::api::time(),
        };

        let _val = todo_data.borrow_mut().insert(uid_string.clone(), todo_unit);

        uid_string
    })
}

#[query]
fn read_todo(uid: UID) -> TodoText {
    if uid.chars().count() != 32 {
        println!("Todo not of desired length: {}", uid.chars().count());
        return "NIL".to_string();
    }

    TODO_DATA.with(|todo_data| {
        if let Some(val) = todo_data.borrow().get(&uid) {
            return val.todo_text.clone();
        }
        println!("Todo not found for uid: {}", uid);
        "NIL".to_string()
    })
}

#[query]
fn read_pagination(page_size: usize, count: usize) -> Vec<TodoUnit> {
    let start = page_size.saturating_mul(count.saturating_sub(1));
    let end = page_size.saturating_mul(count);

    TODO_DATA.with(|todo_data| {
        let mut todo_vec = vec![];
        for (idx, todo_unit) in todo_data
            .borrow()
            .values() // assuming the iterator behaviour to be deterministic each time we invoke this function,
            .enumerate()
        {
            // also making a strong assumption regarinding insertion of values towards end of the map
            if idx < start {
                continue;
            }
            if idx >= end {
                break;
            }
            todo_vec.push(todo_unit.clone());
        }
        todo_vec
    })
}

#[update]
fn update_todo(uid: UID, todo: TodoText) -> bool {
    //TODO: condition checks for todo_texts
    TODO_DATA.with(|todo_data| {
        let new_todo_unit = TodoUnit {
            uid: uid.clone(),
            todo_text: todo,
            start_time: ic_cdk::api::time(),
        };
        // If key and value already exists, it will return false otherwise true if not found already
        !todo_data.borrow_mut().insert(uid, new_todo_unit).is_some()
    })
}

#[update]
fn delete_todo(uid: UID) -> bool {
    TODO_DATA.with(|todo_data| todo_data.borrow_mut().remove(&uid).is_some())
}

ic_cdk::export_candid!();
