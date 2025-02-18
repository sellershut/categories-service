use anyhow::Result;
use sellershut_core::categories::{DeleteCategoryRequest, GetCategoryRequest};
use sqlx::PgPool;
use tonic::IntoRequest;

use crate::helpers::TestApp;

#[sqlx::test(fixtures(path = "../.././fixtures", scripts("categories")))]
async fn delete_category(pool: PgPool) -> Result<()> {
    let mut app = TestApp::new(pool).await;
    let ap_id = "http://localhost/category/item7".to_string();

    let category_request = DeleteCategoryRequest {
        ap_id: ap_id.clone(),
    };

    app.mutate
        .delete(category_request.into_request())
        .await?
        .into_inner();

    let getter = GetCategoryRequest { ap_id }.into_request();

    let response = app
        .query
        .category_by_ap_id(getter)
        .await?
        .into_inner()
        .category;

    assert!(response.is_none());

    Ok(())
}
