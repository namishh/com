use htmlescape;
use inkjet::{Highlighter, Language, formatter};
use once_cell::sync::Lazy;
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd, html};
use regex::Regex;
use serde_json::Value as JsonValue;
use std::collections::HashSet;
use std::sync::Mutex;

pub static FRONTMATTER_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?s)^-{3,}\s*\n(.*?)\n-{3,}\s*\n(.*)").unwrap());

fn get_inkjet_language(lang_str: &str) -> Option<Language> {
    match lang_str.to_lowercase().as_str() {
        "rust" => Some(Language::Rust),
        "javascript" | "js" => Some(Language::Javascript),
        "typescript" | "ts" => Some(Language::Typescript),
        "python" | "py" => Some(Language::Python),
        "css" => Some(Language::Css),
        "html" => Some(Language::Html),
        "lua" => Some(Language::Lua),
        "jsx" => Some(Language::Jsx),
        "tsx" => Some(Language::Tsx),
        "zig" => Some(Language::Zig),
        "nix" => Some(Language::Nix),
        "glsl" => Some(Language::Glsl),
        _ => None,
    }
}

fn extract_language_and_filename(info_string: &str) -> (Option<String>, Option<String>) {
    let parts: Vec<&str> = info_string.split_whitespace().collect();
    let language = if !parts.is_empty() {
        Some(parts[0].to_string())
    } else {
        None
    };
    let filename = parts
        .iter()
        .find(|part| part.starts_with("title="))
        .and_then(|part| {
            let eq_pos = part.find('=').unwrap_or(0);
            if eq_pos < part.len() - 1 {
                let value = &part[eq_pos + 1..];
                if (value.starts_with('"') && value.ends_with('"'))
                    || (value.starts_with('\'') && value.ends_with('\''))
                {
                    Some(value[1..value.len() - 1].to_string())
                } else {
                    Some(value.to_string())
                }
            } else {
                None
            }
        });
    (language, filename)
}

fn parse_highlighting_info(info_string: &str) -> (HashSet<usize>, HashSet<usize>, HashSet<usize>) {
    let mut del_lines = HashSet::new();
    let mut add_lines = HashSet::new();
    let mut h_lines = HashSet::new();

    static DEL_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"del=\{([^}]+)\}").unwrap());
    static ADD_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"add=\{([^}]+)\}").unwrap());
    static H_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\{([^}]+)\}").unwrap());

    let parse_ranges = |range_str: &str| -> HashSet<usize> {
        let mut result = HashSet::new();
        for part in range_str.split(',') {
            let part = part.trim();
            if part.contains('-') {
                let range: Vec<&str> = part.split('-').collect();
                if range.len() == 2 {
                    if let (Ok(start), Ok(end)) = (
                        range[0].trim().parse::<usize>(),
                        range[1].trim().parse::<usize>(),
                    ) {
                        for i in start..=end {
                            result.insert(i);
                        }
                    }
                }
            } else if let Ok(num) = part.parse::<usize>() {
                result.insert(num);
            }
        }
        result
    };

    if let Some(captures) = DEL_RE.captures(info_string) {
        if let Some(ranges) = captures.get(1) {
            del_lines = parse_ranges(ranges.as_str());
        }
    }
    if let Some(captures) = ADD_RE.captures(info_string) {
        if let Some(ranges) = captures.get(1) {
            add_lines = parse_ranges(ranges.as_str());
        }
    }
    for captures in H_RE.captures_iter(info_string) {
        if let Some(range_match) = captures.get(1) {
            let full_match = captures.get(0).unwrap().as_str();
            if !full_match.starts_with("del=") && !full_match.starts_with("add=") {
                h_lines = parse_ranges(range_match.as_str());
            }
        }
    }
    (del_lines, add_lines, h_lines)
}

