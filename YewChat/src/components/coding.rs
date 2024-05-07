pub fn get_username(s: &str) -> String {
    s.split("===").next().unwrap_or("").to_string()
}
pub fn get_image(s: &str) -> String {
    s.splitn(2, "===").nth(1).unwrap_or("").to_string()
}
pub fn set_username(s: &str, new_before: &str) -> String {
    let after = get_image(s);
    format!("{}==={}", new_before, after)
}
pub fn set_image(s: &str, new_after: &str) -> String {
    let before = get_username(s);
    format!("{}==={}", before, new_after)
}