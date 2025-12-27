-- Add migration script here
CREATE TABLE invoices (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    client_id INTEGER NOT NULL,
    status TEXT NOT NULL,
    issued_at TEXT NOT NULL,
    due_at TEXT,

    FOREIGN KEY (client_id) REFERENCES clients(id)
);