use std::{collections::HashMap, error::Error};

use sellershut_core::{
    categories::{
        query_categories_server::QueryCategories, Category, Connection, GetCategoryRequest,
        GetCategoryResponse, GetSubCategoriesRequest, Node, SubCategory,
    },
    common::pagination::{
        self,
        cursor::{cursor_value::CursorType, Index},
        Cursor, CursorBuilder, PageInfo,
    },
};
use time::{format_description::well_known::Rfc3339, OffsetDateTime, UtcOffset};
use tonic::{Request, Response, Status};
use tracing::{debug, debug_span, Instrument};

use crate::entity;

use super::AppState;

#[tonic::async_trait]
impl QueryCategories for AppState {
    #[doc = " gets all categories"]
    #[must_use]
    #[tracing::instrument(skip(self), err(Debug))]
    async fn categories(
        &self,
        request: Request<pagination::Cursor>,
    ) -> Result<Response<Connection>, Status> {
        let pagination = request.into_inner();

        let max = self.config.max_query_results;

        // get count
        let actual_count = pagination::query_count(
            max,
            &pagination.index.ok_or_else(|| {
                tonic::Status::new(tonic::Code::Internal, "missing pagination index")
            })?,
        );

        let get_count: i64 = actual_count as i64 + 1;

        // a cursor was provided, so we are skipping to somewhere
        let connection = if let Some(ref cursor) = pagination.cursor_value {
            let cursor_value = cursor
                .cursor_type
                .as_ref()
                .expect("cursor value is missing");

            let cursor = decode_cursor(cursor_value)?;

            let created_at = OffsetDateTime::parse(cursor.dt(), &Rfc3339)
                .map_err(|e| tonic::Status::internal(e.to_string()))?;

            let id = cursor.id();
            let (count, categories) = match cursor_value {
                CursorType::After(_cursor) => {
                    paginate_categories_after(self, &created_at, id, get_count).await?
                }
                CursorType::Before(_cursor) => {
                    paginate_categories_before(self, &created_at, id, get_count).await?
                }
            };

            let categories: Vec<_> = prepare_categories(categories).values().cloned().collect();

            parse_categories(count, categories, &pagination, actual_count)?
        } else {
            let index = match pagination.index.expect("index to be available") {
                pagination::cursor::Index::First(count) => Index::First(count),
                pagination::cursor::Index::Last(count) => Index::Last(count),
            };

            let categories = match index {
                Index::First(_) => sqlx::query_as!(
                    entity::Category,
                    "select
                        c.id as id,
                        c.name as name,
                        c.image_url as image_url,
                        c.ap_id as ap_id,
                        c.local as local,
                        c.created_at as created_at,
                        c.parent_id as parent_id,
                        c.updated_at as updated_at,
                        subcategory.ap_id AS sub_category_ap_id,
                        subcategory.name AS sub_category_name
                    from category c
                    left join lateral (
                        select ap_id, name
                        from category sub
                        where sub.ap_id = any(c.sub_categories)
                    ) as subcategory on true
                            where c.local = $1
                            order by
                                c.created_at asc
                            limit $2",
                    true,
                    get_count,
                )
                .fetch_all(&self.services.postgres)
                .instrument(debug_span!("pg.select.*"))
                .await
                .map_err(map_err)?,
                Index::Last(_) => sqlx::query_as!(
                    entity::Category,
                    "select
                        c.id as id,
                        c.name as name,
                        c.image_url as image_url,
                        c.ap_id as ap_id,
                        c.local as local,
                        c.created_at as created_at,
                        c.parent_id as parent_id,
                        c.updated_at as updated_at,
                        subcategory.ap_id AS sub_category_ap_id,
                        subcategory.name AS sub_category_name
                    from category c
                    left join lateral (
                        select ap_id, name
                        from category sub
                        where sub.ap_id = any(c.sub_categories)
                    ) as subcategory on true
                            where local = $1
                            order by
                                created_at desc
                            limit $2",
                    true,
                    get_count,
                )
                .fetch_all(&self.services.postgres)
                .instrument(debug_span!("pg.select.*"))
                .await
                .map_err(map_err)?,
            };

            let categories: Vec<_> = prepare_categories(categories).values().cloned().collect();

            parse_categories(
                Some(get_count - categories.len() as i64),
                categories,
                &pagination,
                actual_count,
            )?
        };

        Ok(tonic::Response::new(connection))
    }

