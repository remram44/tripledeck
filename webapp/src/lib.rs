extern crate futures;
extern crate js_sys;
extern crate uuid;
extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;

extern crate tripledeck_core;

use futures::Future;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{JsFuture, future_to_promise};

use tripledeck_core::{Board, List, Storage};

#[wasm_bindgen]
pub struct BoardWrap(tripledeck_core::Board);

fn uuid2str(id: &Uuid) -> String {
    //format!("{:x}", id.to_hyphenated_ref())
    format!("{:X}", id.to_simple_ref())
}

// Storage functions provided by JavaScript
#[wasm_bindgen]
extern {
    pub fn storage_add_board(board: &JsValue) -> js_sys::Promise;
    pub fn storage_get_board(id: &str) -> js_sys::Promise;
    pub fn storage_add_list(board_id: &str, list: &JsValue) -> js_sys::Promise;
}

/// Adapter for Storage trait using JavaScript code.
struct JsStorage;

impl Storage for JsStorage {
    type Error = JsValue;

    fn add_board(&mut self, board: &Board)
        -> Box<Future<Item=(), Error=Self::Error>>
    {
        Box::new(JsFuture::from(storage_add_board(
            &JsValue::from_serde(board).unwrap(),
        )).map(|_| ()))
    }

    fn get_board(&mut self, id: &Uuid)
        -> Box<Future<Item=Option<Board>, Error=Self::Error>>
    {
        Box::new(JsFuture::from(storage_get_board(
            &uuid2str(id),
        )).map(|value| {
            if value == JsValue::NULL {
                None
            } else {
                value.into_serde().unwrap()
            }
        }))
    }

    fn add_list(&mut self, board_id: &Uuid, list: &List)
        -> Box<Future<Item=(), Error=Self::Error>>
    {
        Box::new(JsFuture::from(storage_add_list(
            &uuid2str(board_id),
            &JsValue::from_serde(list).unwrap(),
        )).map(|_| ()))
    }
}

#[wasm_bindgen]
pub fn get_board(id: &str) -> js_sys::Promise {
    // Convert str to Uuid
    let id = Uuid::parse_str(id).expect("Invalid board ID");
    // Get board from storage
    let fut = Board::get(&mut JsStorage, &id)
    // Convert Board to BoardWrap
        .map(
        |option: Option<Board>| option.map(
            |b| BoardWrap(b)
        )
        )
    // Convert to JsValue
        .map(JsValue::from);
    // Convert Future<JsValue> to Promise
    future_to_promise(fut)
}
