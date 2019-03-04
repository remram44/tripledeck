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
pub struct Card {
    pub id: Uuid,
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct List {
    pub id: Uuid,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Board {
    pub id: Uuid,
    pub name: String,
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

pub struct BoardHandle<S: Storage> {
    storage: Rc<S>,
    inner: Rc<RefCell<Board>>,
    lists: Rc<RefCell<Vec<List>>>,
}

impl<S: Storage> BoardHandle<S> {
    pub fn board<'a>(&'a self) -> std::cell::Ref<'a, Board> {
        self.inner.borrow()
    }

    pub fn lists<'a>(&'a self) -> std::cell::Ref<'a, Vec<List>> {
        self.lists.borrow()
    }

    pub fn add_list(&self, name: &str)
        -> Box<Future<Item=(), Error=S::Error>>
    {
        let list = List {
            id: Uuid::new_v4(),
            name: name.into(),
        };
        self.storage.add_list(&self.board().id, &list)
    }
}

pub struct App<S: Storage + 'static> {
    storage: Rc<S>,
    boards: Rc<RefCell<BTreeMap<Uuid, Weak<BoardHandle<S>>>>>,
}

impl<S: Storage> App<S> {
    pub fn new(storage: S) -> App<S> {
        App {
            storage: Rc::new(storage),
            boards: Rc::new(RefCell::new(BTreeMap::new())),
        }
    }

    pub fn new_board(&self, name: &str)
        -> Box<Future<Item=Rc<BoardHandle<S>>, Error=S::Error>>
    {
        // Make it
        let id = Uuid::new_v4();
        let inner = Board {
            id: id.clone(),
            name: name.into(),
        };

        // Add it to storage
        let fut = self.storage.add_board(&inner);

        // Wrap it
        let board = BoardHandle {
            storage: self.storage.clone(),
            inner: Rc::new(RefCell::new(inner)),
            lists: Rc::new(RefCell::new(Vec::new())),
        };
        let rc = Rc::new(board);

        // Add it to the cache
        self.boards.borrow_mut().insert(id, Rc::downgrade(&rc));

        let fut = fut.map(move |()| rc);
        Box::new(fut)
    }

    pub fn get_board(&self, id: &Uuid)
        -> Box<Future<Item=Option<Rc<BoardHandle<S>>>, Error=S::Error>>
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
        // Wrap it
        let storage = self.storage.clone();
        let fut = fut.map(|opt| opt.map(|b| {
            let board = BoardHandle {
                storage: storage,
                inner: Rc::new(RefCell::new(b)),
                lists: Rc::new(RefCell::new(Vec::new())),
            };
            Rc::new(board)
        }));
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

    pub fn add_list(&self, board: Rc<Board>, name: &str)
        -> Box<Future<Item=(), Error=S::Error>>
    {
        let list = List {
            id: Uuid::new_v4(),
            name: name.into(),
        };
        let fut = self.storage.add_list(
            &board.id,
            &list,
        );
        Box::new(fut)
    }
}
