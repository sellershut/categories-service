use sellershut_core::categories::Category;
use tracing::error;
use url::Url;

pub fn validate_input(category: &Category) -> Result<(), tonic::Status> {
    if let Some(ref url) = category.image_url {
        check_url(url)?;
    }

    if let Some(ref url) = category.parent_id {
        check_url(url)?;
    }

    for c in category.sub_categories.iter() {
        check_url(c)?;
    }
    Ok(())
}

pub fn check_url(value: &str) -> Result<Url, tonic::Status> {
    Url::parse(value).map_err(|_e| {
        let msg = "image_url is not a valid url";
        error!(msg);
        tonic::Status::failed_precondition(msg)
    })
}
