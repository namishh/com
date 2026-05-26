use ab_glyph::{FontRef, PxScale};
use image::{DynamicImage, ImageBuffer, ImageEncoder, Rgba, load_from_memory};
use imageproc::drawing;
use once_cell::sync::Lazy;
use std::env;
use std::sync::Arc;

pub const AVATAR_URL: &str = "https://github.com/namishh.png";

static BACKGROUND_IMAGES: Lazy<Arc<BackgroundImages>> = Lazy::new(|| Arc::new(load_backgrounds()));

struct BackgroundImages {
    notes: ImageBuffer<Rgba<u8>, Vec<u8>>,
    blog: ImageBuffer<Rgba<u8>, Vec<u8>>,
    poems: ImageBuffer<Rgba<u8>, Vec<u8>>,
    journal: ImageBuffer<Rgba<u8>, Vec<u8>>,
    others: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

struct TextLayout {
    lines: Vec<String>,
    scale: PxScale,
    line_height: i32,
}

fn load_backgrounds() -> BackgroundImages {
    let current_dir = env::current_dir().expect("Could not get current directory");
    let load_image = |path: &str| {
        image::open(current_dir.join(path))
            .map(|img| img.to_rgba8())
            .unwrap_or_else(|_| {
                let mut fallback = ImageBuffer::new(1200, 630);
                for pixel in fallback.pixels_mut() {
                    *pixel = Rgba([40, 40, 40, 255]);
                }
                fallback
            })
    };

    BackgroundImages {
        notes: load_image("static/_priv/og/notes.png"),
        blog: load_image("static/_priv/og/blog.png"),
        poems: load_image("static/_priv/og/poems.png"),
        journal: load_image("static/_priv/og/journal.png"),
        others: load_image("static/_priv/og/others.png"),
    }
}

pub async fn load_avatar() -> Option<DynamicImage> {
    let response = reqwest::get(AVATAR_URL).await.ok()?;
    if !response.status().is_success() {
        return None;
    }

    let bytes = response.bytes().await.ok()?;
    load_from_memory(&bytes).ok()
}

fn text_width(text: &str, scale: PxScale, font: &FontRef) -> u32 {
    drawing::text_size(scale, font, text).0
}

fn wrap_long_word(word: &str, scale: PxScale, font: &FontRef, max_width: u32) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();

    for character in word.chars() {
        let candidate = format!("{current}{character}");
        if !current.is_empty() && text_width(&candidate, scale, font) > max_width {
            lines.push(current);
            current = character.to_string();
        } else {
            current = candidate;
        }
    }

    if !current.is_empty() {
        lines.push(current);
    }

    lines
}

fn wrap_text(text: &str, scale: PxScale, font: &FontRef, max_width: u32) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();

    for word in text.split_whitespace() {
        let candidate = if current.is_empty() {
            word.to_string()
        } else {
            format!("{current} {word}")
        };

        if text_width(&candidate, scale, font) <= max_width {
            current = candidate;
            continue;
        }

        if !current.is_empty() {
            lines.push(current);
            current = String::new();
        }

        if text_width(word, scale, font) <= max_width {
            current = word.to_string();
        } else {
            lines.extend(wrap_long_word(word, scale, font, max_width));
        }
    }

    if !current.is_empty() {
        lines.push(current);
    }

    lines
}

fn truncate_line(text: &str, scale: PxScale, font: &FontRef, max_width: u32) -> String {
    const ELLIPSIS: &str = "...";
    let mut truncated = String::new();

    for character in text.chars() {
        let candidate = format!("{truncated}{character}{ELLIPSIS}");
        if text_width(&candidate, scale, font) > max_width {
            break;
        }
        truncated.push(character);
    }

    if truncated.is_empty() {
        ELLIPSIS.to_string()
    } else {
        format!("{truncated}{ELLIPSIS}")
    }
}

fn fit_text(
    text: &str,
    font: &FontRef,
    max_width: u32,
    max_height: u32,
    max_size: u32,
    min_size: u32,
) -> TextLayout {
    for size in (min_size..=max_size).rev() {
        let scale = PxScale {
            x: size as f32,
            y: size as f32,
        };
        let line_height = (size as f32 * 1.12).ceil() as i32;
        let lines = wrap_text(text, scale, font, max_width);
        let total_height = lines.len() as u32 * line_height as u32;

        if !lines.is_empty() && total_height <= max_height {
            return TextLayout {
                lines,
                scale,
                line_height,
            };
        }
    }

    let scale = PxScale {
        x: min_size as f32,
        y: min_size as f32,
    };
    let line_height = (min_size as f32 * 1.12).ceil() as i32;
    let max_lines = (max_height / line_height as u32).max(1) as usize;
    let mut lines = wrap_text(text, scale, font, max_width);
    lines.truncate(max_lines);

    if lines.len() == max_lines {
        if let Some(last_line) = lines.last_mut() {
            *last_line = truncate_line(last_line, scale, font, max_width);
        }
    }

    TextLayout {
        lines,
        scale,
        line_height,
    }
}

fn draw_text_layout(
    img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    color: Rgba<u8>,
    x: i32,
    y: i32,
    layout: &TextLayout,
    font: &FontRef,
) {
    for (index, line) in layout.lines.iter().enumerate() {
        drawing::draw_text_mut(
            img,
            color,
            x,
            y + index as i32 * layout.line_height,
            layout.scale,
            font,
            line,
        );
    }
}

