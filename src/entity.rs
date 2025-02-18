use serde::Deserialize;
use time::OffsetDateTime;

#[derive(Debug, Deserialize, Clone)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub sub_category_name: Option<String>,
    pub sub_category_ap_id: Option<String>,
    pub sub_category_parent_id: Option<String>,
    pub sub_category_id: Option<String>,
    pub sub_category_image_url: Option<String>,
    pub sub_category_local: bool,
    pub sub_category_created_at: Option<String>,
    pub sub_category_updated_at: Option<String>,
    pub image_url: Option<String>,
    pub parent_id: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub ap_id: String,
    pub local: bool,
}
