use merino::*;

#[tokio::test]
/// Can we crate a new `Merino` instance
async fn merino_contructor() {
    assert!(Merino::new(1080, "0.0.0.0", Vec::new(), Vec::new())
        .await
        .is_ok())
}
