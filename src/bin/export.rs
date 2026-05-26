use ab_glyph::FontRef;
use chrono::{DateTime, NaiveDate, Utc};
use futures_util::FutureExt;
use image::DynamicImage;
use serde::Serialize;
use serde_json::Value as JsonValue;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs;
use std::panic::AssertUnwindSafe;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tera::Context;

#[path = "../file_tree.rs"]
mod file_tree;
#[path = "../image_generator.rs"]
mod image_generator;
#[path = "../markdown.rs"]
mod markdown;
#[path = "../media.rs"]
mod media;
#[path = "../projects.rs"]
mod projects;
#[path = "../templates.rs"]
mod templates;
#[path = "../tweet.rs"]
mod tweet;

#[derive(Serialize)]
struct SearchDocument {
    title: String,
    url: String,
    content: String,
}

struct ContentItem {
    title: String,
    path: String,
    date: Option<DateTime<Utc>>,
    description: Option<String>,
}

struct Page {
    template: &'static str,
    route: &'static str,
    output: &'static str,
    og_path: Option<&'static str>,
    og_title: &'static str,
    og_subtitle: &'static str,
    context: Option<fn(&mut Context)>,
}

const PAGES: &[Page] = &[
    Page {
        template: "index.html",
        route: "/",
        output: "index.html",
        og_path: Some("index"),
        og_title: "namishh",
        og_subtitle: "personal website and garden",
        context: None,
    },
    Page {
        template: "projects.html",
        route: "/stuff",
        output: "stuff/index.html",
        og_path: Some("stuff"),
        og_title: "namishh",
        og_subtitle: "stuff i have built",
        context: Some(add_projects_context),
    },
    Page {
        template: "art.html",
        route: "/art",
        output: "art/index.html",
        og_path: Some("art"),
        og_title: "namishh",
        og_subtitle: "art i have made",
        context: None,
    },

    Page {
        template: "media.html",
        route: "/media",
        output: "media/index.html",
        og_path: Some("media"),
        og_title: "namishh",
        og_subtitle: "media i consume and review",
        context: Some(add_media_context),
    },
    Page {
        template: "search.html",
        route: "/search",
        output: "search/index.html",
        og_path: Some("search"),
        og_title: "namishh",
        og_subtitle: "search stuff around here",
        context: Some(add_search_context),
    },
    Page {
        template: "404.html",
        route: "/404",
        output: "404.html",
        og_path: None,
        og_title: "",
        og_subtitle: "",
        context: None,
    },
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dist = Path::new("dist");
    if dist.exists() {
        fs::remove_dir_all(dist)?;
    }
    fs::create_dir_all(dist)?;
    write_file(&dist.join(".nojekyll"), "")?;

    copy_dir(Path::new("static"), &dist.join("static"))?;

    let tera = templates::init_tera();
    let file_tree = Arc::new(file_tree::build_file_tree(
        Path::new("content"),
        Path::new(""),
    ));
    let highlighter = Mutex::new(inkjet::Highlighter::new());
    let title_font =
        FontRef::try_from_slice(include_bytes!("../../static/_priv/fonts/InterE.ttf"))?;
    let path_font = FontRef::try_from_slice(include_bytes!("../../static/_priv/fonts/InterM.ttf"))?;
    let avatar = actix_rt::System::new().block_on(image_generator::load_avatar());

    render_pages(dist, &tera, &file_tree)?;

    let mut search_documents = Vec::new();
    let mut content_items = Vec::new();
    let mut tweet_ids = HashSet::new();
    render_content_dir(
        Path::new("content"),
        Path::new("content"),
        dist,
        &tera,
        &file_tree,
        &highlighter,
        &title_font,
        &path_font,
        &avatar,
        &mut search_documents,
        &mut content_items,
        &mut tweet_ids,
    )?;

    write_file(
        &dist.join("search-index.json"),
        &serde_json::to_string(&search_documents)?,
    )?;
    write_file(&dist.join("rss.xml"), &render_rss(content_items))?;
    generate_web_og_images(dist, &title_font, &path_font, &avatar)?;
    actix_rt::System::new().block_on(generate_tweet_images(
        dist,
        &title_font,
        &path_font,
        &tweet_ids,
    ))?;

    println!("Exported static site to dist/");
    Ok(())
}

fn add_projects_context(context: &mut Context) {
    context.insert("projects", &projects::get_projects());
}

