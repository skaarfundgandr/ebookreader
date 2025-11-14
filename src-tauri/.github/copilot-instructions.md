# Stellaron - Ebook Reader Project (Backend)

## Architecture Overview 

**Tauri v2** desktop app with **dual execution contexts** sharing a single SQLite database:

1. **Tauri GUI** - Window management + IPC commands (currently stubs in `commands/`)
2. **Axum REST API** (`api.rs::start()`) - Spawned as tokio background task on `127.0.0.1:3000`

Both share the same `deadpool::Pool<SyncConnectionWrapper<SqliteConnection>>` from `database.rs`. The API spawns in `main()` before the Tauri builder runs, allowing frontend to call either IPC commands or REST endpoints.

### Layer Architecture

```
Frontend (../.)
    ↓ IPC or HTTP
[Controllers] ← DTOs hide sensitive fields (e.g., password_hash)
    ↓
[Services] ← Business logic (mostly stubs)
    ↓
[Repositories] ← Trait-based data access
    ↓
[Database] ← Pooled SQLite + deadpool
```

### Project Structure

```
src/
├── api.rs              # Axum routes: /register, /list_users, /user
├── main.rs             # Entry: spawn API → run Tauri builder
├── lib.rs              # Public module exports
├── commands/           # Tauri IPC (stubs: book_commands, library_commands, metadata_commands)
├── controllers/        # Axum handlers + DTO conversions
│   ├── user_controller.rs  # ✅ Implemented (create, list, get by username)
│   ├── book_controller.rs  # 🚧 Exists but not in API routes
│   └── dto/                # UserDTO, NewUserDTO (sanitize Models)
├── data/
│   ├── database.rs         # connect_from_pool(), lock_db(), PRAGMAS_SET
│   ├── models/             # Diesel entities (Queryable, Insertable, AsChangeset)
│   │   ├── users.rs        # Users, NewUser<'a>, UpdateUser<'a>
│   │   ├── books.rs        # Books, NewBook<'a>, BookForm<'a>
│   │   └── schema.rs       # Auto-generated from migrations
│   ├── repos/
│   │   ├── traits/repository.rs     # Generic Repository trait with GATs
│   │   └── implementors/            # AuthorRepo, BookRepo, UserRepo (8 total)
│   └── migrations/         # Diesel SQL (7 tables: users, books, authors, etc.)
├── handlers/               # File parsers (epub_handler partially done, mobi/pdf stubs)
├── services/               # Business logic (all stubs except AuthenticationService)
└── opds/                   # OPDS feed generation (all stubs)
```

## Developer Workflows

### Environment Setup
Create `.env` file in project root:
```bash
DATABASE_URL=sqlite://path/to/stellaron.db
```

Fish shell: `set -x DATABASE_URL sqlite://stellaron.db`

### Diesel Migrations
```bash
diesel migration generate create_feature  # Creates up.sql/down.sql
diesel migration run                      # Apply pending migrations
diesel migration redo                     # Rollback + reapply last
```
**Always regenerates `schema.rs`** - commit this file after migrations.

### Building & Testing
```bash
cargo tauri dev                     # Requires frontend in ../dist
cargo tauri build                   # Production build
cargo test -- --test-threads=1      # MUST be single-threaded for SQLite
```

**Common issues:**
- "database is locked" → Missing `lock_db()` in write operation
- "failed to apply pragmas" → Check DATABASE_URL format
- Tests fail randomly → Add `#[serial_test::serial]` attribute

## Critical Patterns

### Database Access (MANDATORY)

**Connection acquisition** (all operations):
```rust
use crate::data::database::connect_from_pool;
use diesel::result::{DatabaseErrorKind, Error};

let mut conn = connect_from_pool().await.map_err(|e| {
    Error::DatabaseError(DatabaseErrorKind::UnableToSendCommand, 
                         Box::new(e.to_string()))
})?;
```

**Write operations** (MUST use lock + transaction):
```rust
use crate::data::database::lock_db;
use diesel_async::{scoped_futures::ScopedFutureExt, AsyncConnection};

let db_lock = lock_db();
let _guard = db_lock.lock().await;  // Prevents "database is locked" errors

conn.transaction(|connection| {
    async move {
        diesel::insert_into(table)
            .values(item)
            .execute(connection)
            .await?;
        Ok(())
    }
    .scope_boxed()  // Required for diesel-async transactions
}).await
```

