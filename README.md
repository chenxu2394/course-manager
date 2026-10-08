# Course Manager

A small server-rendered course management application built with Rust and
[Rocket](https://rocket.rs/). It supports creating, viewing, editing, and
deleting courses through HTML forms.

## Highlights

- Rocket routes and form handling
- Shared application state with `Mutex<Vec<Course>>`
- Atomic ID generation with `AtomicU64`
- HTML escaping for user-provided content
- Separate modules for models, routes, state, and rendering
- Route-level tests using Rocket's local HTTP client

## Run locally

Install a current Rust toolchain, then run:

```sh
cargo run
```

Open <http://localhost:8000> in a browser.

## Project structure

```text
src/
├── main.rs             # Rocket setup, seed data, and route-level tests
└── course/
    ├── models.rs       # Course and form data types
    ├── routes.rs       # CRUD request handlers
    ├── states.rs       # Shared application state
    └── utils.rs        # HTML rendering helpers
static/
└── style.css           # Application styles
```

## Data storage

The application stores courses in memory. Data is reset when the server
restarts; there is no database or persistent storage layer.

## Testing

The test suite exercises the application through Rocket's local HTTP client,
without starting a network server. Its seven route-level tests cover:

- rendering the seeded courses
- returning `404 Not Found` for a missing course
- creating, updating, and deleting courses
- rejecting invalid form input
- escaping user-provided HTML

Run the tests with:

```sh
cargo test
```

Run the formatting and lint checks with:

```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```