    #[doc = " get category by id"]
    #[must_use]
    #[tracing::instrument(skip(self), err(Debug))]
    async fn category_by_ap_id(
        &self,
        request: tonic::Request<GetCategoryRequest>,
    ) -> Result<tonic::Response<GetCategoryResponse>, tonic::Status> {
        let id = request.into_inner().ap_id;
        debug!(id = id, "getting by ap_id");
        let category = sqlx::query_as!(
            entity::Category,
            "select
                c.id as id,
                c.name as name,
                c.image_url as image_url,
                c.ap_id as ap_id,
                c.local as local,
                c.created_at as created_at,
                c.parent_id as parent_id,
                c.updated_at as updated_at,
                subcategory.ap_id AS \"sub_category_ap_id?\",
                subcategory.name AS \"sub_category_name?\"
            from category c
            left join lateral (
                select ap_id, name
                from category sub
                where sub.ap_id = any(c.sub_categories)
            ) as subcategory on true
                where c.ap_id = $1 and local = $2",
            id,
            true
        )
        .fetch_all(&self.services.postgres)
        .instrument(debug_span!("pg.select.*"))
        .await
        .map_err(map_err)?;

        let category = prepare_single_category(category)?;

        Ok(tonic::Response::new(GetCategoryResponse {
            category: Some(category),
        }))
    }

    #[doc = " get subcategories"]
    #[must_use]
    #[tracing::instrument(skip(self), err(Debug))]
    async fn sub_categories(
        &self,
        request: Request<GetSubCategoriesRequest>,
    ) -> Result<Response<Connection>, Status> {
        let params = request.into_inner();
        let pagination = params.pagination.expect("pagination params");
        let parent_id = params.id;

        let max = self.config.max_query_results;

        // get count
        let actual_count = pagination::query_count(
            max,
            &pagination.index.ok_or_else(|| {
                tonic::Status::new(tonic::Code::Internal, "missing pagination index")
            })?,
        );

        let get_count: i64 = actual_count as i64 + 1;
        dbg!(get_count);

        // a cursor was provided, so we are skipping to somewhere
        let connection = if let Some(ref cursor) = pagination.cursor_value {
            let cursor_value = cursor
                .cursor_type
                .as_ref()
                .expect("cursor value is missing");

            let cursor = decode_cursor(cursor_value)?;

            let created_at = OffsetDateTime::parse(cursor.dt(), &Rfc3339)
                .map_err(|e| tonic::Status::internal(e.to_string()))?;

            let id = cursor.id();
            let (count, categories) = match cursor_value {
                CursorType::After(_cursor) => {
                    paginate_sub_categories_after(
                        self,
                        &created_at,
                        id,
                        get_count,
                        parent_id.as_deref(),
                    )
                    .await?
                }
                CursorType::Before(_cursor) => {
                    paginate_sub_categories_before(
                        self,
                        &created_at,
                        id,
                        get_count,
                        parent_id.as_deref(),
                    )
                    .await?
                }
            };

            let categories: Vec<_> = prepare_categories(categories).values().cloned().collect();
            parse_categories(count, categories, &pagination, actual_count)?
        } else {
            let index = match pagination.index.expect("index to be available") {
                pagination::cursor::Index::First(count) => Index::First(count),
                pagination::cursor::Index::Last(count) => Index::Last(count),
            };

            let categories = match index {
                Index::First(_) => sqlx::query_as!(
                    entity::Category,
                    "select
                        c.id as id,
                        c.name as name,
                        c.image_url as image_url,
                        c.ap_id as ap_id,
                        c.local as local,
                        c.created_at as created_at,
                        c.parent_id as parent_id,
                        c.updated_at as updated_at,
                        subcategory.ap_id AS \"sub_category_ap_id?\",
                        subcategory.name AS \"sub_category_name?\",
                        subcategory.id AS \"sub_category_id?\",
                        subcategory.image_url AS \"sub_category_image_url?\",
                        subcategory.local AS \"sub_category_local?\",
                        subcategory.created_at AS \"sub_category_created_at?\",
                        subcategory.updated_at AS \"sub_category_updated_at?\",
                        subcategory.parent_id AS \"sub_category_parent_id?\"
                    from category c
                    left join lateral (
                        select ap_id, name, sub_categories, image_url, parent_id, created_at, updated_at, local, id
                        from category sub
                        where sub.ap_id = any(c.sub_categories)
                    ) as subcategory on true
                        where 
                            (($2::text is not null and c.parent_id = $2) or c.parent_id is null)
                            and c.local = $3
                        order by
                            c.created_at asc
                        limit $1",
                    get_count,
                    parent_id,
                    true
                )
                .fetch_all(&self.services.postgres)
                .instrument(debug_span!("pg.select.count"))
                .await
                .map_err(map_err)?,
                Index::Last(_) => sqlx::query_as!(
                    entity::Category,
                    "select
                        c.id as id,
                        c.name as name,
                        c.image_url as image_url,
                        c.ap_id as ap_id,
                        c.local as local,
                        c.created_at as created_at,
                        c.parent_id as parent_id,
                        c.updated_at as updated_at,
                        subcategory.ap_id AS \"sub_category_ap_id?\",
                        subcategory.name AS \"sub_category_name?\"
                    from category c
                    left join lateral (
                        select ap_id, name
                        from category sub
                        where sub.ap_id = any(c.sub_categories)
                    ) as subcategory on true
                        where
                            (($2::text is not null and c.parent_id = $2) or c.parent_id is null)
                             and c.local = $3
                        order by
                            c.created_at desc
                        limit $1",
                    get_count,
                    parent_id,
                    true
                )
                .fetch_all(&self.services.postgres)
                .instrument(debug_span!("pg.select.*"))
                .await
                .map_err(map_err)?,
            };


            let categories: Vec<_> = prepare_categories(categories).values().cloned().collect();
            println!("post: {categories:#?}");
            parse_categories(
                Some(get_count - categories.len() as i64),
                categories,
                &pagination,
                actual_count,
            )?
        };

        Ok(tonic::Response::new(connection))
    }
}