**Upsert pattern** (insert or update):
```rust
diesel::insert_into(table)
    .values((id.eq(1), &data))
    .on_conflict(id)
    .do_update()
    .set(&data)
    .execute(connection).await?;
```

**Return pattern** - Distinguish "not found" from errors:
```rust
match query.first::<Model>(&mut conn).await {
    Ok(value) => Ok(Some(value)),
    Err(Error::NotFound) => Ok(None),  // Entity doesn't exist
    Err(e) => Err(e),                   // Actual error
}
```

### Repository Trait Pattern

**Trait definition** (`repos/traits/repository.rs`):
```rust
use async_trait::async_trait;

#[async_trait]
pub trait Repository {
    type Item;                // Entity (Authors, Books, Users)
    type NewItem<'a>;         // Insertable form (NewAuthor<'a>)
    type Form<'a>;            // Update form (AuthorForm<'a>)
    type Id: Send + Sync;     // Primary key (usually i32)
    
    async fn get_all(&self) -> Result<Option<Vec<Self::Item>>, diesel::result::Error>;
    async fn get_by_id(&self, id: Self::Id) -> Result<Option<Self::Item>, diesel::result::Error>;
    async fn add<'a>(&self, item: Self::NewItem<'a>) -> Result<(), diesel::result::Error>;
    async fn update<'a>(&self, id: Self::Id, item: Self::Form<'a>) -> Result<(), diesel::result::Error>;
    async fn delete(&self, id: Self::Id) -> Result<(), diesel::result::Error>;
}
```

**Implementation** (`repos/implementors/author_repo.rs`):
```rust
pub struct AuthorRepo;  // NEVER store connection - acquire per method

impl AuthorRepo {
    pub async fn new() -> Self { AuthorRepo }
    
    // Custom domain methods beyond trait
    pub async fn search_by_name(&self, query: &str) -> Result<Option<Vec<Authors>>, Error> {
        let mut conn = connect_from_pool().await.map_err(/*...*/)?;
        authors.filter(name.like(format!("%{}%", query))).load(&mut conn).await
    }
}

#[async_trait]
impl Repository for AuthorRepo {
    type Item = Authors;
    type NewItem<'a> = NewAuthor<'a>;
    type Form<'a> = AuthorForm<'a>;
    type Id = i32;
    
    async fn get_all(&self) -> Result<Option<Vec<Self::Item>>, Error> {
        let mut conn = connect_from_pool().await.map_err(/*...*/)?;
        // query logic
    }
}
```

**Naming conventions:**
- Traits: `snake_case.rs` without `I` prefix (`repository.rs`, NOT `IRepository.rs`)
- Implementors: `CamelCase` struct (`AuthorRepo`, `DieselUserRepo` if multiple backends)
- NEVER store `conn` in struct fields - acquire fresh from pool per method

**Associated type lifetimes** (GATs):
- `NewItem<'a>` and `Form<'a>` use Generic Associated Types for borrowed data
- Example: `NewUser<'a>` has `username: &'a str` (no allocation on insert)

### Model Patterns

**Diesel derives** (`data/models/users.rs`):
```rust
use diesel::prelude::*;

#[derive(Queryable, Identifiable, Selectable, PartialEq, Debug)]
#[diesel(table_name = users)]
#[diesel(primary_key(user_id))]
pub struct Users {
    pub user_id: i32,
    pub username: String,
    pub password_hash: String,  // NEVER expose in DTOs
}

#[derive(Insertable, PartialEq, Debug)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,          // Borrowed - no allocation
    pub password_hash: &'a str,
}

#[derive(AsChangeset, PartialEq, Debug)]
#[diesel(table_name = users)]
pub struct UpdateUser<'a> {
    pub username: Option<&'a str>,  // Option for partial updates
    pub email: Option<&'a str>,
}
```

### Controller → DTO Pattern

**Controllers** (`controllers/user_controller.rs`):
```rust
use axum::{Json, response::{IntoResponse, Response}, http::StatusCode};

pub async fn create_user(Json(user): Json<NewUserDTO>) -> impl IntoResponse {
    let repo = UserRepo::new().await;
    let new_user = NewUser {
        username: &user.username,
        password_hash: &hash_password(&user.password),
    };
    
    match repo.add(new_user).await {
        Ok(_) => Response::builder().status(StatusCode::CREATED).body("User created").unwrap(),
        Err(e) => Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(format!("Error: {}", e)).unwrap(),
    }
}
```

