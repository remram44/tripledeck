extern crate serde;
extern crate uuid;

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
    fn add_board(&mut self, board: &Board);
    fn get_board(&mut self, id: &Uuid) -> Option<Board>;
    fn add_list(&mut self, board_id: &Uuid, list: &List);
}

impl Board {
    pub fn new<S: Storage>(storage: &mut S, name: &str) -> Board {
        let board = Board {
            id: Uuid::new_v4(),
            name: name.into(),
            lists: Vec::new(),
        };
        storage.add_board(&board);
        board
    }

    pub fn get<S: Storage>(storage: &mut S, id: &Uuid) -> Option<Board> {
        storage.get_board(id)
    }

    pub fn add_list<S: Storage>(&mut self, storage: &mut S, name: &str) {
        let list = List {
            id: Uuid::new_v4(),
            name: name.into(),
        };
        storage.add_list(&self.id, &list);
    }
}
