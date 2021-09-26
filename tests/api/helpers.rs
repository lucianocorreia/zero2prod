/*
Helper to spawn the app for tests
*/
pub fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");

    let _ = tokio::spawn(server);
}
