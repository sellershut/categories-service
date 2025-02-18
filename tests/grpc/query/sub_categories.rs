use anyhow::Result;
use sellershut_core::{
    categories::GetSubCategoriesRequest,
    common::pagination::{
        cursor::{self, cursor_value::CursorType, CursorValue, Index},
        Cursor,
    },
};
use sqlx::PgPool;
use tonic::IntoRequest;

use crate::helpers::TestApp;

#[sqlx::test(fixtures(path = "../.././fixtures", scripts("categories")))]
async fn sub_no_parent_last(pool: PgPool) -> Result<()> {
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

    assert_eq!(response.edges.len(), 2);

    let first = &response.edges[0];
    assert_eq!(first.node.as_ref().unwrap().id, "3");
    let second = &response.edges[1];
    assert_eq!(second.node.as_ref().unwrap().id, "2");

    Ok(())
}

#[sqlx::test(fixtures(path = "../.././fixtures", scripts("categories")))]
async fn sub_no_parent_first(pool: PgPool) -> Result<()> {
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
    //    dbg!(&response);

    assert_eq!(response.edges.len(), 2);

    let first = &response.edges[0];
    assert_eq!(first.node.as_ref().unwrap().id, "1");
    let second = &response.edges[1];
    assert_eq!(second.node.as_ref().unwrap().id, "2");

    Ok(())
}

#[sqlx::test(fixtures(path = "../.././fixtures", scripts("categories")))]
async fn sub_no_parent_first_lots(pool: PgPool) -> Result<()> {
    let mut app = TestApp::new(pool).await;

    let getter = GetSubCategoriesRequest {
        id: None,
        pagination: Some(Cursor {
            cursor_value: None,
            index: Some(Index::First(500)),
        }),
    }
    .into_request();

    let response = app.query.sub_categories(getter).await?.into_inner();
    //   dbg!(&response);

    assert_eq!(response.edges.len(), 3);
    let first = &response.edges[0];
    assert_eq!(first.node.as_ref().unwrap().id, "1");
    let second = &response.edges[1];
    assert_eq!(second.node.as_ref().unwrap().id, "2");
    let third = &response.edges[2];
    assert_eq!(third.node.as_ref().unwrap().id, "3");

    Ok(())
}

#[sqlx::test(fixtures(path = "../.././fixtures", scripts("categories")))]
async fn sub_no_parent_last_exact(pool: PgPool) -> Result<()> {
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

    let first = &response.edges[0];
    assert_eq!(first.node.as_ref().unwrap().id, "3");
    let second = &response.edges[1];
    assert_eq!(second.node.as_ref().unwrap().id, "2");

    Ok(())
}

#[sqlx::test(fixtures(path = "../.././fixtures", scripts("categories")))]
async fn sub_cursor_last(pool: PgPool) -> Result<()> {
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

    let first = &response.edges[0];
    assert_eq!(first.node.as_ref().unwrap().id, "3");
    let second = &response.edges[1];
    assert_eq!(second.node.as_ref().unwrap().id, "2");

    let cursor = response.page_info.unwrap().end_cursor.unwrap();

    let getter = GetSubCategoriesRequest {
        id: None,
        pagination: Some(Cursor {
            cursor_value: Some(CursorValue {
                cursor_type: Some(CursorType::Before(cursor)),
            }),
            index: Some(cursor::Index::Last(1)),
        }),
    };

    let response = app.query.sub_categories(getter).await?.into_inner();
    assert_eq!(response.edges.len(), 1);
    let first = &response.edges[0];
    assert_eq!(first.node.as_ref().unwrap().id, "1");

    Ok(())
}

#[sqlx::test(fixtures(path = "../.././fixtures", scripts("categories")))]
async fn sub_no_index(pool: PgPool) -> Result<()> {
    let mut app = TestApp::new(pool).await;

    let getter = GetSubCategoriesRequest {
        id: None,
        pagination: Some(Cursor {
            cursor_value: None,
            index: None,
        }),
    }
    .into_request();

    let response = app.query.sub_categories(getter).await;

    // no pagination index
    assert!(response.is_err());

    Ok(())
}

#[sqlx::test(fixtures(path = "../.././fixtures", scripts("categories")))]
async fn sub_cursor_first(pool: PgPool) -> Result<()> {
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

    assert_eq!(response.edges.len(), 2);

    let first = &response.edges[0];
    assert_eq!(first.node.as_ref().unwrap().id, "1");
    let second = &response.edges[1];
    assert_eq!(second.node.as_ref().unwrap().id, "2");

    let cursor = response.page_info.unwrap().end_cursor.unwrap();
    dbg!("cursor", &cursor);

    let getter = GetSubCategoriesRequest {
        id: None,
        pagination: Some(Cursor {
            cursor_value: Some(CursorValue {
                cursor_type: Some(CursorType::After(cursor)),
            }),
            index: Some(cursor::Index::First(1)),
        }),
    };

    let response = app.query.sub_categories(getter).await?.into_inner();
    assert_eq!(response.edges.len(), 1);

    let first = &response.edges[0];
    assert_eq!(first.node.as_ref().unwrap().id, "3");

    Ok(())
}
