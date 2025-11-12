use stellaron_lib::data::database;

#[tokio::test]
#[serial_test::serial]
async fn test_sqlite_connection() {
    let _conn = database::connect_from_pool().await;

    assert!(_conn.is_ok());
}
