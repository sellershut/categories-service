use sellershut_core::{
    categories::{
        mutate_categories_server::MutateCategories, Category, DeleteCategoryRequest,
        UpsertCategoryRequest,
    },
    google::protobuf::Empty,
};
use tonic::{Request, Response, Status};

use super::AppState;

#[tonic::async_trait]
impl MutateCategories for AppState {
    #[doc = " Create a category"]
    #[tracing::instrument(skip(self), err(Debug))]
    async fn create(
        &self,
        _request: tonic::Request<UpsertCategoryRequest>,
    ) -> Result<tonic::Response<Category>, Status> {
        todo!()
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
