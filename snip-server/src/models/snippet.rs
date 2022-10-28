impl Snippet {
    pub fn remove_duplicate_tags_and_lowercase(mut tags: Vec<String>) -> Vec<String> {
        tags = tags.iter().map(|s| s.to_lowercase()).collect();
        tags.sort();
        tags.dedup();
        tags
    }
}
