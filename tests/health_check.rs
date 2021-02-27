use zero2prod::run;
use std::net::TcpListener;

#[actix_rt::test]
async fn health_check_works() {
    spawn_app();
    // reqwest to perform HTTP requests against our app

    let client = reqwest::Client::new();
    let address = spawn_app();

    // Act
    let response = client
            .get(&format!("{}/health_check", &address))
            .send()
            .await
            .expect("Failed to execute request.");

    // Assertions
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    return format!("http://127.0.0.1:{}", port);
}