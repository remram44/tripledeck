Tripledeck
==========

Goal
----

This is an open-source planning application based on the common Kanban approach.

The three main goals are:

* **Automation**: you can add filters and actions, allowing for things to happen automatically (e.g. things assigned to me on this board should be added to that other board, issue close on GitHub means card should move to that list, ...) and problems to be reported (e.g. cards in the "in progress" column with no one assigned, cards in the "done" column but the issue is still open, ...). This is useful for complex workflows, but also for uses beyond project management.
* **Syncing** between devices: this app works offline, on both your laptop and phone, and should properly sync once you connect to a server. It can also work fine without a server (i.e. full client-side functionality). The server itself is a single binary that can be easily deployed anywhere.
* **Integration with services**: your GitHub/GitLab issues and TODOs, RSS feeds, Tweets, Reading lists, ... should be able to sync with this. You shouldn't need to manually check those other places to create cards and organize your work.

Design
------

To achieve this vision, I chose to implement the core functionality (cards behavior, filters and actions, syncing) in the Rust programming language, and compile it for the web using WebAssembly (the server and command-line client are native Rust binaries).

Each deployment has multiple parts:

* The core (native or WebAssembly)
* A frontend (web or command-line; none for the server)
* A storage mechanism (SQL for native, IndexedDB for web)
* A communication mechanism (websocket)

Repository organization
-----------------------

* [core](core/): Core functionality, used by the client, server, and webapp
* [program](program/): Native program, optionally with socket server. Uses core, SQL backend.
* [webapp](webapp/): Progressive web app. Uses core as webassembly, IndexedDB backend.
