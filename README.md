# Invoicer

A work-in-progress invoicing application with a Rust (Axum + SQLx) backend and a React frontend.

This project is being built as a learning-focused but production-minded exercise, with an emphasis on clean API design, explicit data models, and a clear separation between backend and frontend concerns.

---

## Project Structure

invoicer/
├── invoicer-backend/ # Rust backend (Axum + SQLite)
└── invoicer-frontend/ # React frontend (Vite + TypeScript)

---

## Backend

### Tech Stack

- Rust
- Axum (HTTP server and routing)
- SQLx (database access)
- SQLite (local development database)
- Tokio (async runtime)

### Features (so far)

- CRUD endpoints for:
  - Clients
  - Invoices
- Server-side joins (e.g. invoices returned with client names)
- Query parameters for sorting invoice lists
- Parameterized SQL queries (no raw SQL from clients)
- Basic CORS configuration for local development

### Running the backend

cd invoicer-backend
cargo run

The server runs at:

http://127.0.0.1:3000

Make sure DATABASE_URL is set, for example:

export DATABASE_URL=sqlite:data/invoice.db

Then run migrations:

sqlx migrate run

---

## Frontend

### Tech Stack

- React
- TypeScript
- Vite
- pnpm

### Running the frontend

cd invoicer-frontend
pnpm install
pnpm dev

The frontend runs at:

http://localhost:5173

It communicates with the backend via HTTP (REST API).

---

## API Design Notes

- The frontend never accesses the database directly
- All database access is mediated through the backend
- The backend returns data shaped for the UI (e.g. invoices include client names)
- SQL queries are fully controlled server-side
- User authentication and authorization are not yet implemented

---

## Roadmap (high level)

- [ ] Invoice line items
- [ ] Totals and calculations
- [ ] Basic authentication (sessions or tokens)
- [ ] Per-user data isolation
- [ ] UI for managing clients and invoices
- [ ] Pagination and filtering
- [ ] Production-ready CORS and security settings

---

## Status

This project is under active development.
The API and data models may change as features are added and refined.

---

## License

MIT
