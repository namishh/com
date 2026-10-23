use serde::Serialize;

#[derive(Serialize)]
pub struct Artwork {
    pub src: String,
    pub alt: String,
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
                        "webp" | "png" | "jpg" | "jpeg" | "gif" // who knows man someday i might
                                                                // animate 
                    ) {
                        return None;
                    }
                    let alt = stem
                        .trim_start_matches(|c: char| c.is_ascii_digit() || c == '-')
                        .replace('-', " ");
                    Some(Artwork {
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
