use sellershut_core::{
    categories::{
        mutate_categories_server::MutateCategories, CreateCategoryRequest, CreateCategoryResponse,
        DeleteCategoryRequest, UpsertCategoryRequest, UpsertCategoryResponse,
    },
    google::protobuf::Empty,
};
use tonic::{Request, Response, Status};
use tracing::{debug_span, Instrument};

use crate::{
    entity,
    state::query::prepare_single_category,
    utils::{generate_id, validate_input},
};

use super::AppState;

#[tonic::async_trait]
impl MutateCategories for AppState {
    #[doc = " Create a category"]
    #[tracing::instrument(skip(self), err(Debug))]
    async fn create(
        &self,
        request: tonic::Request<CreateCategoryRequest>,
    ) -> Result<tonic::Response<CreateCategoryResponse>, Status> {
        let category = request
            .into_inner()
            .category
            .ok_or_else(|| Status::data_loss("expected category to be available"))?;
        tracing::trace!(id = %category.ap_id, name = %category.name, "creating category");

        validate_input(&category)?;

        let id = generate_id();

        let category = sqlx::query_as!(
            entity::Category,
            "
            with inserted as (
                insert into category (id, name, sub_categories, image_url, parent_id, local, ap_id)
                values ($1, $2, $3, $4, $5, $6, $7) returning *
            )
            select
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
            from inserted c
            left join lateral (
                select ap_id, name
                from category sub
                where sub.ap_id = any(c.sub_categories)
            ) as subcategory on true
                where c.id = $1 and local = $6",
            &id,
            &category.name,
            &category.sub_categories,
            category.image_url,
            category.parent_id,
            category.local,
            category.ap_id,
        )
        .fetch_all(&self.services.postgres)
        .instrument(debug_span!("pg.insert"))
        .await
        .map_err(|e| tonic::Status::internal(e.to_string()))?;

        let category = prepare_single_category(category)?;

        tracing::debug!(id = %category.ap_id, "category created");

        Ok(tonic::Response::new(CreateCategoryResponse {
            category: Some(category),
        }))
    }

    #[doc = " Upsert a category"]
    #[must_use]
    #[tracing::instrument(skip(self), err(Debug))]
    async fn upsert(
        &self,
        request: Request<UpsertCategoryRequest>,
    ) -> Result<Response<UpsertCategoryResponse>, Status> {
        let data = request
            .into_inner()
            .category
            .ok_or_else(|| Status::data_loss("expected category to be available"))?;
        tracing::trace!(id = %data.ap_id, name = %data.name, "upserting category");

        validate_input(&data)?;

        let id = generate_id();

        let category = sqlx::query_as!(
            entity::Category,
            "
            with inserted as (
                insert into category (id, name, sub_categories, image_url, parent_id, local, ap_id)
                values ($1, $2, $3, $4, $5, $6, $7)
                on conflict (ap_id)
                do update 
                set name = excluded.name,
                sub_categories = excluded.sub_categories,
                image_url = excluded.image_url,
                parent_id = excluded.parent_id,
                id = excluded.id,
                local = excluded.local
                returning *
            )
            select
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
            from inserted c
            left join lateral (
                select ap_id, name
                from category sub
                where sub.ap_id = any(c.sub_categories)
            ) as subcategory on true
                where c.ap_id = $7 and local = $6
            ",
            id,
            &data.name,
            &data.sub_categories,
            data.image_url,
            data.parent_id,
            &data.local,
            data.ap_id,
        )
        .fetch_all(&self.services.postgres)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;
        println!("{category:#?}");

        let category = prepare_single_category(category)?;

        tracing::debug!(id = %data.ap_id, name = %category.name, "category upserted");

        Ok(Response::new(UpsertCategoryResponse {
            category: Some(category),
        }))
    }

    #[doc = " Delete a category"]
    #[must_use]
    #[tracing::instrument(skip(self), err(Debug))]
    async fn delete(
        &self,
        request: Request<DeleteCategoryRequest>,
    ) -> Result<Response<Empty>, Status> {
        let id = request.into_inner().ap_id;
        tracing::trace!(id = id, "deleting category");

        sqlx::query!(
            "
            delete from category where ap_id = $1",
            &id
        )
        .execute(&self.services.postgres)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(Empty::default()))
    }
}
