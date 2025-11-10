# Stellaron - Ebook Reader Project

## Architecture Overview

**Tauri v2** desktop app with dual execution contexts sharing a single SQLite database:

1. **Tauri GUI** - Window management + IPC commands (example in `commands/sample.rs`)
2. **Axum REST API** (`api.rs::start()`) - Spawned tokio task on `127.0.0.1:3000`

Both share the same `deadpool` connection pool via `data/database.rs`.

### Project Structure

```
src/
├── api.rs              # Axum REST API server (spawned as background task)
├── main.rs             # Entry point: starts both API and Tauri
├── commands/           # Tauri IPC commands (sample.rs is example only)
├── controllers/        # Axum route handlers with DTOs
│   └── dto/           # Data transfer objects for API
├── data/
│   ├── database.rs    # Pooled SQLite connection (diesel-async)
│   ├── models/        # Diesel schema and model definitions
│   ├── repos/         # Data access layer (async diesel queries)
│   │   ├── traits/    # Repository trait definitions
│   │   └── implementors/ # Diesel/SQLite implementations
│   └── migrations/    # Diesel migration files
├── handlers/          # Business logic (epub_handler, mobi_handler, pdf_handler)
├── services/          # Service layer (book, library, metadata, opds, etc.)
└── opds/              # OPDS feed generation (acquisition, navigation, search)
```

## Developer Workflows

### Environment Setup
Create `.env` file in project root:
```bash
DATABASE_URL=sqlite://path/to/your/database.db
```

### Diesel Migrations
```bash
diesel migration generate <name>    # Create new migration
diesel migration run                # Apply migrations
diesel migration redo               # Regenerate schema.rs
```

### Building & Testing
```bash
cargo tauri dev                     # Dev mode (requires frontend in ../dist)
cargo tauri build                   # Production build
cargo test -- --test-threads=1      # Tests (MUST be single-threaded for SQLite)
```

## Critical Patterns

### Database Access (Required)

**Read operations:**
```rust
let mut conn = connect_from_pool().await.map_err(|e| {
    Error::DatabaseError(DatabaseErrorKind::UnableToSendCommand, 
                         Box::new(e.to_string()))
})?;
```

**Write operations (MUST use lock + transaction):**
```rust
let db_lock = lock_db();
let _guard = db_lock.lock().await;

conn.transaction(|connection| {
    async move {
        diesel::insert_into(table).values(item).execute(connection).await?;
        Ok(())
    }
    .scope_boxed()  // Required for diesel-async transactions
}).await
```

**Upsert pattern** (see `configuration_repo.rs::set_book_path`):
```rust
diesel::insert_into(table)
    .values((id.eq(1), &data))
    .on_conflict(id)
    .do_update()
    .set(&data)
    .execute(connection).await?;
```

### Repository Trait Pattern (Active Refactoring)

Generic `Repository` trait with associated types in `repos/traits/repository.rs`:
```rust
#[async_trait]
pub trait Repository {
    type Item;       // Entity type (e.g., Authors, Books)
    type NewItem;    // Insert form (e.g., NewAuthor)
    type Form;       // Update form (e.g., AuthorForm)
    type Id: Send + Sync;  // Primary key type (usually i32)
    
    async fn get_all(&self) -> Result<Option<Vec<Self::Item>>, diesel::result::Error>;
    async fn get_by_id(&self, id: Self::Id) -> Result<Option<Self::Item>, diesel::result::Error>;
    async fn add(&self, item: Self::Item) -> Result<Self::Item, diesel::result::Error>;
    async fn update(&self, item: Self::Item) -> Result<(), diesel::result::Error>;
    async fn delete(&self, id: Self::Id) -> Result<(), diesel::result::Error>;
}
```

**Implementation example** (`repos/implementors/author_repo.rs`):
```rust
pub struct AuthorRepo;  // No connection stored

#[async_trait]
impl Repository for AuthorRepo {
    type Item = Authors;
    type NewItem = NewAuthor<'static>;
    type Form = AuthorForm<'static>;
    type Id = i32;
    
    async fn get_all(&self) -> Result<Option<Vec<Self::Item>>, diesel::result::Error> {
        let mut conn = connect_from_pool().await.map_err(/*...*/)?;
        // query logic
    }
}
```

