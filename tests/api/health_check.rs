use crate::helpers::spawn_app;

#[actix_web::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;
    println!(
        "{}",
        format!("Server is up and running at :{}", &app.address)
    );

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health_check", app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
