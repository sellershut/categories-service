use sellershut_core::{
    categories::{
        mutate_categories_server::MutateCategories, Category, DeleteCategoryRequest,
        UpsertCategoryRequest,
    },
    google::protobuf::Empty,
};
use tonic::{Request, Response, Status};
use tracing::{debug_span, Instrument};

use crate::{entity, utils::generate_id};

use super::AppState;

#[tonic::async_trait]
impl MutateCategories for AppState {
    #[doc = " Create a category"]
    #[tracing::instrument(skip(self), err(Debug))]
    async fn create(
        &self,
        request: tonic::Request<UpsertCategoryRequest>,
    ) -> Result<tonic::Response<Category>, Status> {
        let category = request
            .into_inner()
            .category
            .ok_or_else(|| Status::data_loss("expected category to be available"))?;
        let id = generate_id();

        // Check if the value fits within the range of i64
        let category = sqlx::query_as!(
            entity::Category,
            "insert into category (id, name, sub_categories, image_url, parent_id, local, ap_id)
                values ($1, $2, $3, $4, $5, $6, $7) returning *",
            &id,
            &category.name,
            &category.sub_categories,
            category.image_url,
            category.parent_id,
            category.local,
            category.ap_id,
        )
        .fetch_one(&self.services.postgres)
        .instrument(debug_span!("pg.insert"))
        .await
        .map_err(|e| tonic::Status::internal(e.to_string()))?;

        let category = Category::from(category);

        Ok(tonic::Response::new(category))
    }

    #[doc = " Update a category"]
    #[must_use]
    #[tracing::instrument(skip(self), err(Debug))]
    async fn update(
        &self,
        _request: Request<UpsertCategoryRequest>,
    ) -> Result<Response<Category>, Status> {
        todo!()
    }

    #[doc = " Delete a category"]
    #[must_use]
    #[tracing::instrument(skip(self), err(Debug))]
    async fn delete(
        &self,
        _request: Request<DeleteCategoryRequest>,
    ) -> Result<Response<Empty>, Status> {
        todo!()
    }
}
