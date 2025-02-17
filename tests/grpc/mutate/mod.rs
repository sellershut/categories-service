use anyhow::Result;
use sellershut_core::categories::{Category, GetCategoryRequest, UpsertCategoryRequest};
use sqlx::PgPool;
use tonic::IntoRequest;

use crate::helpers::TestApp;

#[sqlx::test(fixtures(path = "../.././fixtures", scripts("categories")))]
async fn create_category(pool: PgPool) -> Result<()> {
    let mut app = TestApp::new(pool).await;

    let category = Category {
        name: "Something".into(),
        ap_id: format!("http://localhost"),
        local: true,
        ..Default::default()
    };

    let category_request = UpsertCategoryRequest {
        category: Some(category),
        ..Default::default()
    };

    let response = app
        .mutate
        .create(category_request.into_request())
        .await?
        .into_inner()
        .id;

    let getter = GetCategoryRequest { id: response }.into_request();

    let response = app.query.category_by_id(getter).await;

    assert!(response.is_ok());

    Ok(())
}
