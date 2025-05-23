use crate::config::app_config::{AppConfig, TitleLanguage};

use super::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use strum_macros::{Display, EnumIter, EnumString, IntoStaticStr};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AnimeSeason {
    pub year: u64,
    pub season: Season,
}

#[derive(Clone, Debug, PartialEq, EnumString, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum AnimeField {
    Id,
    Titel,
    MainPicture,
    AlternativeTitles,
    StartDate,
    EndDate,
    Synopsis,
    Mean,
    Rank,
    Popularity,
    NumListUsers,
    NumScoringUsers,
    NSFW,
    CreatedAt,
    UpdatedAt,
    MediaType,
    Status,
    MyListStatus,
    NumEpisodes,
    Broadcast,
    Source,
    AverageEpisodeDuration,
    Rating,
    Pictures,
    Background,
    RelatedAnime,
    RelatedManga,
    Recommendations,
    Studios,
    Statistics,

    NumVolumes,
    NumChapters,
    Authors,

    Name,
    Picture,
    Gender,
    Birthday,
    Location,
    JoinedAt,
    AnimeStatistics,
    TimeZone,
    IsSupporter,
}

#[derive(Clone, Debug, PartialEq, EnumString, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum AnimeMediaType {
    Unknown,
    #[strum(serialize = "tv")]
    TV,
    #[strum(serialize = "ova")]
    OVA,
    Movie,
    Special,
    #[strum(serialize = "ona")]
    ONA,
    Music,
    Other(String),
}

#[derive(Clone, Debug, PartialEq, EnumString, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum AnimeStatus {
    FinishedAiring,
    CurrentlyAiring,
    NotYetAired,
    Other(String),
}

#[derive(Clone, Debug, PartialEq, EnumString, IntoStaticStr, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Source {
    Other,
    Original,
    Manga,
    #[strum(serialize = "4_koma_manga")]
    YonKomaManga,
    WebManga,
    DigitalManga,
    Novel,
    LightNovel,
    VisualNovel,
    Game,
    CardGame,
    Book,
    PictureBook,
    Radio,
    Music,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserAnimeListStatus {
    pub status: UserWatchStatus,
    pub score: u8,
    pub num_episodes_watched: u64,
    pub is_rewatching: bool,
    pub start_date: Option<DateWrapper>,
    pub finish_date: Option<DateWrapper>,
    pub priority: Option<u8>,
    pub num_times_rewatched: Option<u64>,
    pub rewatch_value: Option<u8>,
    pub tags: Option<Vec<String>>,
    pub comments: Option<String>,
    pub updated_at: DateTimeWrapper,
}

#[derive(Clone, Debug, EnumString, IntoStaticStr)]
pub enum Rating {
    G,
    #[strum(serialize = "pg")]
    PG,
    #[strum(serialize = "pg_13")]
    PG13,
    R,
    #[strum(serialize = "r+")]
    Rp,
    #[strum(serialize = "rx")]
    RX,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Anime {
    pub id: u64,
    pub title: String,
    pub main_picture: Option<Picture>,
    pub alternative_titles: Option<AlternativeTitles>,
    pub start_date: Option<DateWrapper>,
    pub end_date: Option<DateWrapper>,
    pub synopsis: Option<String>,
    pub mean: Option<f64>,
    pub rank: Option<u64>,
    pub popularity: Option<u64>,
    pub num_list_users: Option<u64>,
    pub num_scoring_users: Option<u64>,
    pub nsfw: Option<NSFW>,
    pub genres: Option<Vec<Genre>>,
    pub created_at: Option<DateTimeWrapper>,
    pub updated_at: Option<DateTimeWrapper>,
    pub media_type: Option<AnimeMediaType>,
    pub status: Option<AnimeStatus>,
    pub my_list_status: Option<UserAnimeListStatus>,
    pub num_episodes: Option<u64>,
    pub start_season: Option<StartSeason>,
    pub broadcast: Option<Broadcast>,
    pub source: Option<Source>,
    pub average_episode_duration: Option<u64>,
    pub rating: Option<String>,
    pub studios: Option<Vec<Studio>>,
    pub pictures: Option<Vec<Picture>>,
    pub background: Option<String>,
    pub related_anime: Option<Vec<RelatedAnime>>,
    pub related_manga: Option<Vec<RelatedManga>>,
    pub recommendations: Option<Vec<AnimeRecommendation>>,
    pub statistics: Option<MediaDetailStatistics>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AnimeRecommendation {
    pub node: Anime,
    pub num_recommendations: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RelatedAnime {
    pub node: Anime,
    pub relation_type: RelationType,
    pub relation_type_formatted: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StartSeason {
    pub season: Season,
    pub year: u16,
}

#[derive(Clone, Debug, PartialEq, EnumString, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum RelationType {
    Sequel,
    Prequel,
    AlternativeSetting,
    AlternativeVersion,
    SideStory,
    ParentStory,
    Summary,
    FullStory,
    #[strum(serialize = "other")]
    Other(String),
}

#[derive(Clone, Debug, PartialEq, EnumString, IntoStaticStr, Display)]
#[strum(serialize_all = "snake_case")]
pub enum AnimeRankingType {
    All,
    Airing,
    Upcoming,
    #[strum(serialize = "tv")]
    TV,
    #[strum(serialize = "ova")]
    OVA,
    Movie,
    Special,
    #[strum(serialize = "popularity")]
    ByPopularity,
    Favorite,
    Other(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RankingAnimePair {
    pub node: Anime,
    pub ranking: RankingInfo,
}

#[derive(Clone, Debug, PartialEq, Display, EnumString, EnumIter, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum UserWatchStatus {
    Watching,
    Completed,
    OnHold,
    Dropped,
    PlanToWatch,
    #[strum(serialize = "add")]
    Other(String),
}

#[derive(Clone, Debug, PartialEq, EnumString, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum SortStyle {
    ListScore,
    ListUpdatedAt,
    AnimeTitle,
    AnimeStartDate,
    AnimeId,
    Other(String),
}

impl Anime {
    pub fn get_title(&self, app_config: &AppConfig, both: bool) -> Vec<String> {
        if both {
            vec![
                self.title.clone(),
                self.alternative_titles
                    .as_ref()
                    .and_then(|alternative_titles| alternative_titles.en.clone())
                    .unwrap_or_else(|| self.title.clone()),
            ]
        } else {
            match app_config.title_language {
                TitleLanguage::Japanese => vec![self.title.clone()],
                TitleLanguage::English => {
                    if let Some(ref alternative_titles) = self.alternative_titles {
                        if let Some(ref en) = alternative_titles.en {
                            if !en.is_empty() {
                                vec![en.clone()]
                            } else {
                                vec![self.title.clone()]
                            }
                        } else {
                            vec![self.title.clone()]
                        }
                    } else {
                        vec![self.title.clone()]
                    }
                }
            }
        }
    }
}
