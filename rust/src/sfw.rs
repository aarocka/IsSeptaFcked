pub fn is_sfw(host: &str) -> bool {
    host.contains("isseptafcked.com") || host.contains("localhost")
}

pub fn filter(text: &str) -> String {
    text.replace("fuck", "fck")
        .replace("Fuck", "Fck")
        .replace("FUCK", "FCK")
}
