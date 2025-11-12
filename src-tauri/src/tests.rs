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
    use crate::data::models::libraries::NewLibrary;
    use crate::data::repos::implementors::library_repo::LibraryRepo;
    use crate::data::repos::traits::repository::Repository;

    /// Helper function to clear the libraries table before each test
    async fn setup() -> Result<(), Error> {
        let mut conn = database::connect_from_pool()
            .await
            .expect("Failed to get connection from pool for test setup");

        use crate::data::models::schema::libraries::dsl::*;
        diesel::delete(libraries).execute(&mut conn).await?;

        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_get_all_configurations_empty() {
        setup().await.expect("Failed to set up test");

        let repo = LibraryRepo::new().await;
        let libraries = repo.get_all().await.expect("Failed to get libraries");

        assert!(libraries.is_some());
        assert_eq!(libraries.unwrap().len(), 0);
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
        use crate::data::models::libraries::UpdateLibrary;
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
}

#[cfg(test)]
mod user_repo_tests {
    use diesel::result::Error;
    use diesel_async::RunQueryDsl;

    use crate::data::database;
    use crate::data::models::users::NewUser;
    use crate::data::repos::implementors::user_repo::UserRepo;
    use crate::data::repos::traits::repository::Repository;

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
}

// TODO: Test controllers
// TODO: Test services
