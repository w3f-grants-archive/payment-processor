//! ISO-8583 processing tests

use crate::tests::mock::*;

#[tokio::test]
async fn test_works() {
    let api = MockProcessorImpl::new().await;
}
