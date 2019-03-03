import database from "./database.js"

const client = import("./dist/tripledeck_wasm");

client.then(client => {
    client.get_board("936DA01F9ABD4d9d80C70000BBBB0000")
    .then(function(b) {
        console.log("board = ", b);
    });
});
