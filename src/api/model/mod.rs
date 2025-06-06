/// Anime related structs
pub mod anime;
pub use anime::*;
/// Manga related structs
pub mod manga;
pub use manga::*;
/// User related structs
pub mod user;
use serde::de::{self, Visitor};
pub use user::*;

use core::fmt;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Debug;
use std::str::FromStr;
use strum_macros::{Display, EnumString, IntoStaticStr};
use time::{
    format_description::{
        self,
        well_known::{iso8601, Iso8601},
    },
    // format_description::well_known::{iso8601, Iso8601},
    Date,
    Month,
    PrimitiveDateTime,
    Time,
};

pub type Page<T> = PageableData<Vec<Node<T>>>;
pub type Ranking<T> = PageableData<Vec<T>>;

#[derive(Debug, Clone)]
pub enum RankingType {
    AnimeRankingType(AnimeRankingType),
    MangaRankingType(MangaRankingType),
}

// uniform time format: "2021-08-01T00:00:00.0Z"
const CONFIG: iso8601::EncodedConfig = iso8601::Config::DEFAULT
    .set_year_is_six_digits(false)
    .encode();
const FORMAT: Iso8601<CONFIG> = Iso8601::<CONFIG>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Paging {
    pub previous: Option<String>,
    pub next: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PageableData<D: Clone + Debug> {
    pub data: D,
    pub paging: Paging,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Node<N: Clone + std::fmt::Debug> {
    pub node: N,
}

pub enum Media<'a> {
    Anime(&'a Anime),
    Manga(&'a Manga),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Picture {
    pub large: Option<String>,
    pub medium: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AlternativeTitles {
    pub synonyms: Option<Vec<String>>,
    pub en: Option<String>,
    pub jp: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Genre {
    pub id: u64,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, EnumString, IntoStaticStr, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Season {
    Winter,
    Spring,
    Summer,
    Fall,
    Other(String),
}

#[derive(Clone, Debug)]
pub struct TimeWrapper {
    pub time: Time,
}
#[derive(Clone, Debug)]
pub struct DateWrapper {
    pub date: Date,
}

#[derive(Clone, Debug)]
pub struct DateTimeWrapper {
    pub datetime: PrimitiveDateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Broadcast {
    pub day_of_the_week: String,
    pub start_time: Option<TimeWrapper>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Studio {
    pub id: u64,
    pub name: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MediaDetailStatistics {
    pub num_list_users: u64,
    pub status: MediaDetailStatisticsStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaDetailStatisticsStatus {
    //# TODO: check if this is correct
    #[serde(deserialize_with = "string_or_int")]
    pub watching: String,
    #[serde(deserialize_with = "string_or_int")]
    pub completed: String,
    #[serde(deserialize_with = "string_or_int")]
    pub on_hold: String,
    #[serde(deserialize_with = "string_or_int")]
    pub dropped: String,
    #[serde(deserialize_with = "string_or_int")]
    pub plan_to_watch: String,
}
// this function is used to deserialize the statistics status fields which might be received as either a string or an integer
fn string_or_int<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrIntVisitor;

    impl Visitor<'_> for StringOrIntVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an integer or a string")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }
    }

    deserializer.deserialize_any(StringOrIntVisitor)
}

pub const ALL_ANIME_AND_MANGA_FIELDS: &str = "id,title,main_picture,alternative_titles,start_date,end_date,synopsis,mean,rank,popularity,num_list_users,num_scoring_users,nsfw,genres,create_at,updated_at,media_type,status,my_list_status,num_episodes,broadcast,source,average_episode_duration,rating,pictures,background,related_anime,related_manga,recommendations,studios,statistics,num_volumes,num_chapters,authors,start_season";
pub const ALL_USER_FIELDS: &str =
    "id,name,picture,gender,birthday,location,joined_at,anime_statistics,time_zone,is_supporter";

/// Utility to convert a list of fields to a string (in the format expected by query objects)
pub fn fields_to_string(fields: &[AnimeField]) -> String {
    fields
        .iter()
        .map(|field| field.into())
        .collect::<Vec<&str>>()
        .join(",")
}

#[derive(Clone, Debug, PartialEq, EnumString, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum NSFW {
    White,
    Gray,
    Black,
    Other(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RankingInfo {
    pub rank: u64,
    pub previous_rank: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Person {
    pub id: u64,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PersonRole {
    pub node: Person,
    pub role: String,
}

macro_rules! impl_serialize_deserialize {
    (for $( $t:ty ),+) => {
        $(
        impl Serialize for $t {
            fn serialize<S>(
                &self,
                serializer: S,
            ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(self.into())
            }
        }

        impl<'de> Deserialize<'de> for $t {
            fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                match Self::from_str(s.as_str()) {
                    Ok(n) => Ok(n),
                    Err(_) => Ok(Self::Other(s)),
                }
            }

            fn deserialize_in_place<D>(
                deserializer: D,
                place: &mut Self,
            ) -> Result<(), <D as Deserializer<'de>>::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                *place = match Self::from_str(s.as_str()) {
                    Ok(n) => n,
                    Err(_) => Self::Other(s),
                };
                Ok(())
            }
        }
        )*
    };
}

impl_serialize_deserialize!(
    for
    NSFW,
    AnimeMediaType,
    AnimeStatus,
    UserWatchStatus,
    AnimeRankingType,
    MangaRankingType,
    Season,
    SortStyle,
    UserReadStatus,
    MangaMediaType,
    MangaStatus,
    RelationType
);

impl Serialize for Source {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Source {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match Self::from_str(s.as_str()) {
            Ok(n) => Ok(n),
            Err(_) => Ok(Self::Other),
        }
    }

    fn deserialize_in_place<D>(
        deserializer: D,
        place: &mut Self,
    ) -> Result<(), <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        *place = match Self::from_str(s.as_str()) {
            Ok(n) => n,
            Err(_) => Self::Other,
        };
        Ok(())
    }
}

impl Serialize for TimeWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let format = format_description::parse("[hour]:[minute]:[second]").unwrap();
        serializer.serialize_str(&self.time.format(&format).unwrap())
    }
}

impl<'de> Deserialize<'de> for TimeWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let s = String::deserialize(deserializer)?;
        let re = regex::Regex::new(r"([0-9]+):([0-9]+)").unwrap();
        if let Some(caps) = re.captures(&s) {
            let hour = caps.get(1).unwrap();
            let minute = caps.get(2).unwrap();
            let hour = match hour.as_str().parse::<u8>() {
                Ok(hour) => hour,
                Err(e) => return Err(serde::de::Error::custom(e.to_string())),
            };
            let minute = match minute.as_str().parse::<u8>() {
                Ok(minute) => minute,
                Err(e) => return Err(serde::de::Error::custom(e.to_string())),
            };
            let time = match Time::from_hms(hour, minute, 0) {
                Ok(time) => time,
                Err(e) => return Err(serde::de::Error::custom(e.to_string())),
            };
            Ok(TimeWrapper { time })
        } else {
            Err(D::Error::custom("Could not parse time"))
        }
    }

    fn deserialize_in_place<D>(
        deserializer: D,
        place: &mut Self,
    ) -> Result<(), <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let s = String::deserialize(deserializer)?;
        let format = format_description::parse("[hour]:[minute]:[second]").unwrap();
        if let Ok(time) = Time::parse(&s, &format) {
            place.time = time;
            return Ok(());
        }
        let re = regex::Regex::new(r"([0-9]+):([0-9]+)").unwrap();
        if let Some(caps) = re.captures(&s) {
            let hour = caps.get(1).unwrap();
            let minute = caps.get(2).unwrap();
            let hour = match hour.as_str().parse::<u8>() {
                Ok(hour) => hour,
                Err(e) => return Err(serde::de::Error::custom(e.to_string())),
            };
            let minute = match minute.as_str().parse::<u8>() {
                Ok(minute) => minute,
                Err(e) => return Err(serde::de::Error::custom(e.to_string())),
            };
            place.time = match Time::from_hms(hour, minute, 0) {
                Ok(time) => time,
                Err(e) => return Err(serde::de::Error::custom(e.to_string())),
            };
            Ok(())
        } else {
            Err(D::Error::custom("Could not parse time"))
        }
    }
}

impl Serialize for DateWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let format = format_description::parse("[year]-[month]-[day]").unwrap();
        serializer.serialize_str(&self.date.format(&format).unwrap())
        // serializer.serialize_str(&self.date.format("%Y-%m-%d"))
    }
}

