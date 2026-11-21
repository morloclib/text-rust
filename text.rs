// Strings are sequences of Unicode scalar values: chars, lengths and padding
// count chars, not bytes. Whitespace is Python's str.isspace set, which is
// Unicode White_Space plus U+001C..U+001F.

fn text_is_space(c: char) -> bool {
    c.is_whitespace() || ('\u{1c}'..='\u{1f}').contains(&c)
}

pub fn text_split(delim: &str, s: &str) -> Vec<String> {
    if delim.is_empty() {
        return text_chars(s);
    }
    s.split(delim).map(String::from).collect()
}

pub fn text_lines(s: &str) -> Vec<String> {
    s.split('\n').map(String::from).collect()
}

pub fn text_words(s: &str) -> Vec<String> {
    s.split(text_is_space)
        .filter(|w| !w.is_empty())
        .map(String::from)
        .collect()
}

pub fn text_chars(s: &str) -> Vec<String> {
    s.chars().map(String::from).collect()
}

pub fn text_join(delim: &str, xs: &[String]) -> String {
    xs.join(delim)
}

pub fn text_unlines(xs: &[String]) -> String {
    xs.join("\n")
}

pub fn text_unwords(xs: &[String]) -> String {
    xs.join(" ")
}

pub fn text_contains(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

pub fn text_startsWith(prefix: &str, s: &str) -> bool {
    s.starts_with(prefix)
}

pub fn text_endsWith(suffix: &str, s: &str) -> bool {
    s.ends_with(suffix)
}

pub fn text_toUpper(s: &str) -> String {
    s.to_uppercase()
}

pub fn text_toLower(s: &str) -> String {
    s.to_lowercase()
}

pub fn text_trim(s: &str) -> String {
    s.trim_matches(text_is_space).to_string()
}

pub fn text_trimStart(s: &str) -> String {
    s.trim_start_matches(text_is_space).to_string()
}

pub fn text_trimEnd(s: &str) -> String {
    s.trim_end_matches(text_is_space).to_string()
}

pub fn text_replace(old: &str, new: &str, s: &str) -> String {
    s.replacen(old, new, 1)
}

pub fn text_replaceAll(old: &str, new: &str, s: &str) -> String {
    s.replace(old, new)
}

fn text_padding(width: i64, fill: &str, s: &str) -> Option<String> {
    let len = s.chars().count() as i64;
    if fill.is_empty() || len >= width {
        return None;
    }
    Some(fill.chars().cycle().take((width - len) as usize).collect())
}

pub fn text_padLeft(width: i64, fill: &str, s: &str) -> String {
    match text_padding(width, fill, s) {
        Some(p) => p + s,
        None => s.to_string(),
    }
}

pub fn text_padRight(width: i64, fill: &str, s: &str) -> String {
    match text_padding(width, fill, s) {
        Some(p) => s.to_string() + &p,
        None => s.to_string(),
    }
}

pub fn text_isBlank(s: &str) -> bool {
    s.chars().all(text_is_space)
}
