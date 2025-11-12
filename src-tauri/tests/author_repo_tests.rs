use diesel::result::Error;
use diesel_async::RunQueryDsl;

use stellaron_lib::data::database;
use stellaron_lib::data::models::authors::{AuthorForm, NewAuthor};
use stellaron_lib::data::repos::implementors::author_repo::AuthorRepo;
use stellaron_lib::data::repos::traits::repository::Repository;

/// Helper function to clear the tables before each test
async fn setup() -> Result<(), Error> {
    let mut conn = database::connect_from_pool()
        .await
        .expect("Failed to get connection from pool for test setup");

    use stellaron_lib::data::models::schema::authors::dsl::*;
    use stellaron_lib::data::models::schema::book_authors::dsl::*;
    use stellaron_lib::data::models::schema::books::dsl::*;
    use stellaron_lib::data::models::schema::publishers::dsl::*;
    use stellaron_lib::data::models::schema::user_library::dsl::*;
    use stellaron_lib::data::models::schema::users::dsl::*;

    diesel::delete(book_authors).execute(&mut conn).await?;
    diesel::delete(user_library).execute(&mut conn).await?;
    diesel::delete(books).execute(&mut conn).await?;
    diesel::delete(authors).execute(&mut conn).await?;
    diesel::delete(publishers).execute(&mut conn).await?;
    diesel::delete(users).execute(&mut conn).await?;

    Ok(())
}

/// Helper function to create a test author
async fn create_test_author(name_val: &str) -> Result<(), Error> {
    let repo = AuthorRepo::new().await;
    let new_author = NewAuthor { name: name_val };

    repo.add(new_author).await
}

#[tokio::test]
#[serial_test::serial]
async fn test_create_author() {
    setup().await.expect("Failed to set up test");

    let name = "Test Author";
    let result = create_test_author(name).await;
    assert!(result.is_ok());

    let repo = AuthorRepo::new().await;
    let authors = repo.get_all().await.expect("Failed to get authors");

    assert!(authors.is_some());
    let authors_vec = authors.unwrap();
    assert_eq!(authors_vec.len(), 1);
    assert_eq!(authors_vec[0].name, name);
}

#[tokio::test]
#[serial_test::serial]
async fn test_get_all_authors_empty() {
    setup().await.expect("Failed to set up test");

    let repo = AuthorRepo::new().await;
    let authors = repo.get_all().await.expect("Failed to get authors");

    assert!(authors.is_some());
    assert_eq!(authors.unwrap().len(), 0);
}

#[tokio::test]
#[serial_test::serial]
async fn test_get_author_by_id() {
    setup().await.expect("Failed to set up test");

    let name = "Test Author";
    create_test_author(name)
        .await
        .expect("Failed to create test author");

    let repo = AuthorRepo::new().await;
    let authors = repo
        .get_all()
        .await
        .expect("Failed to get authors")
        .unwrap();
    let author_id = authors[0].author_id;

    let author = repo
        .get_by_id(author_id)
        .await
        .expect("Failed to get author by id");

    assert!(author.is_some());
    let found_author = author.unwrap();
    assert_eq!(found_author.name, name);
}

#[tokio::test]
#[serial_test::serial]
async fn test_update_author() {
    setup().await.expect("Failed to set up test");

    let name = "Original Name";
    create_test_author(name)
        .await
        .expect("Failed to create test author");

    let repo = AuthorRepo::new().await;
    let authors = repo
        .get_all()
        .await
        .expect("Failed to get authors")
        .unwrap();
    let author_id = authors[0].author_id;

    let new_name = "Updated Name";
    let form = AuthorForm {
        name: Some(new_name),
    };
    let result = repo.update(author_id, form).await;
    assert!(result.is_ok());

    let updated_author = repo
        .get_by_id(author_id)
        .await
        .expect("Failed to get author by id")
        .unwrap();
    assert_eq!(updated_author.name, new_name);
}

#[tokio::test]
#[serial_test::serial]
async fn test_delete_author() {
    setup().await.expect("Failed to set up test");

    let name = "To Be Deleted";
    create_test_author(name)
        .await
        .expect("Failed to create test author");

    let repo = AuthorRepo::new().await;
    let authors = repo
        .get_all()
        .await
        .expect("Failed to get authors")
        .unwrap();
    let author_id = authors[0].author_id;

    let result = repo.delete(author_id).await;
    assert!(result.is_ok());

    let author = repo
        .get_by_id(author_id)
        .await
        .expect("Failed to get author by id");
    assert!(author.is_none());
}

#[tokio::test]
#[serial_test::serial]
async fn test_search_author_by_name() {
    setup().await.expect("Failed to set up test");

    create_test_author("Jane Austen").await.unwrap();
    create_test_author("George Orwell").await.unwrap();
    create_test_author("George R. R. Martin").await.unwrap();

    let repo = AuthorRepo::new().await;
    let results = repo
        .search_by_name("George")
        .await
        .expect("Failed to search by name")
        .unwrap();

    assert_eq!(results.len(), 2);
    assert!(results.iter().any(|a| a.name == "George Orwell"));
    assert!(results.iter().any(|a| a.name == "George R. R. Martin"));
}
