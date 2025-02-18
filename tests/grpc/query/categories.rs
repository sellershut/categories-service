use anyhow::Result;

use sellershut_core::common::pagination::{
    cursor::{self, cursor_value::CursorType, CursorValue, Index},
    Cursor,
};
use sqlx::PgPool;
use tonic::IntoRequest;

use crate::helpers::TestApp;

#[sqlx::test(fixtures(path = "../.././fixtures", scripts("categories")))]
async fn no_parent_last(pool: PgPool) -> Result<()> {
    let mut app = TestApp::new(pool).await;

    let getter = Cursor {
        cursor_value: None,
        index: Some(Index::Last(2)),
    }
    .into_request();

    let response = app.query.categories(getter).await?.into_inner();

    assert_eq!(response.edges.len(), 2);

    Ok(())
}

#[sqlx::test(fixtures(path = "../.././fixtures", scripts("categories")))]
async fn no_parent_first(pool: PgPool) -> Result<()> {
    let mut app = TestApp::new(pool).await;

    let getter = Cursor {
        cursor_value: None,
        index: Some(Index::First(2)),
    }
    .into_request();

    let response = app.query.categories(getter).await?.into_inner();

    assert_eq!(response.edges.len(), 2);

    Ok(())
}

#[sqlx::test(fixtures(path = "../.././fixtures", scripts("categories")))]
async fn no_parent_last_exact(pool: PgPool) -> Result<()> {
    let mut app = TestApp::new(pool).await;

    let getter = Cursor {
        cursor_value: None,
        index: Some(Index::Last(2)),
    }
    .into_request();

    let response = app.query.categories(getter).await?.into_inner();

    assert_eq!(response.edges.len(), 2);

    Ok(())
}

#[sqlx::test(fixtures(path = "../.././fixtures", scripts("categories")))]
async fn cursor_last(pool: PgPool) -> Result<()> {
    let mut app = TestApp::new(pool).await;

    let getter = Cursor {
        cursor_value: None,
        index: Some(Index::Last(2)),
    }
    .into_request();

    let response = app.query.categories(getter).await?.into_inner();
    dbg!(&response);

    assert_eq!(response.edges.len(), 2);
    let cursor = response.page_info.unwrap().end_cursor.unwrap();

    let getter = Cursor {
        cursor_value: Some(CursorValue {
            cursor_type: Some(CursorType::Before(cursor)),
        }),
        index: Some(cursor::Index::Last(1)),
    };

    let response = app.query.categories(getter).await?.into_inner();
    assert_eq!(response.edges.len(), 1);
    Ok(())
}

#[sqlx::test(fixtures(path = "../.././fixtures", scripts("categories")))]
async fn no_index(pool: PgPool) -> Result<()> {
    let mut app = TestApp::new(pool).await;

    let getter = Cursor {
        cursor_value: None,
        index: None,
    }
    .into_request();

    let response = app.query.categories(getter).await;

    // no pagination index
    assert!(response.is_err());

    Ok(())
}

#[sqlx::test(fixtures(path = "../.././fixtures", scripts("categories")))]
async fn cursor_first_sub(pool: PgPool) -> Result<()> {
    let mut app = TestApp::new(pool).await;

    let getter = Cursor {
        cursor_value: None,
        index: Some(Index::First(2)),
    }
    .into_request();

    let response = app.query.categories(getter).await?.into_inner();

    assert_eq!(response.edges.len(), 2);

    let cursor = response.page_info.unwrap().end_cursor.unwrap();

    let getter = Cursor {
        cursor_value: Some(CursorValue {
            cursor_type: Some(CursorType::After(cursor)),
        }),
        index: Some(cursor::Index::First(1)),
    };

    let response = app.query.categories(getter).await?.into_inner();
    assert_eq!(response.edges.len(), 1);

    Ok(())
}
