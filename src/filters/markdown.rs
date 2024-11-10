use std::collections::HashMap;

pub fn markdown_filter(value: &tera::Value, _: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    let value = tera::from_value::<String>(value.clone())?;
    let md = markdown::to_html(&value);
    Ok(tera::to_value(md)?)
}