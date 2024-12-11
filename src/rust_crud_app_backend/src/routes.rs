use ic_cdk::{query, update};

use crate::{
    todo_type::{TodoText, TodoUnit, UID},
    TODO_DATA, TODO_DATA_PAGE,
};

/// Router for submitting a todo information.
#[update]
pub fn generate_new_todo(todo_text: TodoText) -> UID {
    //TODO: apply data size assertion checks: Result should be used somehow

    // generate a uid of 32 bit sized
    let uid_string = crate::utils::generate_random_string(32);
    let curr_time = ic_cdk::api::time();

    TODO_DATA_PAGE.with(|todo_data_page| {
        todo_data_page
            .borrow_mut()
            .push((curr_time, uid_string.clone()));
    });

    TODO_DATA.with(|todo_data| {
        let todo_unit = TodoUnit {
            uid: uid_string.clone(),
            todo_text,
            start_time: curr_time,
        };

        let _val = todo_data.borrow_mut().insert(uid_string.clone(), todo_unit);

        uid_string
    })
}

// route for reading a todo
#[query]
pub fn read_todo(uid: UID) -> TodoText {
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

// route for reading a todo
#[query]
pub fn read_complete_todo(uid: UID) -> Result<TodoUnit, String> {
    if uid.chars().count() != 32 {
        println!("Todo not of desired length: {}", uid.chars().count());
        return Err("NIL".to_string());
    }

    TODO_DATA.with(|todo_data| {
        if let Some(val) = todo_data.borrow().get(&uid) {
            return Ok(val.clone());
        }
        println!("Todo not found for uid: {}", uid);
        Err("NIL".to_string())
    })
}

// route for getting multiple todos
#[query]
pub fn read_pagination(page_size: usize, count: usize) -> Vec<TodoUnit> {
    let start = page_size.saturating_mul(count.saturating_sub(1));
    let end = page_size.saturating_mul(count);

    TODO_DATA_PAGE.with(|todo_data| {
        let mut todo_vec = vec![];

        for (_, todo_id) in todo_data.borrow().iter().skip(start).take(end) {
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
pub fn update_todo(uid: UID, todo: TodoText) -> bool {
    //TODO: condition checks for todo_texts
    TODO_DATA.with(|todo_data| {
        let new_todo_unit = TodoUnit {
            uid: uid.clone(),
            todo_text: todo,
            start_time: ic_cdk::api::time(),
        };
        // If key and value already exists
        todo_data.borrow_mut().insert(uid, new_todo_unit).is_some()
    })
}

#[update]
pub fn delete_todo(uid: UID) -> bool {
    TODO_DATA.with(|todo_data| todo_data.borrow_mut().remove(&uid).is_some())
}
