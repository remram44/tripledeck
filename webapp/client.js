const js = import("./dist/tripledeck_wasm");

js.then(js => {
    js.test("World!");
});
