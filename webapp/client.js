import database from "./database.js"

const client = import("./dist/tripledeck_wasm");

const BOARD_ID = "936DA01F9ABD4D9D80C70000BBBB0000";

client.then(client => {
    client.get_board(BOARD_ID)
    .then((b) => {
        console.log("board = ", b);
    });

    [].forEach.call(document.querySelectorAll(".d3ck-card"), (card) => {
        var card_id = card.id.replace(/card-([0-9A-F]+)/, "$1");
        card.draggable = true;
        card.querySelector("a").draggable = false;
        card.addEventListener("dragstart", (e) => {
            if(e.target.className != "d3ck-card")
                return;
            console.log("Drag start");
            setTimeout(() => { card.style.display = "none"; }, 1);
            e.dataTransfer.setData("text/uri-list", "https://tripledeck.remram.fr/#/" + BOARD_ID + "/" + card_id);
            e.dataTransfer.dropEffect = "move";
        });
        card.addEventListener("dragend", (e) => {
            if(e.target.className != "d3ck-card")
                return;
            console.log("Drag end");
            card.style.display = "";
        });
    });

    [].forEach.call(document.querySelectorAll(".d3ck-list"), (list) => {
        var cards = list.querySelector(".d3ck-list-cards");
        list.addEventListener("dragover", (e) => {
            e.preventDefault();
            e.dataTransfer.dropEffect = "move";
        });
        list.addEventListener("drop", (e) => {
            e.preventDefault();
            var card_id = e.dataTransfer.getData("text/uri-list");
            card_id = /\/([0-9A-F]+)$/.exec(card_id);
            if(card_id)
                card_id = card_id[1];
            else
                return;
            cards.appendChild(document.getElementById("card-" + card_id));
        });
    });
});
