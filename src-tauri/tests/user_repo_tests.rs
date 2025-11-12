use diesel::result::Error;
use diesel_async::RunQueryDsl;

use stellaron_lib::data::database;
use stellaron_lib::data::models::users::NewUser;
use stellaron_lib::data::repos::implementors::user_repo::UserRepo;
use stellaron_lib::data::repos::traits::repository::Repository;

/// Helper function to clear the users table before each test
async fn setup() -> Result<(), Error> {
    let mut conn = database::connect_from_pool()
        .await
        .expect("Failed to get connection from pool for test setup");

    use stellaron_lib::data::models::schema::users::dsl::*;
    diesel::delete(users).execute(&mut conn).await?;

    Ok(())
}

/// Helper function to create a test user
async fn create_test_user(
    username_val: &str,
    email_val: &str,
    password_val: &str,
) -> Result<(), Error> {
    let repo = UserRepo::new().await;
    let new_user = NewUser {
        username: username_val,
        email: email_val,
        password_hash: password_val,
        created_at: None,
    };

    repo.add(new_user).await
}

#[tokio::test]
#[serial_test::serial]
async fn test_create_user() {
    setup().await.expect("Failed to set up test");

    let username = "testuser";
    let email = "test@example.com";
    let password = "hashedpassword123";

    let result = create_test_user(username, email, password).await;
    assert!(result.is_ok());

    let repo = UserRepo::new().await;
    let users = repo.get_all().await.expect("Failed to get users");

    assert!(users.is_some());
    let users_vec = users.unwrap();
    assert_eq!(users_vec.len(), 1);
    assert_eq!(users_vec[0].username, username);
    assert_eq!(users_vec[0].email, email);
    assert_eq!(users_vec[0].password_hash, password);
}

#[tokio::test]
#[serial_test::serial]
async fn test_get_all_users_empty() {
    setup().await.expect("Failed to set up test");

    let repo = UserRepo::new().await;
    let users = repo.get_all().await.expect("Failed to get users");

    assert!(users.is_some());
    assert_eq!(users.unwrap().len(), 0);
}

#[tokio::test]
#[serial_test::serial]
async fn test_get_user_by_id() {
    setup().await.expect("Failed to set up test");

    let username = "testuser";
    let email = "test@example.com";
    let password = "hashedpassword123";

    create_test_user(username, email, password)
        .await
        .expect("Failed to create test user");

    let repo = UserRepo::new().await;
    let users = repo.get_all().await.expect("Failed to get users").unwrap();
    let user_id = users[0].user_id;

    let user = repo
        .get_by_id(user_id)
        .await
        .expect("Failed to get user by id");

    assert!(user.is_some());
    let found_user = user.unwrap();
    assert_eq!(found_user.username, username);
    assert_eq!(found_user.email, email);
}

#[tokio::test]
#[serial_test::serial]
async fn test_get_user_by_id_nonexistent() {
    setup().await.expect("Failed to set up test");

    let repo = UserRepo::new().await;
    let user = repo
        .get_by_id(999)
        .await
        .expect("Failed to execute get_user_by_id");

    assert!(user.is_none());
}

#[tokio::test]
#[serial_test::serial]
async fn test_get_user_by_username() {
    setup().await.expect("Failed to set up test");

    let username = "uniqueuser";
    let email = "unique@example.com";
    let password = "uniquepass123";

    create_test_user(username, email, password)
        .await
        .expect("Failed to create test user");

    let repo = UserRepo::new().await;
    let user = repo
        .search_by_username(username)
        .await
        .expect("Failed to get user by username");

    assert!(user.is_some());
    let found_user = user.unwrap();
    assert_eq!(found_user.username, username);
    assert_eq!(found_user.email, email);
}

#[tokio::test]
#[serial_test::serial]
async fn test_get_user_by_username_nonexistent() {
    setup().await.expect("Failed to set up test");

    let repo = UserRepo::new().await;
    let user = repo
        .search_by_username("nonexistentuser")
        .await
        .expect("Failed to execute get_user_by_username");

    assert!(user.is_none());
}

#[tokio::test]
#[serial_test::serial]
async fn test_get_all_users_multiple() {
    setup().await.expect("Failed to set up test");

    let usernames = ["user1", "user2", "user3"];
    let emails = [
        "user1@example.com",
        "user2@example.com",
        "user3@example.com",
    ];
    let passwords = ["pass1", "pass2", "pass3"];

    for i in 0..3 {
        create_test_user(usernames[i], emails[i], passwords[i])
            .await
            .expect("Failed to create test user");
    }

    let repo = UserRepo::new().await;
    let users = repo.get_all().await.expect("Failed to get users");

    assert!(users.is_some());
    let users_vec = users.unwrap();
    assert_eq!(users_vec.len(), 3);

    for i in 0..3 {
        let user = users_vec.iter().find(|u| u.username == usernames[i]);
        assert!(user.is_some());
        let user = user.unwrap();
        assert_eq!(user.email, emails[i]);
        assert_eq!(user.password_hash, passwords[i]);
    }
}
