extern crate futures;
extern crate rusqlite;
extern crate tripledeck_core;
extern crate uuid;

use futures::{Future, future};
use rusqlite::Connection;
use rusqlite::types::ToSql;
use std::path::Path;
use uuid::Uuid;

use tripledeck_core::{List, Board, BoardHandle, Storage};

fn uuid2str(id: &Uuid) -> String {
    format!("{:X}", id.to_simple_ref())
}

struct SqliteStorage {
    sql_connection: Connection,
}

impl SqliteStorage {
    fn new<P: AsRef<Path>>(path: P) -> rusqlite::Result<SqliteStorage> {
        Ok(SqliteStorage {
            sql_connection: Connection::open(path)?,
        })
    }
}

impl Storage for SqliteStorage {
    type Error = rusqlite::Error;

    fn add_board(&self, board: &Board)
        -> Box<Future<Item=(), Error=Self::Error>>
    {
        let res = self.sql_connection.execute(
            "INSERT INTO boards(id, name) VALUES(?, ?);",
            &[&uuid2str(&board.id) as &ToSql, &board.name as &ToSql],
        );
        Box::new(future::result(res.map(|_| ())))
    }

    fn get_board(&self, id: &Uuid)
        -> Box<Future<Item=Option<Board>, Error=Self::Error>>
    {
        let res = self.sql_connection.query_row(
            "SELECT id, name FROM boards WHERE id=?;",
            &[&uuid2str(id)],
            |row| {
                let id: String = row.get(0);
                Board {
                    id: Uuid::parse_str(&id).unwrap(),
                    name: row.get(1),
                }
            },
        );
        let res = match res {
            Ok(b) => Ok(Some(b)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        };
        Box::new(future::result(res))
    }

    fn get_lists(&self, board_id: &Uuid)
        -> Box<Future<Item=Vec<List>, Error=Self::Error>>
    {
        let res = self.sql_connection.prepare(
            "SELECT id, name FROM lists WHERE board_id=?;",
        );
        let res = res.and_then(|mut stmt| {
            stmt.query_map(
                &[&uuid2str(board_id)],
                |row| {
                    let id: String = row.get(0);
                    List {
                        id: Uuid::parse_str(&id).unwrap(),
                        name: row.get(1),
                    }
                },
            ).map(
                |mut iter| iter.map(Result::unwrap).collect()
            )
        });
        Box::new(future::result(res))
    }

    fn add_list(&self, board_id: &Uuid, list: &List)
        -> Box<Future<Item=(), Error=Self::Error>>
    {
        let res = self.sql_connection.execute(
            "INSERT INTO lists(board_id, id, name) VALUES(?, ?, ?);",
            &[&uuid2str(&board_id) as &ToSql, &uuid2str(&list.id) as &ToSql, &list.name as &ToSql],
        );
        Box::new(future::result(res.map(|_| ())))
    }
}

fn main() {
    println!("Hello, world!");
}
