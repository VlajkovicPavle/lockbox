use api::server::Server;
use axum::http::StatusCode;

const LOCALHOST: &str = "127.0.01";

#[tokio::test]
async fn test_server_health_route() {
    let server = Server::new(3000, LOCALHOST.to_string()).await.unwrap();
    tokio::spawn(async move {
        server.run().await.unwrap();
    });
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    let client = httpc_test::new_client(format!("http://{}:3000", LOCALHOST)).unwrap();
    let res = client.do_get("/health").await.unwrap();
    assert!(res.status() == StatusCode::OK)
}
