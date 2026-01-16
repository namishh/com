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
            name: "Solo Levelling".to_string(),
            category: "anime".to_string(),
            tags: Some(vec!["abysmal dogshit".to_string()]),
            image: "https://m.media-amazon.com/images/M/MV5BM2M4YzdkMTEtMjUyYy00ZWY0LWI5ODQtNGRkZWQ1MzU5MWM2XkEyXkFqcGc@._V1_FMjpg_UX1000_.jpg".to_string(),
            desc: Some("insomnia final treatment".to_string()),
            score: 0.0,
        },
        MediaItem {
            name: "Eighty Six".to_string(),
            category: "anime".to_string(),
            tags: None,
            image: "https://m.media-amazon.com/images/M/MV5BOWNmY2IzOGItMmQyNy00ZTM0LThiNjItODM3YzdkYjRlNWU1XkEyXkFqcGc@._V1_FMjpg_UX1000_.jpg".to_string(),
            desc: Some("season 2 is much better, and the ending is really well wrapped up".to_string()),
            score: 8.6,
        },
        MediaItem {
            name: "Love is War".to_string(),
            category: "anime".to_string(),
            tags: None,
            image: "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQZ8uWVp4-mIpRO0NQ2JdmRt1bS7gE-m-wUWiUvgkpmqSlH5mXlkbL9NkuCWbcx1wWpkwuI&s=10".to_string(),
            desc: Some("i usually don't watch romance anime because they make me feel lonely but this is way too over the top and funny. funny enough to make me laugh even during the rewatch".to_string()),
            score: 9.0,
        },
        MediaItem {
            name: "Love is War: Stairway to Adulthood".to_string(),
            category: "anime movie".to_string(),
            tags: None,
            image: "https://m.media-amazon.com/images/M/MV5BMjE3OTAyZTAtNmUxNy00ZjA0LTgzZGEtZjYwMjk3NzNjYTdmXkEyXkFqcGc@._V1_QL75_UY281_CR5,0,190,281_.jpg".to_string(),
            desc: Some("some funny moments but does not live up to the hype".to_string()),
            score: 6.7,
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
