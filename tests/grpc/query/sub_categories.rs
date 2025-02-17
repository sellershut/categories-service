use anyhow::Result;
use sellershut_core::{
    categories::GetSubCategoriesRequest,
    common::pagination::{cursor::Index, Cursor},
};
use sqlx::PgPool;
use tonic::IntoRequest;

use crate::helpers::TestApp;

#[sqlx::test(fixtures(path = "../.././fixtures", scripts("categories")))]
async fn no_parent_last(pool: PgPool) -> Result<()> {
    let mut app = TestApp::new(pool).await;

    let getter = GetSubCategoriesRequest {
        id: None,
        pagination: Some(Cursor {
            cursor_value: None,
            index: Some(Index::Last(2)),
        }),
    }
    .into_request();

    let response = app.query.sub_categories(getter).await?.into_inner();
    dbg!(&response);

    assert_eq!(response.edges.len(), 2);

    Ok(())
}

#[sqlx::test(fixtures(path = "../.././fixtures", scripts("categories")))]
async fn no_parent_first(pool: PgPool) -> Result<()> {
    let mut app = TestApp::new(pool).await;

    let getter = GetSubCategoriesRequest {
        id: None,
        pagination: Some(Cursor {
            cursor_value: None,
            index: Some(Index::First(2)),
        }),
    }
    .into_request();

    let response = app.query.sub_categories(getter).await?.into_inner();
    dbg!(&response);

    assert_eq!(response.edges.len(), 2);

    Ok(())
}
