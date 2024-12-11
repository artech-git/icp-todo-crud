use ic_cdk::init;
use ic_stable_structures::btreemap::BTreeMap;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::DefaultMemoryImpl;
use std::cell::RefCell;
use todo_type::{TodoText, TodoUnit, UID};

pub mod app;
pub mod routes;
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
            MEMORY_MANAGER.with(|m| m.borrow_mut().get(MemoryId::new(0)))
        ));

    // static TODO_DATA_PAGE: RefCell<IcStableVec<(u64, UID), Memory>> = RefCell::new(IcStableVec::init(
    //     MEMORY_MANAGER.with(|m| m.borrow_mut().get(MemoryId::new(1)))
    // ).unwrap());

    // Hot page cache
    static TODO_DATA_PAGE: RefCell<Vec<(u64, UID)>> = RefCell::new(vec![]);
}

#[init]
fn init() {
    //Prepare a index once we start/reload canisters
    TODO_DATA.with(|todo_data| {
        TODO_DATA_PAGE.with(|todo_data_page| {
            for todo_unit in todo_data.borrow().values() {
                todo_data_page
                    .borrow_mut()
                    .push((todo_unit.start_time, todo_unit.uid.clone()));
            }
            todo_data_page.borrow_mut().sort_by(|a, b| a.0.cmp(&b.0));
            ic_cdk::println!(
                "Data index complete 🚀 for todo_data_page index size: {}",
                todo_data_page.borrow().len()
            );
        })
    });
}

ic_cdk::export_candid!();

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use ic_cdk::api::time;
//     use pocket_ic::PocketIc;

//     #[test]
//     fn test_create_todo() {
//         let mut pocket_ic = PocketIc::new();
//         pocket_ic.execute(|| {
//             let todo_text = "Test todo".to_string();
//             let uid = routes::generate_new_todo(todo_text.clone());
//             assert_eq!(routes::read_todo(uid.clone()), todo_text);
//         });
//     }

//     #[test]
//     fn test_read_pagination() {
//         let todo_text_1 = "Test todo".to_string();
//         let todo_text_2 = "Test todo".to_string();
//         let uid_1 = routes::generate_new_todo(todo_text_1.clone());
//         let todo_unit_1 = TodoUnit {
//             uid: uid_1.clone(),
//             todo_text: todo_text_1.clone(),
//             start_time: time(),
//         };
//         let uid_2 = routes::generate_new_todo(todo_text_2.clone());
//         let todo_unit_2 = TodoUnit {
//             uid: uid_2.clone(),
//             todo_text: todo_text_2.clone(),
//             start_time: time(),
//         };
//         let todo_vec = routes::read_pagination(2, 1);
//         assert_eq!(todo_vec[0].uid, todo_unit_1.uid);
//         assert_eq!(todo_vec[0].todo_text, todo_unit_1.todo_text);
//         assert_eq!(todo_vec[1].uid, todo_unit_2.uid);
//         assert_eq!(todo_vec[1].todo_text, todo_unit_2.todo_text);
//     }

//     #[test]
//     fn test_update_todo() {
//         let todo_text = "Test todo".to_string();
//         let uid = routes::generate_new_todo(todo_text.clone());
//         let new_todo_text = "Updated todo".to_string();
//         assert_eq!(routes::update_todo(uid.clone(), new_todo_text.clone()), true);
//         assert_eq!(routes::read_todo(uid.clone()), new_todo_text);
//     }

//     #[test]
//     fn test_delete_todo() {
//         let todo_text = "Test todo".to_string();
//         let uid = routes::generate_new_todo(todo_text.clone());
//         assert_eq!(routes::delete_todo(uid.clone()), true);
//         assert_eq!(routes::read_todo(uid.clone()), "NIL".to_string());
//     }
// }
