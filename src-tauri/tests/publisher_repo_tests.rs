use diesel::result::Error;
use diesel_async::RunQueryDsl;

use stellaron_lib::data::database;
use stellaron_lib::data::models::publishers::{NewPublisher, UpdatePublisher};
use stellaron_lib::data::repos::implementors::publisher_repo::PublisherRepo;
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

/// Helper function to create a test publisher
async fn create_test_publisher(name_val: &str) -> Result<(), Error> {
    let repo = PublisherRepo::new().await;
    let new_publisher = NewPublisher { name: name_val };

    repo.add(new_publisher).await
}

#[tokio::test]
#[serial_test::serial]
async fn test_create_publisher() {
    setup().await.expect("Failed to set up test");

    let name = "Test Publisher";
    let result = create_test_publisher(name).await;
    assert!(result.is_ok());

    let repo = PublisherRepo::new().await;
    let publishers = repo.get_all().await.expect("Failed to get publishers");

    assert!(publishers.is_some());
    let publishers_vec = publishers.unwrap();
    assert_eq!(publishers_vec.len(), 1);
    assert_eq!(publishers_vec[0].name, name);
}

#[tokio::test]
#[serial_test::serial]
async fn test_get_all_publishers_empty() {
    setup().await.expect("Failed to set up test");

    let repo = PublisherRepo::new().await;
    let publishers = repo.get_all().await.expect("Failed to get publishers");

    assert!(publishers.is_none());
}

#[tokio::test]
#[serial_test::serial]
async fn test_get_publisher_by_id() {
    setup().await.expect("Failed to set up test");

    let name = "Test Publisher";
    create_test_publisher(name)
        .await
        .expect("Failed to create test publisher");

    let repo = PublisherRepo::new().await;
    let publishers = repo
        .get_all()
        .await
        .expect("Failed to get publishers")
        .unwrap();
    let publisher_id = publishers[0].publisher_id;

    let publisher = repo
        .get_by_id(publisher_id)
        .await
        .expect("Failed to get publisher by id");

    assert!(publisher.is_some());
    let found_publisher = publisher.unwrap();
    assert_eq!(found_publisher.name, name);
}

#[tokio::test]
#[serial_test::serial]
async fn test_update_publisher() {
    setup().await.expect("Failed to set up test");

    let name = "Original Name";
    create_test_publisher(name)
        .await
        .expect("Failed to create test publisher");

    let repo = PublisherRepo::new().await;
    let publishers = repo
        .get_all()
        .await
        .expect("Failed to get publishers")
        .unwrap();
    let publisher_id = publishers[0].publisher_id;

    let new_name = "Updated Name";
    let form = UpdatePublisher {
        name: Some(new_name),
    };
    let result = repo.update(publisher_id, form).await;
    assert!(result.is_ok());

    let updated_publisher = repo
        .get_by_id(publisher_id)
        .await
        .expect("Failed to get publisher by id")
        .unwrap();
    assert_eq!(updated_publisher.name, new_name);
}

#[tokio::test]
#[serial_test::serial]
async fn test_delete_publisher() {
    setup().await.expect("Failed to set up test");

    let name = "To Be Deleted";
    create_test_publisher(name)
        .await
        .expect("Failed to create test publisher");

    let repo = PublisherRepo::new().await;
    let publishers = repo
        .get_all()
        .await
        .expect("Failed to get publishers")
        .unwrap();
    let publisher_id = publishers[0].publisher_id;

    let result = repo.delete(publisher_id).await;
    assert!(result.is_ok());

    let publisher = repo
        .get_by_id(publisher_id)
        .await
        .expect("Failed to get publisher by id");
    assert!(publisher.is_none());
}

#[tokio::test]
#[serial_test::serial]
async fn test_search_publisher_by_name() {
    setup().await.expect("Failed to set up test");

    create_test_publisher("Penguin Books").await.unwrap();
    create_test_publisher("HarperCollins").await.unwrap();
    create_test_publisher("Penguin Random House").await.unwrap();

    let repo = PublisherRepo::new().await;
    let results = repo
        .search_by_name("Penguin")
        .await
        .expect("Failed to search by name")
        .unwrap();

    assert_eq!(results.len(), 2);
    assert!(results.iter().any(|p| p.name == "Penguin Books"));
    assert!(results.iter().any(|p| p.name == "Penguin Random House"));
}
