use image::ImageEncoder;
use once_cell::sync::Lazy;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Serialize)]
pub struct Artwork {
    pub src: String,
    pub alt: String,
    pub placeholder: String,
}

static PLACEHOLDERS: Lazy<Mutex<HashMap<String, String>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

// tiny (16px wide) pixelated mosaic as a data URI, shown until the full image loads.
// cached on disk in static/images/art/px/, regenerated only when the source file is newer
fn make_placeholder(name: &str) -> String {
    if let Some(cached) = PLACEHOLDERS.lock().unwrap().get(name) {
        return cached.clone();
    }
    let placeholder = load_or_generate(name).unwrap_or_default();
    PLACEHOLDERS
        .lock()
        .unwrap()
        .insert(name.to_string(), placeholder.clone());
    placeholder
}

fn load_or_generate(name: &str) -> Option<String> {
    let src_path = format!("static/images/art/{name}");
    let px_path = format!("static/images/art/px/{}.png", name.rsplit_once('.')?.0);

    let mtime = |p: &str| std::fs::metadata(p).and_then(|m| m.modified()).ok();
    let fresh = mtime(&px_path)
        .zip(mtime(&src_path))
        .is_some_and(|(px, src)| px >= src);

    let buf = if fresh {
        std::fs::read(&px_path).ok()?
    } else {
        let img = image::open(&src_path).ok()?;
        let w = 16;
        let h = (img.height() * w / img.width()).max(1);
        let small = image::imageops::resize(&img, w, h, image::imageops::FilterType::Nearest);
        let mut buf = Vec::new();
        image::codecs::png::PngEncoder::new(&mut buf)
            .write_image(small.as_raw(), w, h, image::ExtendedColorType::Rgba8)
            .ok()?;
        std::fs::create_dir_all("static/images/art/px").ok()?;
        std::fs::write(&px_path, &buf).ok()?;
        buf
    };
    Some(format!("data:image/png;base64,{}", base64(&buf)))
}

fn base64(data: &[u8]) -> String {
    const T: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in data.chunks(3) {
        let n = chunk.iter().fold(0u32, |acc, &b| (acc << 8) | b as u32) << (8 * (3 - chunk.len()));
        out.push(T[(n >> 18) as usize & 63] as char);
        out.push(T[(n >> 12) as usize & 63] as char);
        out.push(if chunk.len() > 1 { T[(n >> 6) as usize & 63] as char } else { '=' });
        out.push(if chunk.len() > 2 { T[n as usize & 63] as char } else { '=' });
    }
    out
}

pub fn get_artworks() -> Vec<Artwork> {
    let mut artworks: Vec<Artwork> = std::fs::read_dir("static/images/art")
        .map(|rd| {
            rd.flatten()
                .filter_map(|entry| {
                    let name = entry.file_name().into_string().ok()?;
                    let (stem, ext) = name.rsplit_once('.')?;
                    if !matches!(
                        ext.to_ascii_lowercase().as_str(),
                        "webp" | "png" | "jpg" | "jpeg" | "gif" | "avif"
                    ) {
                        return None;
                    }
                    let alt = stem
                        .trim_start_matches(|c: char| c.is_ascii_digit() || c == '-')
                        .replace('-', " ");
                    Some(Artwork {
                        placeholder: make_placeholder(&name),
                        src: format!("/static/images/art/{name}"),
                        alt,
                    })
                })
                .collect()
        })
        .unwrap_or_default();
    artworks.sort_by(|a, b| a.src.cmp(&b.src));
    artworks
}
