use anyhow::Result;
use sellershut_core::categories::{Category, GetCategoryRequest, UpsertCategoryRequest};
use sqlx::PgPool;
use tonic::IntoRequest;

use crate::helpers::TestApp;

#[sqlx::test(fixtures(path = "../.././fixtures", scripts("categories")))]
async fn upsert_category_create(pool: PgPool) -> Result<()> {
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
        .upsert(category_request.into_request())
        .await?
        .into_inner()
        .category
        .unwrap();

    let getter = GetCategoryRequest {
        ap_id: response.ap_id,
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

#[sqlx::test(fixtures(path = "../.././fixtures", scripts("categories")))]
async fn upsert_category_update(pool: PgPool) -> Result<()> {
    let mut app = TestApp::new(pool).await;

    let name = "Electro-test";
    let category = Category {
        name: name.into(),
        ap_id: format!("http://localhost/category/cat1"),
        local: true,
        ..Default::default()
    };

    let category_request = UpsertCategoryRequest {
        category: Some(category),
        ..Default::default()
    };

    let response = app
        .mutate
        .upsert(category_request.into_request())
        .await?
        .into_inner()
        .category
        .unwrap();

    let getter = GetCategoryRequest {
        ap_id: response.ap_id,
    }
    .into_request();

    let response = app
        .query
        .category_by_ap_id(getter)
        .await?
        .into_inner()
        .category
        .unwrap();

    assert_eq!(response.name, name);

    Ok(())
}
