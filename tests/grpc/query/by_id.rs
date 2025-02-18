use anyhow::Result;
use sellershut_core::categories::GetCategoryRequest;
use sqlx::PgPool;
use tonic::IntoRequest;

use crate::helpers::TestApp;

#[sqlx::test(fixtures(path = "../.././fixtures", scripts("categories")))]
async fn category_by_id(pool: PgPool) -> Result<()> {
    let mut app = TestApp::new(pool).await;

    let getter = GetCategoryRequest {
        ap_id: "http://localhost/category/cat1".to_string(),
    }
    .into_request();

    let response = app
        .query
        .category_by_ap_id(getter)
        .await?
        .into_inner()
        .category;

    assert!(response.is_some());

    Ok(())
}