async fn paginate_sub_categories_before(
    state: &AppState,
    created_at: &OffsetDateTime,
    id: &str,
    get_count: i64,
    parent_id: Option<&str>,
) -> Result<(Option<i64>, Vec<entity::Category>), tonic::Status> {
    let fut_count = sqlx::query_scalar!(
        "
            select count(*) from category
            where 
                (((
                    created_at <> $1
                    or id > $2
                )
                and created_at >= $1) and (($3::text is not null and parent_id = $3) or parent_id is null)) and local = $4
        ",
        created_at,
        id,
        parent_id,
        true
    )
    .fetch_one(&state.services.postgres)
    .instrument(debug_span!("pg.select.count"));

    let fut_categories = sqlx::query_as!(
        entity::Category,
            "select
                c.id as id,
                c.name as name,
                c.image_url as image_url,
                c.ap_id as ap_id,
                c.local as local,
                c.created_at as created_at,
                c.parent_id as parent_id,
                c.updated_at as updated_at,
                subcategory.ap_id AS \"sub_category_ap_id?\",
                subcategory.name AS \"sub_category_name?\"
            from category c
            left join lateral (
                select ap_id, name
                from category sub
                where sub.ap_id = any(c.sub_categories)
            ) as subcategory on true
            where 
                (((
                    c.created_at = $1
                    and c.id < $2
                )
                or c.created_at < $1) and (($4::text is not null and c.parent_id = $4) or c.parent_id is null)) and c.local = $5
            order by
                c.created_at desc,
                c.id desc
            limit
                $3
        ",
        created_at,
        id,
        get_count,
        parent_id,
        true
    )
    .fetch_all(&state.services.postgres)
    .instrument(debug_span!("pg.select.*"));

    tokio::try_join!(fut_count, fut_categories).map_err(map_err)
}