impl<'de> Deserialize<'de> for DateWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let s = String::deserialize(deserializer)?;
        let format = format_description::parse("[year]-[month]-[day]").unwrap();
        if let Ok(date) = Date::parse(&s, &format) {
            return Ok(DateWrapper { date });
        }
        let re = regex::Regex::new(r"([0-9]+)-?([0-9]+)?").unwrap();
        if let Some(caps) = re.captures(&s) {
            let year = caps.get(1).unwrap();
            let year = match year.as_str().parse::<i32>() {
                Ok(year) => year,
                Err(e) => return Err(serde::de::Error::custom(e.to_string())),
            };
            let month = if let Some(month) = caps.get(2) {
                match month.as_str().parse::<u8>() {
                    // convert to Month type
                    Ok(month) => month,
                    Err(e) => return Err(serde::de::Error::custom(e.to_string())),
                }
            } else {
                1
            };
            let date = match Date::from_calendar_date(year, Month::try_from(month).unwrap(), 1) {
                // TODO: double check Month::try_from
                Ok(date) => date,
                Err(e) => return Err(serde::de::Error::custom(e.to_string())),
            };
            Ok(DateWrapper { date })
        } else {
            Err(D::Error::custom("Could not parse date"))
        }
    }

    fn deserialize_in_place<D>(
        deserializer: D,
        place: &mut Self,
    ) -> Result<(), <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let s = String::deserialize(deserializer)?;
        let format = format_description::parse("[year]-[month]-[day]").unwrap();
        if let Ok(date) = Date::parse(&s, &format) {
            place.date = date;
            return Ok(());
        };
        let re = regex::Regex::new(r"([0-9]+)-?([0-9]+)?").unwrap();
        if let Some(caps) = re.captures(&s) {
            let year = caps.get(1).unwrap();
            let year = match year.as_str().parse::<i32>() {
                Ok(year) => year,
                Err(e) => return Err(serde::de::Error::custom(e.to_string())),
            };
            let month = if let Some(month) = caps.get(2) {
                match month.as_str().parse::<u8>() {
                    Ok(month) => month,
                    Err(e) => return Err(serde::de::Error::custom(e.to_string())),
                }
            } else {
                1
            };
            place.date = match Date::from_calendar_date(year, Month::try_from(month).unwrap(), 1) {
                Ok(date) => date,
                Err(e) => return Err(serde::de::Error::custom(e.to_string())),
            };
            Ok(())
        } else {
            Err(D::Error::custom("Could not parse date"))
        }
    }
}

impl Serialize for DateTimeWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.datetime.format(&FORMAT).unwrap())
    }
}

impl<'de> Deserialize<'de> for DateTimeWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let s = String::deserialize(deserializer)?;
        match PrimitiveDateTime::parse(&s, &FORMAT) {
            Ok(datetime) => Ok(DateTimeWrapper { datetime }),
            Err(e) => Err(D::Error::custom(e.to_string())),
        }
    }

    fn deserialize_in_place<D>(
        deserializer: D,
        place: &mut Self,
    ) -> Result<(), <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let s = String::deserialize(deserializer)?;
        match PrimitiveDateTime::parse(&s, &FORMAT) {
            Ok(datetime) => {
                place.datetime = datetime;
                Ok(())
            }
            Err(e) => Err(D::Error::custom(e.to_string())),
        }
    }
}
