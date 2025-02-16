use anyhow::Result;
use sellershut_core::categories::{Category, UpsertCategoryRequest};
use sqlx::PgPool;
use tonic::IntoRequest;

use crate::helpers::TestApp;

#[sqlx::test(fixtures(path = "../.././fixtures", scripts("categories")))]
async fn create_user(pool: PgPool) -> Result<()> {
    let mut app = TestApp::new(pool).await;

    let category = Category {
        name: "Something".into(),
        ap_id: format!("http://localhost"),
        local: true,
        ..Default::default()
    };

    let user_request = UpsertCategoryRequest {
        category: Some(category),
        ..Default::default()
    };

    let response = app.mutate.create(user_request.into_request()).await;

    assert!(response.is_ok());

    Ok(())
}