**Naming conventions:**
- Trait files: `snake_case` without `I` prefix (e.g., `repository.rs`, `author_repository.rs`)
- Implementor structs: `CamelCase` with optional backend qualifier (e.g., `AuthorRepo`, `DieselAuthorRepo`)
- Never store connection objects in repo struct fields - acquire per method

### Module Conventions

- **Models** (`data/models/*.rs`): Diesel structs with `Queryable`, `Insertable`, `AsChangeset` derives
- **Repos** (`data/repos/`): Trait-based architecture
  - **Traits** (`repos/traits/`): Generic `Repository` trait with associated types (`Item`, `NewItem`, `Form`, `Id`)
  - **Implementors** (`repos/implementors/`): Concrete Diesel implementations (e.g., `AuthorRepo`, `BookRepo`)
  - Return `Result<Option<T>, Error>` - `None` = not found, `Err` = query failed
  - **Don't store connections in struct fields** - acquire from pool per method via `connect_from_pool()`
- **Controllers** (`controllers/*.rs`): Axum handlers, convert Models → DTOs to hide sensitive fields
- **DTOs** (`controllers/dto/*.rs`): Implement `From<Model>` for clean conversions
- **Services** (`services/*.rs`): Business logic layer coordinating repos and handlers
- **Handlers** (`handlers/*.rs`): File format-specific logic (epub, mobi, pdf parsing)

### Testing

All database tests **require** `#[serial_test::serial]` due to shared SQLite:
```rust
#[tokio::test]
#[serial_test::serial]
async fn test_name() {
    setup().await.expect("Failed to setup");  // Clears tables
    // test logic
}
```

Run with: `cargo test`

## Key Dependencies

- **rbook**: EPUB parsing library (with `threadsafe` feature)
- **diesel-async**: Async diesel with `sqlite` + `deadpool` features
- **libsqlite3-sys**: Force `bundled` feature for cross-platform compatibility
- **tauri**: v2 with custom titlebar (transparent, no decorations)

## Integration Points

### Tauri ↔ Rust
- Commands defined in `commands/` with `#[tauri::command]` macro
- Registered via `tauri::generate_handler![command_name]` in main Tauri builder
- See `commands/sample.rs` for example structure (sample only, not production)

### REST API Endpoints
- Server auto-starts on `127.0.0.1:3000` via tokio spawn in `main()`
- Routes: `/create_user` (POST), `/list_users` (GET), `/user?user_id=<id>` (GET)

### Database Schema
- **Multi-table relationships**: Books ↔ Authors (via book_authors junction), Books → Publishers, Users ↔ Books (via user_library)
- **Schema location**: `src/data/models/schema.rs` (auto-generated by Diesel)
- **Migration directory**: `src/data/migrations/`

## Current TODOs & Limitations

- **Repository refactoring**: Converting from standalone functions to trait-based implementors (see `repos/traits/` and `repos/implementors/`)
- **EPUB handler** (`handlers/epub_handler.rs`): `scan_epubs()` implementation incomplete
- **File handlers**: MOBI and PDF handlers are empty stubs
- **Authentication**: User controller lacks auth/authorization
- **Book controller**: Not yet exposed via API (exists but no routes defined)
- **Tauri commands**: Only sample `greet` command exists as structural example
- **Tests**: Controller layer tests not yet implemented

## Development Tips

1. **SQLite PRAGMA settings**: Connection pool auto-configures WAL mode, foreign keys, and mmap on first connection (see `database.rs` PRAGMAS_SET)
2. **Async runtime**: Everything runs on Tokio - avoid blocking operations
3. **Error handling**: Repos use `Result<Option<T>, Error>` pattern - distinguish between "not found" (Ok(None)) vs actual errors
4. **Frontend integration**: Expects built frontend in `../dist` (configured in `tauri.conf.json`)
5. **SQLite concurrency**: Use `lock_db()` mutex for all write operations to prevent "database is locked" errors
6. **Associated types**: Use `Send + Sync` bounds on trait `Id` types for async/thread safety
