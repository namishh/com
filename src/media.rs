use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MediaItem {
    pub name: String,
    pub category: String,
    pub tags: Option<Vec<String>>,
    pub image: String,
    pub desc: Option<String>,
    pub score: f32,
}

const REVIEWS: &str = include_str!("../static/_priv/reviews.txt");

pub fn get_media() -> Vec<MediaItem> {
    REVIEWS
        .split("\n---")
        .filter_map(|block| {
            let block = block.trim_matches(|c| c == '-' || c == '\n');
            let mut lines = block.lines();
            let name = lines.next()?.trim();
            if name.is_empty() {
                return None;
            }
            let tags = lines.next().unwrap_or("").trim();
            let category = lines.next().unwrap_or("").trim();
            let score = lines.next().unwrap_or("0").trim();
            let image = lines.next().unwrap_or("").trim();
            let desc = lines.collect::<Vec<_>>().join("\n");
            let desc = desc.trim();
            Some(MediaItem {
                name: name.to_string(),
                category: category.to_string(),
                tags: (!tags.is_empty())
                    .then(|| tags.split(',').map(|t| t.trim().to_string()).collect()),
                image: image.to_string(),
                desc: (!desc.is_empty()).then(|| desc.to_string()),
                score: score.parse().unwrap_or(0.0),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_reviews() {
        let media = get_media();
        assert_eq!(media.len(), 23);
        let balatro = &media[0];
        assert_eq!(balatro.name, "Balatro");
        assert_eq!(balatro.tags.as_ref().unwrap().len(), 2);
        assert_eq!(balatro.score, 10.0);
        let reze = media.iter().find(|m| m.name == "Reze Arc").unwrap();
        assert!(reze.tags.is_none());
        assert!(reze.desc.is_some());
        let boi = media.iter().find(|m| m.name == "The Beginning Of Infinity").unwrap();
        assert!(boi.desc.is_none());
        assert!(media.iter().all(|m| !m.image.is_empty() && !m.category.is_empty()));
    }
}