pub fn generate_content_og_image(
    title: &str,
    dir_path: &str,
    title_font: &FontRef,
    path_font: &FontRef,
    avatar: &Option<DynamicImage>,
) -> Vec<u8> {
    let bg = match dir_path {
        path if path.starts_with("notes") => &BACKGROUND_IMAGES.notes,
        path if path.starts_with("blog") => &BACKGROUND_IMAGES.blog,
        path if path.starts_with("journal") => &BACKGROUND_IMAGES.journal,
        path if path.starts_with("poems") => &BACKGROUND_IMAGES.poems,
        _ => &BACKGROUND_IMAGES.notes,
    };
    let mut img = bg.clone(); // Clone the preloaded background

    let text_color = Rgba([255, 255, 255, 255]);
    let title_layout = fit_text(title, title_font, 1000, 280, 96, 44);
    draw_text_layout(&mut img, text_color, 100, 170, &title_layout, title_font);

    let path_scale = PxScale { x: 36.0, y: 36.0 };
    let path_text = format!("/{}", dir_path);
    drawing::draw_text_mut(
        &mut img,
        Rgba([240, 240, 240, 255]),
        100,
        500,
        path_scale,
        path_font,
        &path_text,
    );

    if let Some(avatar_img) = avatar {
        static AVATAR_SIZE: u32 = 50;
        static MASK: once_cell::sync::Lazy<Vec<bool>> = once_cell::sync::Lazy::new(|| {
            let mut mask = vec![false; (AVATAR_SIZE * AVATAR_SIZE) as usize];
            let center = AVATAR_SIZE as f32 / 2.0;
            for y in 0..AVATAR_SIZE {
                for x in 0..AVATAR_SIZE {
                    let distance =
                        ((x as f32 - center).powi(2) + (y as f32 - center).powi(2)).sqrt();
                    if distance <= center {
                        mask[(y * AVATAR_SIZE + x) as usize] = true;
                    }
                }
            }
            mask
        });

        let resized_avatar = avatar_img
            .resize_exact(
                AVATAR_SIZE,
                AVATAR_SIZE,
                image::imageops::FilterType::Lanczos3,
            )
            .to_rgba8();
        let avatar_x = 1200 - AVATAR_SIZE - 30;
        let avatar_y = 630 - AVATAR_SIZE - 30;

        for (i, &in_mask) in MASK.iter().enumerate() {
            if in_mask {
                let x = (i as u32 % AVATAR_SIZE) + avatar_x;
                let y = (i as u32 / AVATAR_SIZE) + avatar_y;
                if x < 1200 && y < 630 {
                    img.put_pixel(
                        x,
                        y,
                        *resized_avatar.get_pixel(i as u32 % AVATAR_SIZE, i as u32 / AVATAR_SIZE),
                    );
                }
            }
        }
    }

    let mut bytes = Vec::new();
    image::codecs::png::PngEncoder::new(&mut bytes)
        .write_image(&img.into_raw(), 1200, 630, image::ExtendedColorType::Rgba8)
        .expect("Failed to encode image");
    bytes
}

pub fn generate_web_og_image(
    title: &str,
    subtitle: &str,
    title_font: &FontRef,
    path_font: &FontRef,
    avatar: &Option<DynamicImage>,
) -> Vec<u8> {
    let mut img = BACKGROUND_IMAGES.others.clone();

    let title_layout = fit_text(title, title_font, 760, 150, 120, 64);
    draw_text_layout(
        &mut img,
        Rgba([255, 255, 255, 255]),
        100,
        180,
        &title_layout,
        title_font,
    );

    let subtitle_scale = PxScale { x: 48.0, y: 48.0 };
    drawing::draw_text_mut(
        &mut img,
        Rgba([240, 240, 240, 255]),
        100,
        320,
        subtitle_scale,
        path_font,
        subtitle,
    );

    if let Some(avatar_img) = avatar {
        static AVATAR_SIZE: u32 = 150;
        static MASK: once_cell::sync::Lazy<Vec<bool>> = once_cell::sync::Lazy::new(|| {
            let mut mask = vec![false; (AVATAR_SIZE * AVATAR_SIZE) as usize];
            let center = AVATAR_SIZE as f32 / 2.0;
            for y in 0..AVATAR_SIZE {
                for x in 0..AVATAR_SIZE {
                    let distance =
                        ((x as f32 - center).powi(2) + (y as f32 - center).powi(2)).sqrt();
                    if distance <= center {
                        mask[(y * AVATAR_SIZE + x) as usize] = true;
                    }
                }
            }
            mask
        });

        let resized_avatar = avatar_img
            .resize_exact(
                AVATAR_SIZE,
                AVATAR_SIZE,
                image::imageops::FilterType::Lanczos3,
            )
            .to_rgba8();
        let avatar_x = 1200 - AVATAR_SIZE - 80;
        let avatar_y = 80;

        for (i, &in_mask) in MASK.iter().enumerate() {
            if in_mask {
                let x = (i as u32 % AVATAR_SIZE) + avatar_x;
                let y = (i as u32 / AVATAR_SIZE) + avatar_y;
                if x < 1200 && y < 630 {
                    img.put_pixel(
                        x,
                        y,
                        *resized_avatar.get_pixel(i as u32 % AVATAR_SIZE, i as u32 / AVATAR_SIZE),
                    );
                }
            }
        }
    }

    let mut bytes = Vec::new();
    image::codecs::png::PngEncoder::new(&mut bytes)
        .write_image(&img.into_raw(), 1200, 630, image::ExtendedColorType::Rgba8)
        .expect("Failed to encode image");
    bytes
}
