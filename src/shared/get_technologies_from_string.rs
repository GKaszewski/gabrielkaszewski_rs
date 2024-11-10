pub fn get_technologies_from_string(technologies: &str) -> Vec<String> {
    technologies
        .split(',')
        .map(|s| s.to_string())
        .filter(|s| !s.trim().is_empty())
        .collect()
}

pub fn convert_technologies_to_string(technologies: Vec<String>) -> String {
    technologies.join(", ")
}