pub fn markdown_to_html(
    content: &str,
    highlighter: &Mutex<Highlighter>,
) -> (String, Vec<(u8, String, String)>) {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_MATH);
    let parser = Parser::new_ext(content, options);

    let mut in_code_block = false;
    let mut code_content = String::new();
    let mut current_language = None;
    let mut current_filename = None;
    let mut current_heading: Option<(u8, Vec<Event>)> = None;
    let mut headings = Vec::new();
    let mut current_highlighting: (HashSet<usize>, HashSet<usize>, HashSet<usize>) =
        (HashSet::new(), HashSet::new(), HashSet::new());
    let mut events = Vec::new();

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                in_code_block = true;
                let lang_info = match kind {
                    CodeBlockKind::Fenced(lang) => lang.to_string(),
                    _ => String::new(),
                };
                let (lang, filename) = extract_language_and_filename(&lang_info);
                current_language = lang;
                current_filename = filename;
                current_highlighting = parse_highlighting_info(&lang_info);
                code_content.clear();
            }
            Event::Text(text) if in_code_block => {
                code_content.push_str(&text);
            }
            Event::DisplayMath(text) => {
                let math_html = format!(
                    r#"<div class="math math-display">\[{}\]</div>"#,
                    htmlescape::encode_minimal(&text)
                );
                events.push(Event::Html(math_html.into()));
            }
            Event::InlineMath(text) => {
                let math_html = format!(
                    r#"<span class="math math-inline">\({}\)</span>"#,
                    htmlescape::encode_minimal(&text)
                );
                events.push(Event::Html(math_html.into()));
            }
            Event::End(TagEnd::CodeBlock) if in_code_block => {
                in_code_block = false;
                let highlighted_html = if let Some(lang_str) = current_language.as_ref() {
                    if let Some(inkjet_lang) = get_inkjet_language(lang_str) {
                        match highlighter.lock().unwrap().highlight_to_string(
                            inkjet_lang,
                            &formatter::Html,
                            &code_content,
                        ) {
                            Ok(html) => html,
                            Err(e) => {
                                eprintln!("Error highlighting code: {}", e);
                                htmlescape::encode_minimal(&code_content)
                            }
                        }
                    } else {
                        htmlescape::encode_minimal(&code_content)
                    }
                } else {
                    htmlescape::encode_minimal(&code_content)
                };
                let lines: Vec<&str> = highlighted_html.lines().collect();
                let total_lines = lines.len();
                let width_needed = if total_lines > 0 {
                    total_lines.to_string().len()
                } else {
                    1
                };
                let (del_lines, add_lines, highlight_lines) = &current_highlighting;
                let line_numbered_html = lines
                    .iter()
                    .enumerate()
                    .map(|(i, line)| {
                        let line_num = i + 1;
                        let mut line_class = String::new();
                        if del_lines.contains(&line_num) {
                            line_class = " class=\"highlight-del\"".to_string();
                        } else if add_lines.contains(&line_num) {
                            line_class = " class=\"highlight-add\"".to_string();
                        } else if highlight_lines.contains(&line_num) {
                            line_class = " class=\"highlight\"".to_string();
                        }
                        format!(
                            "<span{line_class}><span class=\"line-number\">{:0width$}</span><span class=\"code-line\">{}</span></span>",
                            line_num,
                            line,
                            width = width_needed,
                            line_class = line_class
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\n");
                let code_html = if let Some(filename) = current_filename.as_ref() {
                    format!(
                        r#"<div class="code-block"><div class="code-header flex items-center justify-end"><span class="code-filename">{}</span><button class="copy-button" onclick="copyCode(this)"><i class="ph ph-copy"></i></button></div><pre><code>{}</code></pre></div>"#,
                        filename, line_numbered_html
                    )
                } else {
                    format!(
                        r#"<div class="code-block"><div class="code-header flex items-center justify-end"><button class="copy-button" onclick="copyCode(this)"><i class="ph ph-copy"></i></button></div><pre><code>{}</code></pre></div>"#,
                        line_numbered_html
                    )
                };
                events.push(Event::Html(code_html.into()));
                current_language = None;
                current_filename = None;
                current_highlighting = (HashSet::new(), HashSet::new(), HashSet::new());
            }
            Event::Start(Tag::Heading { level, .. }) => {
                current_heading = match level {
                    pulldown_cmark::HeadingLevel::H1 => Some((1, Vec::new())),
                    pulldown_cmark::HeadingLevel::H2 => Some((2, Vec::new())),
                    pulldown_cmark::HeadingLevel::H3 => Some((3, Vec::new())),
                    pulldown_cmark::HeadingLevel::H4 => Some((4, Vec::new())),
                    _ => None,
                };
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some((level, inner_events)) = current_heading.take() {
                    let mut text_content = String::new();
                    for e in &inner_events {
                        if let Event::Text(t) = e {
                            text_content.push_str(t);
                        }
                    }
                    let slug = text_content
                        .trim()
                        .to_lowercase()
                        .replace(' ', "-")
                        .replace(|c: char| !c.is_alphanumeric() && c != '-', "");
                    headings.push((level, text_content.clone(), slug.clone()));
                    let mut inner_html = String::new();
                    html::push_html(&mut inner_html, inner_events.into_iter());
                    let heading_html =
                        format!("<h{} id=\"{}\">{}</h{}>", level, slug, inner_html, level);
                    events.push(Event::Html(heading_html.into()));
                }
            }
            _ => {
                if in_code_block {
                    if let Event::Text(text) = event {
                        code_content.push_str(&text);
                    }
                } else if let Some((_, ref mut inner_events)) = current_heading {
                    inner_events.push(event);
                } else {
                    events.push(event);
                }
            }
        }
    }

    let mut html_output = String::new();
    html::push_html(&mut html_output, events.into_iter());
    (html_output, headings)
}

pub fn extract_frontmatter(content: &str) -> (JsonValue, &str) {
    if let Some(caps) = FRONTMATTER_REGEX.captures(content) {
        let yaml_str = caps.get(1).unwrap().as_str();
        let body = caps.get(2).unwrap().as_str().trim_start();
        let yaml: JsonValue = serde_yaml::from_str(yaml_str).unwrap_or_else(|e| {
            eprintln!("Frontmatter parse error: {}", e);
            JsonValue::Null
        });
        (yaml, body)
    } else {
        (JsonValue::Null, content)
    }
}
