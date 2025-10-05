pub mod commands;
pub mod controllers;
pub mod data;
pub mod handlers;
pub mod utils;

#[cfg(test)]
mod connection_tests {
    #[tokio::test]
    #[serial_test::serial]
    async fn test_sqlite_connection() {
        use crate::data::database;
        let _conn = database::connect_from_pool().await;

        assert!(_conn.is_ok());
    }
}

#[cfg(test)]
mod configuration_repo_tests {
    use diesel::result::Error;
    use diesel_async::RunQueryDsl;

    use crate::data::database;
    use crate::data::repos::configuration_repo::{
        get_all_configurations, get_book_path, set_book_path,
    };

    /// Helper function to clear the configuration table before each test
    async fn setup() -> Result<(), Error> {
        let mut conn = database::connect_from_pool()
            .await
            .expect("Failed to get connection from pool for test setup");

        use crate::data::models::schema::configuration::dsl::*;
        diesel::delete(configuration).execute(&mut conn).await?;

        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_get_all_configurations_empty() {
        // Setup: ensure table is empty
        setup().await.expect("Failed to set up test");

        // Test get_all_configurations with empty table
        let configurations = get_all_configurations()
            .await
            .expect("Failed to get configurations");

        // Should return Some with an empty vector
        assert!(configurations.is_some());
        assert_eq!(configurations.unwrap().len(), 0);
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_set_and_get_book_path() {
        // Setup: ensure table is empty
        setup().await.expect("Failed to set up test");

        // Test setting a book path
        let test_path = "/test/book/path";
        let result = set_book_path(test_path).await;
        assert!(result.is_ok());

        // Test getting the book path
        let path = get_book_path().await.expect("Failed to get book path");
        assert_eq!(path, Some(test_path.to_string()));
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_get_all_configurations_with_data() {
        // Setup: ensure table is empty
        setup().await.expect("Failed to set up test");

        // Insert a test configuration directly
        let test_path = "/another/test/path";
        let result = set_book_path(test_path).await;
        assert!(result.is_ok());

        // Test get_all_configurations with data
        let configurations = get_all_configurations()
            .await
            .expect("Failed to get configurations");

        // Should return Some with one configuration
        assert!(configurations.is_some());
        let configs = configurations.unwrap();
        assert_eq!(configs.len(), 1);
        assert_eq!(configs[0].book_path, Some(test_path.to_string()));
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_update_book_path() {
        // Setup: ensure table is empty
        setup().await.expect("Failed to set up test");

        // Insert initial book path
        let initial_path = "/initial/path";
        let result = set_book_path(initial_path).await;
        assert!(result.is_ok());

        // Update the book path
        let updated_path = "/updated/path";
        let result = set_book_path(updated_path).await;
        assert!(result.is_ok());

        // Verify the update
        let path = get_book_path().await.expect("Failed to get book path");
        assert_eq!(path, Some(updated_path.to_string()));
    }
}

#[cfg(test)]
mod user_repo_tests {
    use diesel::result::Error;
    use diesel_async::RunQueryDsl;

    use crate::controllers::dto::user_dto::NewUserDTO;
    use crate::data::database;
    use crate::data::repos::user_repo::{
        create_user, get_all_users, get_user_by_id, get_user_by_username,
    };

    /// Helper function to clear the users table before each test
    async fn setup() -> Result<(), Error> {
        let mut conn = database::connect_from_pool()
            .await
            .expect("Failed to get connection from pool for test setup");

        use crate::data::models::schema::users::dsl::*;
        diesel::delete(users).execute(&mut conn).await?;

        Ok(())
    }

    /// Helper function to create a test user
    async fn create_test_user(
        username_val: &str,
        email_val: &str,
        password_val: &str,
    ) -> Result<(), Error> {
        let new_user = NewUserDTO {
            username: username_val,
            email: email_val,
            password_hash: password_val,
            created_at: None,
        };

        create_user(new_user).await
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_create_user() {
        // Setup: ensure table is empty
        setup().await.expect("Failed to set up test");

        // Create a test user
        let username = "testuser";
        let email = "test@example.com";
        let password = "hashedpassword123";

        let result = create_test_user(username, email, password).await;
        assert!(result.is_ok());

        // Verify the user was created by getting all users
        let users = get_all_users().await.expect("Failed to get users");

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
        // Setup: ensure table is empty
        setup().await.expect("Failed to set up test");

        // Test get_all_users with empty table
        let users = get_all_users().await.expect("Failed to get users");

        // Should return Some with an empty vector
        assert!(users.is_some());
        assert_eq!(users.unwrap().len(), 0);
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_get_user_by_id() {
        // Setup: ensure table is empty
        setup().await.expect("Failed to set up test");

        // Create a test user
        let username = "testuser";
        let email = "test@example.com";
        let password = "hashedpassword123";

        create_test_user(username, email, password)
            .await
            .expect("Failed to create test user");

        // Get all users to find the ID
        let users = get_all_users().await.expect("Failed to get users").unwrap();
        let user_id = users[0].user_id;

        // Test get_user_by_id
        let user = get_user_by_id(user_id)
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
        // Setup: ensure table is empty
        setup().await.expect("Failed to set up test");

        // Test get_user_by_id with nonexistent ID
        let user = get_user_by_id(999)
            .await
            .expect("Failed to execute get_user_by_id");

        assert!(user.is_none());
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_get_user_by_username() {
        // Setup: ensure table is empty
        setup().await.expect("Failed to set up test");

        // Create a test user
        let username = "uniqueuser";
        let email = "unique@example.com";
        let password = "uniquepass123";

        create_test_user(username, email, password)
            .await
            .expect("Failed to create test user");

        // Test get_user_by_username
        let user = get_user_by_username(username)
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
        // Setup: ensure table is empty
        setup().await.expect("Failed to set up test");

        // Test get_user_by_username with nonexistent username
        let user = get_user_by_username("nonexistentuser")
            .await
            .expect("Failed to execute get_user_by_username");

        assert!(user.is_none());
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_get_all_users_multiple() {
        // Setup: ensure table is empty
        setup().await.expect("Failed to set up test");

        // Create multiple test users
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

        // Test get_all_users
        let users = get_all_users().await.expect("Failed to get users");

        assert!(users.is_some());
        let users_vec = users.unwrap();
        assert_eq!(users_vec.len(), 3);

        // Verify each user
        for i in 0..3 {
            let user = users_vec.iter().find(|u| u.username == usernames[i]);
            assert!(user.is_some());
            let user = user.unwrap();
            assert_eq!(user.email, emails[i]);
            assert_eq!(user.password_hash, passwords[i]);
        }
    }
}
// TODO: Test controllers
