extern crate uuid;
extern crate wasm_bindgen;
extern crate web_sys;

extern crate tripledeck_core;

use uuid::Uuid;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

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

/// Adapter for Storage trait using JavaScript code.
struct JsStorage;

impl Storage for JsStorage {
    fn add_board(&mut self, board: &Board) {
        unimplemented!()/*
        storage_add_board(
            &JsValue::from_serde(board).unwrap(),
        );*/
    }

    fn get_board(&mut self, id: &Uuid) -> Option<Board> {
        unimplemented!()/*
        let value = storage_get_board(
            &uuid2str(id),
        );
        if value == JsValue::NULL {
            None
        } else {
            value.into_serde().unwrap()
        }*/
    }

    fn add_list(&mut self, board_id: &Uuid, list: &List) {
        unimplemented!()/*
        storage_add_list(
            &uuid2str(board_id),
            &JsValue::from_serde(list).unwrap(),
        );*/
    }
}

#[wasm_bindgen]
pub fn get_board(id: &str) -> Option<BoardWrap> {
    let id = Uuid::parse_str(id).expect("Invalid board ID");
    Board::get(&mut JsStorage, &id).map(|b| BoardWrap(b))
}

#[wasm_bindgen]
pub struct App {
    storage: JsStorage,
}

#[wasm_bindgen]
pub fn init() -> Result<App, JsValue> {
    let factory = web_sys::window().unwrap().indexed_db()?;
    let factory = if let Some(f) = factory {
        f
    } else {
        return Err(JsValue::from("Unable to initialize IndexedDB"));
    };
    let mut request = factory.open_with_u32("tripledeck", 1)?;
    let c = Closure::wrap(Box::new(move || {}) as Box<dyn Fn()>);
    request.set_onerror(Some(c.as_ref().unchecked_ref()));
    unimplemented!()
}