**DTOs** (`controllers/dto/user_dto.rs`):
```rust
#[derive(Serialize, Deserialize)]
pub struct UserDTO {
    pub username: String,
    pub email: String,
    // NO password_hash field - sanitized for API responses
}

impl From<Users> for UserDTO {
    fn from(user: Users) -> Self {
        UserDTO { username: user.username, email: user.email }
    }
}
```

### Testing Patterns

**All database tests MUST use serial execution**:
```rust
use serial_test::serial;
use stellaron_lib::data::database;

async fn setup() -> Result<(), Error> {
    let mut conn = database::connect_from_pool().await?;
    use stellaron_lib::data::models::schema::users::dsl::*;
    diesel::delete(users).execute(&mut conn).await?;
    Ok(())
}

#[tokio::test]
#[serial]  // CRITICAL: Prevents SQLite locking errors
async fn test_create_user() {
    setup().await.expect("Failed to setup");
    
    let repo = UserRepo::new().await;
    let new_user = NewUser { username: "test", email: "test@example.com", password_hash: "hash", role: None, created_at: None };
    
    assert!(repo.add(new_user).await.is_ok());
    
    let users = repo.get_all().await.unwrap();
    assert!(users.is_some());
    assert_eq!(users.unwrap().len(), 1);
}
```

**Test imports** - Use `stellaron_lib` crate name:
```rust
use stellaron_lib::data::repos::implementors::user_repo::UserRepo;
use stellaron_lib::data::repos::traits::repository::Repository;
```

**Run tests**: `cargo test -- --test-threads=1`

## Key Dependencies

- **tauri** v2 - Desktop framework with custom titlebar (`transparent`, no decorations)
- **diesel-async** - Async ORM with `sqlite` + `deadpool` connection pooling
- **libsqlite3-sys** - **MUST use `bundled` feature** for cross-platform compatibility
- **rbook** - EPUB parsing with `threadsafe` feature enabled
- **axum** - HTTP framework for REST API (routes in `api.rs`)
- **tokio** - Async runtime (`features = ["full"]`)
- **argon2** - Password hashing (`features = ["rand", "std"]`)
- **serial_test** - Force sequential test execution (SQLite requirement)

## Integration Points

### Tauri ↔ Rust IPC
- Commands in `commands/` with `#[tauri::command]` attribute
- Registration: `tauri::generate_handler![command_name]` in builder
- Example: `commands/sample.rs` has `greet()` command (structural reference only)

### REST API Structure
```rust
// api.rs - spawned in main() before Tauri builder
tokio::spawn(async move {
    let api = Router::new()
        .route("/register", post(user_controller::create_user))
        .route("/list_users", get(user_controller::list_users))
        .route("/user", get(user_controller::get_user));
    
    axum::serve(TcpListener::bind("127.0.0.1:3000").await.unwrap(), api).await
});
```

### Database Schema Relationships
```
users ←─┐
        ├─ user_library ─→ books ←─┐
        └─ libraries               ├─ book_authors ─→ authors
                                   └─→ publishers
```

**Junction tables:**
- `book_authors` (many-to-many: books ↔ authors)
- `user_library` (many-to-many: users ↔ books with metadata)

**Schema location**: `src/data/models/schema.rs` (auto-generated by Diesel)
**Migrations**: `src/data/migrations/YYYY-MM-DD-*/up.sql`

## Implementation Status

**✅ Production Ready:**
- Database connection pooling + WAL mode + foreign keys
- Repository trait with 7 implementors (Author, Book, BookAuthor, Library, Publisher, User, UserLibrary)
- User REST endpoints (`/register`, `/list_users`, `/user`)
- DTO sanitization (UserDTO strips `password_hash`)
- Argon2 password hashing (AuthenticationService)
- Test infrastructure with serial execution

**🚧 Partial Implementation:**
- EPUB handler (`scan_epubs()` exists, metadata parsing incomplete)
- Book controller (exists but not exposed in `api.rs`)
- AuthenticationService (hashing done, JWT TODO)

**❌ Not Started:**
- All Tauri IPC commands (only `sample.rs` exists as template)
- Services layer (book, library, metadata, cover, OPDS services are stubs)
- MOBI/PDF handlers (empty files)
- OPDS feed generation (acquisition, navigation, search stubs)
- Authentication middleware (no auth on endpoints yet)

