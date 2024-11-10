pub fn get_technologies_from_string(technologies: &str) -> Vec<String> {
    technologies
        .split(',')
        .map(|s| s.to_string())
        .filter(|s| !s.trim().is_empty())
        .collect()
}
