pub mod data;
pub mod handlers;
pub mod commands;
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_sqlite_connection(){
        use crate::data::database;
        let _conn = database::connect_from_pool().await;

        assert!(_conn.is_ok());
    }
}



