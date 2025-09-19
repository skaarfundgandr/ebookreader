pub mod commands;
pub mod controllers;
pub mod data;
pub mod handlers;
pub mod utils;

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_sqlite_connection() {
        use crate::data::database;
        let _conn = database::connect_from_pool().await;

        assert!(_conn.is_ok());
    }

    //TODO: Finish this test
    #[tokio::test]
    async fn test_configuration_repo() {
        use crate::data::repos::configuration_repo::get_all_configurations;

        let configurations = get_all_configurations().await;

        assert!(!configurations.is_err());
    }
}
