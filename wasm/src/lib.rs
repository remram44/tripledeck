extern crate uuid;
extern crate wasm_bindgen;

extern crate tripledeck_core;

use uuid::Uuid;
use wasm_bindgen::prelude::*;

use tripledeck_core::{Board, List, Storage};

#[wasm_bindgen]
pub struct BoardWrap(tripledeck_core::Board);

fn uuid2str(id: &Uuid) -> String {
    format!("{:x}", id.to_hyphenated_ref())
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn test(name: &str) {
    alert(&format!("Hello, {}", name));
}

// Storage functions provided by JavaScript
#[wasm_bindgen]
extern {
    pub fn storage_add_board(board: &JsValue);
    pub fn storage_get_board(id: &str) -> JsValue;
    pub fn storage_add_list(board_id: &str, list: &JsValue);
}

/// Adapter for Storage trait using JavaScript code.
struct JsStorage;

impl Storage for JsStorage {
    fn add_board(&mut self, board: &Board) {
        storage_add_board(
            &JsValue::from_serde(board).unwrap(),
        );
    }

    fn get_board(&mut self, id: &Uuid) -> Board {
        storage_get_board(
            &uuid2str(id),
        ).into_serde().unwrap()
    }

    fn add_list(&mut self, board_id: &Uuid, list: &List) {
        storage_add_list(
            &uuid2str(board_id),
            &JsValue::from_serde(list).unwrap(),
        );
    }
}

#[wasm_bindgen]
pub fn get_board(id: &str) -> BoardWrap {
    BoardWrap(Board::get(&mut JsStorage, &Uuid::parse_str(id).expect("Invalid board ID")))
}
