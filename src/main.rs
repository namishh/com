use ab_glyph::FontRef;
use actix_web::{App, HttpServer, middleware, web};
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::file_tree::build_file_tree;
use crate::handlers::{
    generate_og_image, generate_tweet_image, generate_web_og, health_check, index, media, projects,
    resume, search, search_page, view_markdown, arts
};
use crate::rss::rss_feed;
use crate::search::initialize_search_index;
use crate::state::AppState;
use crate::templates::init_tera;

mod file_tree;
mod handlers;
mod image_generator;
mod markdown;
mod media;
mod projects;
mod rss;
mod search;
mod state;
mod templates;
mod tweet;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let highlighter = Arc::new(Mutex::new(inkjet::Highlighter::new()));

    let base_path = Path::new("content");
    let initial_tree = build_file_tree(base_path, Path::new(""));
    let file_tree = Arc::new(initial_tree);

    initialize_search_index(base_path)?;

    let title_font_data: &'static [u8] = include_bytes!("../static/_priv/fonts/InterE.ttf");
    let title_font = FontRef::try_from_slice(title_font_data).expect("Error loading title font");
    let title_font_arc = Arc::new(title_font);

    let path_font_data: &'static [u8] = include_bytes!("../static/_priv/fonts/InterM.ttf");
    let path_font = FontRef::try_from_slice(path_font_data).expect("Error loading path font");
    let path_font_arc = Arc::new(path_font);

    let mut address = "127.0.0.1:8080";
    if let Ok(arg) = std::env::var("ENVIRONMENT") {
        if arg == "PRODUCTION" {
            address = "0.0.0.0:8080";
        }
    }

    let server = HttpServer::new(move || {
        let tera = init_tera();

        let app_state = AppState {
            tera,
            highlighter: highlighter.clone(),
            file_tree: file_tree.clone(),
            title_font: title_font_arc.clone(),
            path_font: path_font_arc.clone(),
        };

        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(middleware::Logger::default())
            .service(actix_files::Files::new("/static", "./static"))
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/stuff").route(web::get().to(projects)))
            .service(web::resource("/art").route(web::get().to(arts)))
            .service(web::resource("/media").route(web::get().to(media)))
            .service(web::resource("/resume").route(web::get().to(resume)))
            .service(web::resource("/search").route(web::get().to(search_page)))
            .service(web::resource("/og/content/{path:.*}").route(web::get().to(generate_og_image)))
            .service(web::resource("/og/web/{path:.*}").route(web::get().to(generate_web_og)))
            .service(web::resource("/tweet/{path:.*}").route(web::get().to(generate_tweet_image)))
            .service(web::resource("/rss.xml").route(web::get().to(rss_feed)))
            .service(web::resource("/api/search").route(web::get().to(search)))
            .service(web::resource("/health").route(web::get().to(health_check)))
            .service(web::resource("/{path:.*}").route(web::get().to(view_markdown)))
    })
    .bind(address)?
    .run();

    server.await
}
