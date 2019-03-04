const DB_NAME = "tripledeck";
const DB_VERSION = 1;
var db = null;

var request = window.indexedDB.open(DB_NAME, DB_VERSION);

request.onerror = function(event) {
    alert("Couldn't access indexed storage");
}

request.onblocked = function(event) {
    alert("Please close all other tabs of this site to allow the database " +
          "to upgrade");
};

request.onupgradeneeded = function(event) {
    console.log("Database upgrade...");
    var db = event.target.result;

    db.onversionchange = function(event) {
        db.close();
        alert("Database upgraded, please reload or close this tab");
    }

    var boards = db.createObjectStore("boards", {keyPath: "id"});
    boards.createIndex("name", "name", {unique: false});

    var lists = db.createObjectStore("lists", {keyPath: "id"});
    lists.createIndex("board", "board", {unique: false});

    var cards = db.createObjectStore("cards", {keyPath: "id"});
    cards.createIndex("board", "board", {unique: false});
    cards.createIndex("list", "list", {unique: false});

    cards.transaction.oncomplete = function() {
        var tran = db.transaction(["boards", "lists", "cards"],
                                  "readwrite");

        var boards = tran.objectStore("boards");
        boards.add({id: "936DA01F9ABD4D9D80C70000BBBB0000", name: "board"});

        var lists = tran.objectStore("lists");
        lists.add({id: "936DA01F9ABD4D9D80C7000011110001", name: "todo",
                  board: "936DA01F9ABD4D9D80C70000BBBB0000"});
        lists.add({id: "936DA01F9ABD4D9D80C7000011110002", name: "doing",
                  board: "936DA01F9ABD4D9D80C70000BBBB0000"});
        lists.add({id: "936DA01F9ABD4D9D80C7000011110003", name: "done",
                  board: "936DA01F9ABD4D9D80C70000BBBB0000"});

        var cards = tran.objectStore("cards");
        cards.add({id: "936DA01F9ABD4D9D80C70000CCCC0001", title: "design",
                   board: "936DA01F9ABD4D9D80C70000BBBB0000",
                   list: "936DA01F9ABD4D9D80C7000011110003"});
        cards.add({id: "936DA01F9ABD4D9D80C70000CCCC0002", title: "implement",
                   board: "936DA01F9ABD4D9D80C70000BBBB0000",
                  list: "936DA01F9ABD4D9D80C7000011110002"});
        cards.add({id: "936DA01F9ABD4D9D80C70000CCCC0003", title: "test",
                   board: "936DA01F9ABD4D9D80C70000BBBB0000",
                   list: "936DA01F9ABD4D9D80C7000011110001"});
        cards.add({id: "936DA01F9ABD4D9D80C70000CCCC0004", title: "document",
                   board: "936DA01F9ABD4D9D80C70000BBBB0000",
                   list: "936DA01F9ABD4D9D80C7000011110001"});

        console.log("Database upgrade complete");
    };
};

request.onsuccess = function(event) {
    console.log("Database opened");
    db = event.target.result;
    window.tripledeck_db = db;
};

window.storage_get_board = function(id) {
    console.log("Storage: get_board(", id, ")");
    return new Promise(function(resolve, reject) {
        var tran = db.transaction(["boards", "lists"]);

        var req = tran.objectStore("boards").get(id);
        req.onerror = function(event) { reject(event.target.errorCode); };
        req.onsuccess = function() {
            var board = req.result;
            console.log("Storage: got board:", board);
            if(board == undefined) {
                resolve(null);
            } else {
                resolve({
                    id: id,
                    name: board.name
                });
            }
        };
    });
};

window.storage_add_board = function(board) {
    console.log("Storage: add_board(", board.id, ")");
    return new Promise(function(resolve, reject) {
        var tran = db.transaction(["boards", "lists"], "readwrite");
        tran.onerror = function(event) { reject(tran.error); };

        // Add the board
        var req_b = tran.objectStore("boards").put({
            id: board.id,
            name: board.name
        });
        req_b.onerror = function(event) { reject(event.target.errorCode); };
        req_b.onsuccess = function() {
            for(var i = 0; i < board.lists.length; ++i) {
                var list = board.lists[i];
                // Add the lists
                tran.objectStore("lists", "readwrite").put({
                    id: list.id,
                    name: list.name,
                    board: board.id
                });
            }
            tran.oncomplete = function() { resolve(); };
        };
    });
};

window.storage_get_lists = function(board_id) {
    console.log("Storage: get_lists(", board_id, ")");
    return new Promise(function(resolve, reject) {
        var lists = [];
        var tran = db.transaction(["lists"]);
        var req = tran.objectStore("lists").index("board").openCursor(IDBKeyRange.only(board_id));
        req.onerror = function(event) { console.log("no");reject(event.target.errorCode); };
        req.onsuccess = function(event) {
            var cursor = event.target.result;
            if(cursor) {
                lists.push(cursor.value);
                cursor.continue();
            } else {
                console.log("Storage: got lists:", lists);
                resolve(lists);
            }
        };
    });
}

window.storage_add_list = function(board_id, list) {
    console.log("Storage: add_list(", board_id, ", ", list.id, ")");
    return new Promise(function(resolve, reject) {
        var tran = db.transaction(["lists"], "readwrite");
        tran.onerror = function(event) { reject(tran.error); };

        var req = tran.objectStore("lists").put({
            id: list.id,
            name: list.name,
            board: board_id
        });
        tran.oncomplete = function() { resolve(); };
    });
};