fn add_media_context(context: &mut Context) {
    context.insert("media", &media::get_media());
}

fn add_search_context(context: &mut Context) {
    context.insert("has_query", &false);
}

fn render_pages(
    dist: &Path,
    tera: &tera::Tera,
    file_tree: &Arc<Vec<file_tree::FileNode>>,
) -> Result<(), Box<dyn std::error::Error>> {
    for page in PAGES {
        render_web_page(tera, file_tree, page, &dist.join(page.output))?;
    }
    Ok(())
}

fn render_web_page(
    tera: &tera::Tera,
    file_tree: &Arc<Vec<file_tree::FileNode>>,
    page: &Page,
    output: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut context = Context::new();
    context.insert("file_tree", &file_tree::get_file_tree(file_tree));
    context.insert("path", page.route);
    if let Some(add_context) = page.context {
        add_context(&mut context);
    }
    write_file(output, &tera.render(page.template, &context)?)
}

fn render_content_dir(
    base: &Path,
    current: &Path,
    dist: &Path,
    tera: &tera::Tera,
    file_tree: &Arc<Vec<file_tree::FileNode>>,
    highlighter: &Mutex<inkjet::Highlighter>,
    title_font: &FontRef,
    path_font: &FontRef,
    avatar: &Option<DynamicImage>,
    search_documents: &mut Vec<SearchDocument>,
    content_items: &mut Vec<ContentItem>,
    tweet_ids: &mut HashSet<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default();
        if file_name.starts_with('.') {
            continue;
        }

        if path.is_dir() {
            render_content_dir(
                base,
                &path,
                dist,
                tera,
                file_tree,
                highlighter,
                title_font,
                path_font,
                avatar,
                search_documents,
                content_items,
                tweet_ids,
            )?;
        } else if path.extension().is_some_and(|ext| ext == "md") {
            render_markdown_file(
                base,
                &path,
                dist,
                tera,
                file_tree,
                highlighter,
                title_font,
                path_font,
                avatar,
                search_documents,
                content_items,
                tweet_ids,
            )?;
        }
    }
    Ok(())
}

fn render_markdown_file(
    base: &Path,
    file_path: &Path,
    dist: &Path,
    tera: &tera::Tera,
    file_tree: &Arc<Vec<file_tree::FileNode>>,
    highlighter: &Mutex<inkjet::Highlighter>,
    title_font: &FontRef,
    path_font: &FontRef,
    avatar: &Option<DynamicImage>,
    search_documents: &mut Vec<SearchDocument>,
    content_items: &mut Vec<ContentItem>,
    tweet_ids: &mut HashSet<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let raw = fs::read_to_string(file_path)?;
    let (frontmatter, body) = markdown::extract_frontmatter(&raw);
    let body = rewrite_tweet_urls(body, tweet_ids);
    let (content_html, headings) = markdown::markdown_to_html(&body, highlighter);
    let rel_path = file_path.strip_prefix(base)?.with_extension("");
    let url = rel_path.to_string_lossy().replace('\\', "/");

    let mut context = Context::new();
    let mut title = file_path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let mut draft = false;
    let mut description = None;
    let mut date = None;

    if let JsonValue::Object(map) = frontmatter {
        for (key, value) in map {
            if key == "title" {
                if let Some(value) = value.as_str() {
                    title = value.to_string();
                }
            }
            if key == "draft" {
                draft = value.as_bool().unwrap_or(false);
            }
            if key == "description" {
                description = value.as_str().map(ToString::to_string);
            }
            if key == "date" {
                date = value.as_str().and_then(parse_date).map(|date| {
                    DateTime::<Utc>::from_naive_utc_and_offset(
                        date.and_hms_opt(0, 0, 0).unwrap(),
                        Utc,
                    )
                });
            }
            context.insert(key, &value);
        }
    }

    context.insert("title", &title);
    context.insert("headings", &headings);
    context.insert("file_tree", &file_tree::get_file_tree(file_tree));
    context.insert("content", &content_html);
    context.insert("file_path", &url);
    context.insert("path", &format!("/{url}"));

    write_file(
        &dist.join(&rel_path).join("index.html"),
        &tera.render("view.html", &context)?,
    )?;

    if !draft {
        search_documents.push(SearchDocument {
            title: title.clone(),
            url: url.clone(),
            content: body,
        });
        content_items.push(ContentItem {
            title: title.clone(),
            path: url.clone(),
            date,
            description,
        });
    }

    let dir_path = file_path
        .parent()
        .and_then(|path| path.strip_prefix(base).ok())
        .map(|path| path.to_string_lossy().to_string())
        .unwrap_or_default();
    let image = image_generator::generate_content_og_image(
        &title, &dir_path, title_font, path_font, avatar,
    );
    write_bytes(&dist.join("og/content").join(format!("{url}.png")), &image)?;

    Ok(())
}

