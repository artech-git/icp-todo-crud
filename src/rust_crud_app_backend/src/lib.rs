use ic_cdk::{println, query, update};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use todo_type::{TodoText, TodoUnit, UID};
// use std::borrow::Borrow;
use ic_stable_structures::btreemap::BTreeMap;
use ic_stable_structures::{DefaultMemoryImpl, MinHeap};
use std::cell::RefCell;

pub mod app;
pub mod todo_type;
pub mod utils;

//TODO: work on ensuring memory metrics doesn't get overfilled
type Memory = VirtualMemory<DefaultMemoryImpl>;
// declare the storage region for todo in canister
thread_local! {

    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static TODO_DATA: RefCell<BTreeMap<UID, TodoUnit, Memory>> =
        RefCell::new(BTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))
        ));

    static TODO_DATA_PAGE: RefCell<MinHeap<(u64, UID), Memory>> = RefCell::new(MinHeap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ).unwrap());
}

/// Router for submitting a todo information.
#[update]
fn generate_new_todo(todo_text: TodoText) -> UID {
    //TODO: apply data size assertion checks: Result should be used somehow

    // generate a uid of 32 bit sized
    let uid_string = utils::generate_random_string(32);
    let curr_time = ic_cdk::api::time();

    TODO_DATA_PAGE.with(|todo_data_page| {
        let _val = todo_data_page
            .borrow_mut()
            .push(&(curr_time, uid_string.clone()));
    });

    TODO_DATA.with(|todo_data| {
        let todo_unit = TodoUnit {
            uid: uid_string.clone(),
            todo_text: todo_text,
            start_time: curr_time,
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

    TODO_DATA_PAGE.with(|todo_data| {
        let mut todo_vec = vec![];

        for (idx, todo_id) in todo_data.borrow().iter().skip(start).take(end)
        // .values() // assuming the iterator behaviour to be deterministic each time we invoke this function,
        {
            TODO_DATA.with(|todo_data| {
                if let Some(todo_unit) = todo_data.borrow().get(&todo_id) {
                    todo_vec.push(todo_unit.clone());
                }
            });
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
