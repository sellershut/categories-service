use nanoid::nanoid;
use sellershut_core::categories::Category;
use tracing::{error, trace};
use url::Url;
/// Alphabet of characters making up an ID
const ID_ALPHABET: [char; 36] = [
    '2', '3', '4', '5', '6', '7', '8', '9', '_', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '-',
];

/// Length of characters in ID
pub const ID_LENGTH: usize = 21;

/// Generates a nanoid (21 characters)
pub fn generate_id() -> String {
    let id = nanoid!(ID_LENGTH, &ID_ALPHABET);
    trace!(id = id, "id generated");
    id
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_in_id(character: char, expected_result: bool) -> String {
        let id = generate_id();
        let actual_result = id.contains(character);
        assert_eq!(expected_result, actual_result);
        id
    }

    #[test]
    fn check_valid() {
        check_in_id('1', false);
        let id = check_in_id('0', false);

        assert_eq!(ID_LENGTH, id.len());
    }
}

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
