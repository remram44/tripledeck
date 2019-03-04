CREATE TABLE boards(id TEXT PRIMARY KEY, name TEXT);
CREATE TABLE lists(id TEXT PRIMARY KEY, board_id TEXT, name TEXT);
CREATE TABLE cards(id TEXT PRIMARY KEY, board_id TEXT, list_id TEXT, title TEXT);

INSERT INTO boards(id, name) VALUES('936DA01F9ABD4D9D80C70000BBBB0000', 'board');

INSERT INTO lists(id, name, board_id) VALUES('936DA01F9ABD4D9D80C7000011110001', 'todo', '936DA01F9ABD4D9D80C70000BBBB0000');
INSERT INTO lists(id, name, board_id) VALUES('936DA01F9ABD4D9D80C7000011110002', 'doing', '936DA01F9ABD4D9D80C70000BBBB0000');
INSERT INTO lists(id, name, board_id) VALUES('936DA01F9ABD4D9D80C7000011110003', 'done', '936DA01F9ABD4D9D80C70000BBBB0000');

INSERT INTO cards(id, title, board_id, list_id) VALUES('936DA01F9ABD4D9D80C70000CCCC0001', 'design', '936DA01F9ABD4D9D80C70000BBBB0000', '936DA01F9ABD4D9D80C7000011110003');
INSERT INTO cards(id, title, board_id, list_id) VALUES('936DA01F9ABD4D9D80C70000CCCC0002', 'implement', '936DA01F9ABD4D9D80C70000BBBB0000', '936DA01F9ABD4D9D80C7000011110002');
INSERT INTO cards(id, title, board_id, list_id) VALUES('936DA01F9ABD4D9D80C70000CCCC0003', 'test', '936DA01F9ABD4D9D80C70000BBBB0000', '936DA01F9ABD4D9D80C7000011110001');
INSERT INTO cards(id, title, board_id, list_id) VALUES('936DA01F9ABD4D9D80C70000CCCC0004', 'document', '936DA01F9ABD4D9D80C70000BBBB0000', '936DA01F9ABD4D9D80C7000011110001');
