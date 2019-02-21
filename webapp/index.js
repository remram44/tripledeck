const js = import("./tripledeck_wasm");

js.then(js => {
    js.test("World!");
});