fn rewrite_tweet_urls(body: &str, tweet_ids: &mut HashSet<String>) -> String {
    let re = regex::Regex::new(r"/tweet/([0-9]+)(\.png)?").unwrap();
    re.replace_all(body, |captures: &regex::Captures| {
        let id = captures.get(1).unwrap().as_str();
        tweet_ids.insert(id.to_string());
        format!("/tweet/{id}.png")
    })
    .into_owned()
}

async fn generate_tweet_images(
    dist: &Path,
    title_font: &FontRef<'_>,
    path_font: &FontRef<'_>,
    tweet_ids: &HashSet<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut tweet_ids = tweet_ids.iter().collect::<Vec<_>>();
    tweet_ids.sort();

    for id in tweet_ids {
        let result = AssertUnwindSafe(tweet::generate_tweet(id, title_font, path_font))
            .catch_unwind()
            .await;
        match result {
            Ok(Ok(image)) => write_bytes(&dist.join("tweet").join(format!("{id}.png")), &image)?,
            Ok(Err(error)) => eprintln!("Failed to generate tweet {id}: {error}"),
            Err(_) => eprintln!("Failed to generate tweet {id}: generator panicked"),
        }
    }

    Ok(())
}

fn generate_web_og_images(
    dist: &Path,
    title_font: &FontRef,
    path_font: &FontRef,
    avatar: &Option<DynamicImage>,
) -> Result<(), Box<dyn std::error::Error>> {
    for page in PAGES {
        let Some(path) = page.og_path else {
            continue;
        };
        let image = image_generator::generate_web_og_image(
            page.og_title,
            page.og_subtitle,
            title_font,
            path_font,
            avatar,
        );
        write_bytes(&dist.join("og/web").join(format!("{path}.png")), &image)?;
    }
    Ok(())
}

fn render_rss(mut items: Vec<ContentItem>) -> String {
    items.sort_by(|a, b| match (&a.date, &b.date) {
        (Some(a_date), Some(b_date)) => b_date.cmp(a_date),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => a.title.cmp(&b.title),
    });

    let base_url = "https://namishh.com";
    let mut channel = rss::ChannelBuilder::default()
        .title("namishh")
        .link(base_url)
        .description("Personal website and digital garden of namishh")
        .language(Some("en-us".to_string()))
        .last_build_date(Some(chrono::Utc::now().to_rfc2822()))
        .build();

    for item in items {
        channel.items.push(
            rss::ItemBuilder::default()
                .title(Some(item.title))
                .link(Some(format!("{}/{}", base_url, item.path)))
                .pub_date(
                    item.date
                        .map(|date| date.format("%a, %d %b %Y").to_string()),
                )
                .description(
                    item.description
                        .or_else(|| Some("Read more about this content".to_string())),
                )
                .build(),
        );
    }

    channel.to_string()
}

fn parse_date(date_str: &str) -> Option<NaiveDate> {
    let parts: Vec<_> = date_str.split_whitespace().collect();
    if parts.len() != 3 {
        return None;
    }
    let day = parts[0].parse().ok()?;
    let year = parts[2].parse().ok()?;
    let month = match &parts[1].to_lowercase()[..3] {
        "jan" => 1,
        "feb" => 2,
        "mar" => 3,
        "apr" => 4,
        "may" => 5,
        "jun" => 6,
        "jul" => 7,
        "aug" => 8,
        "sep" => 9,
        "oct" => 10,
        "nov" => 11,
        "dec" => 12,
        _ => return None,
    };
    NaiveDate::from_ymd_opt(year, month, day)
}

fn copy_dir(from: &Path, to: &Path) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let target = to.join(entry.file_name());
        if entry.path().is_dir() {
            copy_dir(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}

fn write_file(path: &Path, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    write_bytes(path, content.as_bytes())
}

fn write_bytes(path: &Path, content: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)?;
    Ok(())
}
