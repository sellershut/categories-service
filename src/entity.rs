use serde::Deserialize;
use time::OffsetDateTime;

#[derive(Debug, Deserialize, Clone)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub sub_categories: Vec<String>,
    pub image_url: Option<String>,
    pub parent_id: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub ap_id: String,
    pub local: bool,
}

impl From<Category> for sellershut_core::categories::Category {
    fn from(value: Category) -> Self {
        Self {
            id: value.id,
            name: value.name,
            sub_categories: value.sub_categories,
            image_url: value.image_url,
            parent_id: value.parent_id,
            created_at: Some(value.created_at.into()),
            updated_at: Some(value.updated_at.into()),
            ap_id: value.ap_id,
            local: value.local,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct CategoryDetailed {
    pub id: String,
    pub name: String,
    pub sub_category_name: Option<String>,
    pub sub_category_ap_id: Option<String>,
    pub image_url: Option<String>,
    pub parent_id: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub ap_id: String,
    pub local: bool,
}
