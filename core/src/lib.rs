extern crate futures;
extern crate serde;
extern crate uuid;

use futures::Future;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct List {
    pub id: Uuid,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Board {
    pub id: Uuid,
    pub name: String,
    pub lists: Vec<List>,
}

pub trait Storage {
    type Error: 'static;

    fn add_board(&mut self, board: &Board)
        -> Box<Future<Item=(), Error=Self::Error>>;
    fn get_board(&mut self, id: &Uuid)
        -> Box<Future<Item=Option<Board>, Error=Self::Error>>;
    fn add_list(&mut self, board_id: &Uuid, list: &List)
        -> Box<Future<Item=(), Error=Self::Error>>;
}

impl Board {
    pub fn new<S: Storage>(storage: &mut S, name: &str)
        -> Box<Future<Item=Board, Error=S::Error>>
    {
        let board = Board {
            id: Uuid::new_v4(),
            name: name.into(),
            lists: Vec::new(),
        };
        Box::new(storage.add_board(&board).map(|()| board))
    }

    pub fn get<S: Storage>(storage: &mut S, id: &Uuid)
        -> Box<Future<Item=Option<Board>, Error=S::Error>>
    {
        storage.get_board(id)
    }

    pub fn add_list<S: Storage>(&mut self, storage: &mut S, name: &str)
        -> Box<Future<Item=(), Error=S::Error>>
    {
        let list = List {
            id: Uuid::new_v4(),
            name: name.into(),
        };
        storage.add_list(&self.id, &list)
    }
}
