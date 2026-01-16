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

pub fn get_media() -> Vec<MediaItem> {
    vec![
        MediaItem {
            name: "Cowboy Bebop".to_string(),
            category: "anime".to_string(),
            tags: Some(vec!["my top #3".to_string()]),
            image: "https://upload.wikimedia.org/wikipedia/en/a/a9/Cowboy_Bebop_key_visual.jpg".to_string(),
            desc: Some("makes smoking look cool. 10/10".to_string()),
            score: 10.0,
        },
        MediaItem {
            name: "Frieren: Beyond Journey's End".to_string(),
            category: "anime".to_string(),
            tags: None,
            image: "https://m.media-amazon.com/images/M/MV5BZTI4ZGMxN2UtODlkYS00MTBjLWE1YzctYzc3NDViMGI0ZmJmXkEyXkFqcGc@._V1_.jpg".to_string(),
            desc: Some("good world building. also a great filter for racists".to_string()),
            score: 7.0,
        },
        MediaItem {
            name: "Arcane".to_string(),
            category: "tv".to_string(),
            tags: None,
            image: "https://m.media-amazon.com/images/M/MV5BYjA2NzhlMDItNWRmZC00MzRjLWE3ZjAtZjBlZDAwOWY2ODdjXkEyXkFqcGc@._V1_FMjpg_UX1000_.jpg".to_string(),
            desc: Some("season 1 is great, season 2 is meh".to_string()),
            score: 8.5,
        },
    ]
}
