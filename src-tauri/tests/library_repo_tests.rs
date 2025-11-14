use diesel::result::Error;
use diesel_async::RunQueryDsl;

use stellaron_lib::data::database;
use stellaron_lib::data::models::libraries::{NewLibrary, UpdateLibrary};
use stellaron_lib::data::repos::implementors::library_repo::LibraryRepo;
use stellaron_lib::data::repos::traits::repository::Repository;

/// Helper function to clear the libraries table before each test
async fn setup() -> Result<(), Error> {
    let mut conn = database::connect_from_pool()
        .await
        .expect("Failed to get connection from pool for test setup");

    use stellaron_lib::data::models::schema::libraries::dsl::*;
    diesel::delete(libraries).execute(&mut conn).await?;

    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_get_all_configurations_empty() {
    setup().await.expect("Failed to set up test");

    let repo = LibraryRepo::new().await;
    let libraries = repo.get_all().await.expect("Failed to get libraries");

    assert!(libraries.is_none());
}

#[tokio::test]
#[serial_test::serial]
async fn test_set_and_get_book_path() {
    setup().await.expect("Failed to set up test");

    let repo = LibraryRepo::new().await;
    let test_path = "/test/book/path";
    let new_library = NewLibrary {
        name: "Test Library",
        path: test_path,
        added_by: None,
    };

    let result = repo.add(new_library).await;
    assert!(result.is_ok());

    let libraries = repo.get_all().await.expect("Failed to get libraries");
    assert!(libraries.is_some());
    let libs = libraries.unwrap();
    assert_eq!(libs.len(), 1);
    assert_eq!(libs[0].path, test_path);
}

#[tokio::test]
#[serial_test::serial]
async fn test_get_all_configurations_with_data() {
    setup().await.expect("Failed to set up test");

    let repo = LibraryRepo::new().await;
    let test_path = "/another/test/path";
    let new_library = NewLibrary {
        name: "Another Library",
        path: test_path,
        added_by: None,
    };

    let result = repo.add(new_library).await;
    assert!(result.is_ok());

    let libraries = repo.get_all().await.expect("Failed to get libraries");

    assert!(libraries.is_some());
    let libs = libraries.unwrap();
    assert_eq!(libs.len(), 1);
    assert_eq!(libs[0].path, test_path);
}

#[tokio::test]
#[serial_test::serial]
async fn test_update_book_path() {
    setup().await.expect("Failed to set up test");

    let repo = LibraryRepo::new().await;
    let initial_path = "/initial/path";
    let new_library = NewLibrary {
        name: "Initial Library",
        path: initial_path,
        added_by: None,
    };

    repo.add(new_library).await.expect("Failed to add library");

    let libraries = repo
        .get_all()
        .await
        .expect("Failed to get libraries")
        .unwrap();
    let library_id = libraries[0].library_id;

    let updated_path = "/updated/path";
    let update = UpdateLibrary {
        name: None,
        path: Some(updated_path),
        added_by: None,
    };

    let result = repo.update(library_id, update).await;
    assert!(result.is_ok());

    let library = repo
        .get_by_id(library_id)
        .await
        .expect("Failed to get library");
    assert!(library.is_some());
    assert_eq!(library.unwrap().path, updated_path);
}