## Critical Gotchas

1. **SQLite PRAGMA auto-config**: First connection sets WAL mode, foreign keys, mmap (see `database.rs` PRAGMAS_SET atomic flag)
2. **Async runtime**: All code runs on Tokio - blocking operations cause deadlocks
3. **Write locks are mandatory**: SQLite errors if concurrent writes without `lock_db()`
4. **Repo connection lifetime**: NEVER store `conn` in repo struct - causes pool exhaustion
5. **Test parallelism**: SQLite can't handle concurrent tests - always use `#[serial]`
6. **Frontend location**: Tauri expects built frontend in `../dist` (set in `tauri.conf.json`)
7. **Error pattern**: `Result<Option<T>, Error>` means Ok(None) = not found, Err = actual failure
8. **Associated type bounds**: Trait `Id` types need `Send + Sync` for async methods
9. **Fish shell**: Use `set -x VAR value` for env vars, not `export`

## Quick Reference Commands

```fish
# Environment setup (Fish shell)
set -x DATABASE_URL sqlite://stellaron.db

# Diesel workflow
diesel migration generate add_feature
diesel migration run
diesel migration redo

# Development
cargo tauri dev                    # Frontend in ../dist required
cargo check                        # Fast compile check

# Testing
cargo test -- --test-threads=1     # All tests
cargo test user_repo               # Specific module
cargo test test_create_user        # Specific test

# Build
cargo tauri build                  # Production bundle
```

## Adding New Features

**New endpoint**:
1. Create controller function in `controllers/`
2. Create DTO in `controllers/dto/` (strip sensitive fields)
3. Add route to `api.rs` Router
4. Test with controller test in `tests/controller_tests.rs`

**New entity**:
1. Create migration: `diesel migration generate create_entity`
2. Write `up.sql` (CREATE TABLE) and `down.sql` (DROP TABLE)
3. Run `diesel migration run` (regenerates `schema.rs`)
4. Create model in `data/models/entity.rs` with derives
5. Create repo in `repos/implementors/entity_repo.rs`
6. Implement `Repository` trait with associated types
7. Add tests in `tests/entity_repo_tests.rs`

**Custom repo query**:
```rust
impl EntityRepo {
    pub async fn custom_query(&self, param: &str) -> Result<Option<Vec<Entity>>, Error> {
        use crate::data::models::schema::entities::dsl::*;
        let mut conn = connect_from_pool().await.map_err(/*...*/)?;
        
        match entities.filter(field.eq(param)).load(&mut conn).await {
            Ok(v) => Ok(Some(v)),
            Err(Error::NotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }
}
```
## Data Flow (Native)
1. Frontend (Tauri) sends IPC command contained within `src/commands/`
2. IPC command calls Service layer (e.g., BookService) for business logic
3. Service calls Repository layer (e.g., BookRepo) for data access
4. Repository interacts with Database via Diesel ORM
5. If repo returns a raw path. Service may call the handlers to process it further (e.g., read file contents) 
6. Processed data returns back up the chain to Frontend

## Data Flow (API)
1. Frontend sends HTTP request to Axum REST API (e.g., `/register`)
2. API route calls corresponding Controller function (e.g., `user_controller::create_user`)
3. Controller invokes Service layer (e.g., UserService) for business logic
4. Service calls Repository layer (e.g., UserRepo) for data access and then branches into two(2) possible flows:
   - If the repository returns a raw path to a file, the Service may call the appropriate handler to process the apropriate filetype (e.g., read metadata/contents from an epub file).
   - Else, it continues the normal data flow.
5. Repository interacts with Database via Diesel ORM
6. Data returns back up the chain to Controller
7. Controller converts Models to DTOs and sends HTTP response back to Frontend

## Planned features (backend)
- [] Read and process the following ebook formats to be sent as raw html to be rendered to the tauri frontend: MOBI, PDF, EPUB (partial)
- [] Implement Authentication with JWT tokens
- [] Bookmarking and Annotations
- [] OPDS feed generation and navigation for remote access
- [] Cover image extraction and caching
- [] (Optional) Dictionary lookup of highlighted words <-- DO NOT IMPLEMENT YET
- [] Multiple library support per user
- [] Sync reading progress across clients via REST API
- [] Advanced search and filtering options