async fn paginate_sub_categories_after(
    state: &AppState,
    created_at: &OffsetDateTime,
    id: &str,
    get_count: i64,
    parent_id: Option<&str>,
) -> Result<(Option<i64>, Vec<entity::Category>), tonic::Status> {
    let fut_count = sqlx::query_scalar!(
        "
            select count(*) from category
            where 
                (((
                    created_at <> $1
                    or id <= $2
                )
                and created_at < $1) and (($3::text is not null and parent_id = $3) or parent_id is null)) and local = $4
        ",
        created_at,
        id,
        parent_id,
        true
    )
    .fetch_one(&state.services.postgres)
    .instrument(debug_span!("pg.select.count"));

    let fut_categories = sqlx::query_as!(
        entity::Category,
            "select
                c.id as id,
                c.name as name,
                c.image_url as image_url,
                c.ap_id as ap_id,
                c.local as local,
                c.created_at as created_at,
                c.parent_id as parent_id,
                c.updated_at as updated_at,
                subcategory.ap_id AS \"sub_category_ap_id?\",
                subcategory.name AS \"sub_category_name?\"
            from category c
            left join lateral (
                select ap_id, name
                from category sub
                where sub.ap_id = any(c.sub_categories)
            ) as subcategory on true
            where 
                (((
                    c.created_at = $1
                    and c.id > $2
                )
                or c.created_at >= $1) and (($4::text is not null and c.parent_id = $4) or c.parent_id is null)) and c.local = $5
            order by
                c.created_at asc,
                c.id asc
            limit
                $3
        ",
        created_at,
        id,
        get_count,
        parent_id,
        true
    )
    .fetch_all(&state.services.postgres)
    .instrument(debug_span!("pg.select.*"));

    tokio::try_join!(fut_count, fut_categories).map_err(map_err)
}

async fn paginate_categories_before(
    state: &AppState,
    created_at: &OffsetDateTime,
    id: &str,
    get_count: i64,
) -> Result<(Option<i64>, Vec<entity::Category>), tonic::Status> {
    let fut_count = sqlx::query_scalar!(
        "
            select count(*) from category
            where 
                ((
                    created_at <> $1
                    or id > $2
                )
                and created_at >= $1) and local = $3
        ",
        created_at,
        id,
        true
    )
    .fetch_one(&state.services.postgres)
    .instrument(debug_span!("pg.select.count"));

    let fut_categories = sqlx::query_as!(
        entity::Category,
        "select
                c.id as id,
                c.name as name,
                c.image_url as image_url,
                c.ap_id as ap_id,
                c.local as local,
                c.created_at as created_at,
                c.parent_id as parent_id,
                c.updated_at as updated_at,
                subcategory.ap_id AS \"sub_category_ap_id?\",
                subcategory.name AS \"sub_category_name?\"
            from category c
            left join lateral (
                select ap_id, name
                from category sub
                where sub.ap_id = any(c.sub_categories)
            ) as subcategory on true
            where 
                ((
                    c.created_at = $1
                    and c.id < $2
                )
                or c.created_at < $1) and c.local = $4
            order by
                c.created_at desc,
                c.id desc
            limit
                $3
        ",
        created_at,
        id,
        get_count,
        true
    )
    .fetch_all(&state.services.postgres)
    .instrument(debug_span!("pg.select.*"));

    tokio::try_join!(fut_count, fut_categories).map_err(map_err)
}

async fn paginate_categories_after(
    state: &AppState,
    created_at: &OffsetDateTime,
    id: &str,
    get_count: i64,
) -> Result<(Option<i64>, Vec<entity::Category>), tonic::Status> {
    let fut_count = sqlx::query_scalar!(
        "
            select count(*) from category
            where 
                ((
                    created_at <> $1
                    or id <= $2
                )
                and created_at < $1) and local = $3
        ",
        created_at,
        id,
        true
    )
    .fetch_one(&state.services.postgres)
    .instrument(debug_span!("pg.select.count"));

    let fut_categories = sqlx::query_as!(
        entity::Category,
        "select
                c.id as id,
                c.name as name,
                c.image_url as image_url,
                c.ap_id as ap_id,
                c.local as local,
                c.created_at as created_at,
                c.parent_id as parent_id,
                c.updated_at as updated_at,
                subcategory.ap_id AS \"sub_category_ap_id?\",
                subcategory.name AS \"sub_category_name?\"
            from category c
            left join lateral (
                select ap_id, name
                from category sub
                where sub.ap_id = any(c.sub_categories)
            ) as subcategory on true
            where 
                ((
                    c.created_at = $1
                    and c.id > $2
                )
                or c.created_at >= $1) and c.local = $4
            order by
                c.created_at asc,
                c.id asc
            limit
                $3
        ",
        created_at,
        id,
        get_count,
        true
    )
    .fetch_all(&state.services.postgres)
    .instrument(debug_span!("pg.select.*"));

    tokio::try_join!(fut_count, fut_categories).map_err(map_err)
}

