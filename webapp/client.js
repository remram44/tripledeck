const client = import("./dist/tripledeck_wasm");

window.storage_get_board = function(id) {
    return {
        id: id,
        name: "Board " + id,
        lists: []
    };
}

client.then(client => {
    client.test("World!");
    var b = client.get_board("936DA01F9ABD4d9d80C702AF85C822A8");
    console.log("board = ", b);
});
