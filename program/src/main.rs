extern crate clap;
extern crate futures;
extern crate rusqlite;
extern crate tripledeck_core;
extern crate uuid;

use clap::{App, Arg};
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
        let sql_connection = Connection::open(path.as_ref())?;
        if !path.as_ref().exists() {
            sql_connection.execute(
                "
                CREATE TABLE boards(id TEXT PRIMARY KEY, name TEXT);
                CREATE TABLE lists(id TEXT PRIMARY KEY, board_id TEXT, name TEXT);
                CREATE TABLE cards(id TEXT PRIMARY KEY, board_id TEXT, list_id TEXT, title TEXT);
                ",
                rusqlite::NO_PARAMS,
            );
        }
        Ok(SqliteStorage {
            sql_connection,
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
            &[&uuid2str(id) as &ToSql],
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
                |iter| iter.map(Result::unwrap).collect()
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
    let mut cli = App::new("tripledeck")
        .bin_name("tripledeck")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("db")
             .help("Path to database")
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("board")
             .help("Board ID")
             .required(false)
             .takes_value(true));
    let matches = match cli.get_matches_from_safe_borrow(std::env::args_os()) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(2);
        }
    };
    let db = matches.value_of_os("db")
        .expect("No value for db");

    let storage = SqliteStorage::new(db).expect("Can't open database");
    let app = tripledeck_core::App::new(storage);

    if let Some(board_id) = matches.value_of("board") {
        let fut = app.get_board(&Uuid::parse_str(board_id)
                                  .expect("Invalid UUID"));
        let fut = fut.map(|opt| {
            match opt {
                None => println!("No such board"),
                Some(board) => {
                    println!("Board: {}", board.board().name);
                    println!("TODO: print lists, cards");
                }
            }
        });
        futures::executor::spawn(fut).wait_future().unwrap();
    } else {
        println!("TODO: print boards");
    }
}
