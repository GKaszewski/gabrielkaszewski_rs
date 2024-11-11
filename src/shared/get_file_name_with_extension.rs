use axum::extract::multipart::Field;
use loco_rs::prelude::*;

pub fn get_file_name_with_extension_from_field(
    field: &Field<'_>,
    default_extension: &str,
) -> Result<(String, String)> {
    let file_name = field.file_name().ok_or_else(|| ModelError::Any("Failed to get file name".into()))?;
    let mut parts = file_name.split('.').collect::<Vec<&str>>();
    let extension = parts.pop().unwrap_or(default_extension);
    let file_name = parts.join(".");
    Ok((file_name, extension.to_string()))
}