use crate::file_tree::FileNode;
use ab_glyph::FontRef;
use inkjet::Highlighter;
use std::sync::{Arc, Mutex};
use tera::Tera;

pub struct AppState {
    pub tera: Tera,
    pub highlighter: Arc<Mutex<Highlighter>>,
    pub file_tree: Arc<Vec<FileNode>>,
    pub title_font: Arc<FontRef<'static>>,
    pub path_font: Arc<FontRef<'static>>,
}