fn decode_cursor(cursor_value: &CursorType) -> Result<CursorBuilder, Status> {
    CursorBuilder::decode(cursor_value).map_err(|e| tonic::Status::internal(e.to_string()))
}

fn map_err(err: impl Error) -> tonic::Status {
    tonic::Status::new(tonic::Code::Internal, err.to_string())
}

fn parse_categories(
    count_on_other_end: Option<i64>,
    categories: Vec<Category>,
    pagination: &Cursor,
    actual_count: i32,
) -> Result<Connection, tonic::Status> {
    let user_count = actual_count as usize;

    let count_on_other_end = count_on_other_end
        .ok_or_else(|| tonic::Status::new(tonic::Code::Internal, "count returned no items"))?;
    let left_side = CursorBuilder::is_paginating_from_left(pagination);

    let len = categories.len();

    let has_more = len > user_count;

    let to_node = |category: Category| -> Result<Node, tonic::Status> {
        let dt = category.created_at.expect("to exist");
        let dt = OffsetDateTime::try_from(dt)
            .map_err(|_| tonic::Status::invalid_argument("timestamp is invalid"))?;

        dt.to_offset(UtcOffset::UTC)
            .format(&Rfc3339)
            .map(|dt| {
                let cursor = CursorBuilder::new(&category.id, &dt);
                Node {
                    node: Some(category),
                    cursor: cursor.encode(),
                }
            })
            .map_err(map_err)
    };

    dbg!(&categories);

    let categories: Result<Vec<_>, _> = if left_side {
        categories
            .into_iter()
            .take(user_count)
            .map(&to_node)
            .collect()
    } else if has_more {
        categories
            .into_iter()
            .rev() // need to take from the right hand side as those
            // are the last ones
            .take(user_count)
            // https://relay.dev/graphql/connections.htm#sel-FAJJDCBEBay8J
            //  .rev() // restore the order
            .map(&to_node)
            .collect()
    } else {
        // restore order from db https://relay.dev/graphql/connections.htm#sel-FAJJDCBEBay8J
        categories.into_iter().rev().map(&to_node).collect()
    };
    dbg!(count_on_other_end);

    let edges = categories?;
    let start = edges.first().map(|f| f.cursor.clone());
    let end = edges.last().map(|f| f.cursor.clone());

    let connection = Connection {
        edges,
        page_info: Some(PageInfo {
            has_next_page: {
                if !left_side {
                    false
                } else {
                    count_on_other_end > 0
                }
            },
            has_previous_page: {
                if left_side {
                    false
                } else {
                    count_on_other_end > 0
                }
            },
            start_cursor: start,
            end_cursor: end,
        }),
    };

    Ok(connection)
}

pub fn prepare_categories(category: Vec<entity::Category>) -> HashMap<String, Category> {
    let mut categories_map = HashMap::new();
    for result in category.iter() {
        let category = categories_map
            .entry(result.ap_id.clone())
            .or_insert(Category {
                id: result.id.clone(),
                name: result.name.to_string(),
                sub_categories: Vec::with_capacity(category.len()),
                image_url: result.image_url.to_owned(),
                parent_id: result.parent_id.to_owned(),
                created_at: Some(result.created_at.into()),
                updated_at: Some(result.updated_at.into()),
                ap_id: result.ap_id.to_string(),
                local: result.local,
            });

        if let (Some(name), Some(ap_id)) = (
            result.sub_category_name.as_ref(),
            result.sub_category_ap_id.as_ref(),
        ) {
            category.sub_categories.push(SubCategory {
                name: name.to_string(),
                ap_id: ap_id.to_string(),
            });
        }
    }

    categories_map
}

pub fn prepare_single_category(category: Vec<entity::Category>) -> Result<Category, Status> {
    let categories_map = prepare_categories(category);

    categories_map
        .into_iter()
        .next()
        .map(|(_, category)| category)
        .ok_or_else(|| tonic::Status::internal("query returned none"))
}
