extern crate futures;
extern crate js_sys;
extern crate uuid;
extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;

extern crate tripledeck_core;

use futures::Future;
use std::rc::Rc;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{JsFuture, future_to_promise};

use tripledeck_core::{List, Board, BoardHandle, Storage};

#[wasm_bindgen]
pub struct BoardWrap(Rc<tripledeck_core::BoardHandle<JsStorage>>);

fn uuid2str(id: &Uuid) -> String {
    format!("{:X}", id.to_simple_ref())
}

thread_local! {
    static APP: tripledeck_core::App<JsStorage> =
        tripledeck_core::App::new(JsStorage);
}

// Storage functions provided by JavaScript
#[wasm_bindgen]
extern {
    pub fn storage_add_board(board: &JsValue) -> js_sys::Promise;
    pub fn storage_get_board(id: &str) -> js_sys::Promise;
    pub fn storage_get_lists(board_id: &str) -> js_sys::Promise;
    pub fn storage_add_list(board_id: &str, list: &JsValue) -> js_sys::Promise;
}

/// Adapter for Storage trait using JavaScript code.
struct JsStorage;

impl Storage for JsStorage {
    type Error = JsValue;

    fn add_board(&self, board: &Board)
        -> Box<Future<Item=(), Error=Self::Error>>
    {
        Box::new(JsFuture::from(storage_add_board(
            &JsValue::from_serde(board).unwrap(),
        )).map(|_| ()))
    }

    fn get_board(&self, id: &Uuid)
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

    fn get_lists(&self, board_id: &Uuid)
        -> Box<Future<Item=Vec<List>, Error=Self::Error>>
    {
        Box::new(JsFuture::from(storage_get_lists(
            &uuid2str(board_id)
        )).map(|array| {
            array.into_serde().unwrap()
        }))
    }

    fn add_list(&self, board_id: &Uuid, list: &List)
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
    let fut = APP.with(|app_| app_.get_board(&id))
    // Convert Board to BoardWrap
        .map(
        |option: Option<Rc<BoardHandle<JsStorage>>>| option.map(
            |b| BoardWrap(b)
        )
        )
    // Convert to JsValue
        .map(JsValue::from);
    // Convert Future<JsValue> to Promise
    future_to_promise(fut)
}
