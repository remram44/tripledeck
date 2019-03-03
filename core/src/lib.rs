extern crate futures;
extern crate serde;
extern crate uuid;

use futures::Future;
use serde::{Serialize, Deserialize};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::{Rc, Weak};
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

    fn add_board(&self, board: &Board)
        -> Box<Future<Item=(), Error=Self::Error>>;
    fn get_board(&self, id: &Uuid)
        -> Box<Future<Item=Option<Board>, Error=Self::Error>>;
    fn add_list(&self, board_id: &Uuid, list: &List)
        -> Box<Future<Item=(), Error=Self::Error>>;
}

impl Board {
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

pub struct App<S: Storage> {
    storage: S,
    boards: Rc<RefCell<BTreeMap<Uuid, Weak<Board>>>>,
}

impl<S: Storage> App<S> {
    pub fn new(storage: S) -> App<S> {
        App {
            storage,
            boards: Rc::new(RefCell::new(BTreeMap::new())),
        }
    }

    pub fn new_board(&self, name: &str)
        -> Box<Future<Item=Rc<Board>, Error=S::Error>>
    {
        // Make it
        let id = Uuid::new_v4();
        let board = Board {
            id: id.clone(),
            name: name.into(),
            lists: Vec::new(),
        };
        let rc = Rc::new(board);

        // Add it to the cache
        self.boards.borrow_mut().insert(id, Rc::downgrade(&rc));

        // Add it to storage
        let fut = self.storage.add_board(&*rc);

        let fut = fut.map(|()| rc);
        Box::new(fut)
    }

    pub fn get_board(&self, id: &Uuid)
        -> Box<Future<Item=Option<Rc<Board>>, Error=S::Error>>
    {
        // Get from cache
        let opt = self.boards.borrow().get(id).cloned();
        if let Some(weak) = opt {
            if let Some(rc) = weak.upgrade() {
                return Box::new(futures::future::ok(Some(rc)));
            }
        }

        // Get it from storage
        let fut = self.storage.get_board(id);
        // Turn into Rc
        let fut = fut.map(|opt| opt.map(|b| Rc::new(b)));
        // Add it to the cache
        let id = id.clone();
        let boards_map = self.boards.clone();
        let fut = fut.map(move |opt| {
            if let Some(ref rc) = opt {
                boards_map.borrow_mut().insert(
                    id,
                    Rc::downgrade(rc),
                );
            }
            opt
        });
        Box::new(fut)
    }
}
