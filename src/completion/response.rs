pub fn valid(prefix: &str, candidate: &str) -> bool {
    candidate.starts_with(prefix)
        && candidate.len() > prefix.len()
}