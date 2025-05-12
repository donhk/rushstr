pub fn prepare_string(text: &str) -> String {
    text.trim().replace("\\s+", "\\s")
}
