use fruit_cards::main;

#[tokio::test]
async fn health_check_works(){
    // Arrange
    let address = spawn_app();

    // Act
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}