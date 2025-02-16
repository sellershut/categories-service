use sellershut_core::{
    categories::{
        query_categories_server::QueryCategories, Category, Connection, GetCategoryRequest,
        GetSubCategoriesRequest,
    },
    common::pagination,
};
use tonic::{Request, Response, Status};

use super::AppState;

#[tonic::async_trait]
impl QueryCategories for AppState {
    #[doc = " gets all categories"]
    #[must_use]
    #[tracing::instrument(skip(self), err(Debug))]
    async fn categories(
        &self,
        _request: Request<pagination::Cursor>,
    ) -> Result<Response<Connection>, Status> {
        todo!()
    }

    #[doc = " get category by id"]
    #[must_use]
    #[tracing::instrument(skip(self), err(Debug))]
    async fn category_by_id(
        &self,
        _request: tonic::Request<GetCategoryRequest>,
    ) -> Result<tonic::Response<Category>, tonic::Status> {
        todo!()
    }

    #[doc = " get subcategories"]
    #[must_use]
    #[tracing::instrument(skip(self), err(Debug))]
    async fn sub_categories(
        &self,
        _request: Request<GetSubCategoriesRequest>,
    ) -> Result<Response<Connection>, Status> {
        todo!()
    }
}
