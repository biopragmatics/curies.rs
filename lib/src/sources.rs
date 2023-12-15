use crate::{error::CuriesError, Converter};

/// Get the latest OBO Foundry context.
pub async fn get_obo_converter() -> Result<Converter, CuriesError> {
    Converter::from_jsonld("http://purl.obolibrary.org/meta/obo_context.jsonld").await
}
