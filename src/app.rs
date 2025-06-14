#![allow(clippy::new_without_default)]
#![allow(clippy::large_enum_variant)]
use crate::api::{self, model::*};
use crate::config::app_config::AppConfig;
use crate::network::IoEvent;
use chrono::Datelike;
use image::{DynamicImage, ImageError};
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::{Block, Borders};
use ratatui::Frame;
use ratatui_image::picker::Picker;
use ratatui_image::protocol::StatefulProtocol;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::sync::mpsc::Sender;
use tui_logger::{TuiLoggerWidget, TuiWidgetState};

use strum_macros::IntoStaticStr;
use tracing::warn;
use tui_scrollview::ScrollViewState;
const DEFAULT_ROUTE: Route = Route {
    data: None,
    block: ActiveDisplayBlock::Empty, //afterTest: change to empty
    title: String::new(),
    image: None,
};

pub const DISPLAY_RAWS_NUMBER: usize = 5;

pub const SEASONS: [&str; 4] = ["Winter", "Spring", "Summer", "Fall"];

pub const DISPLAY_COLUMN_NUMBER: usize = 3;

pub const ANIME_OPTIONS: [&str; 3] = ["Seasonal", "Ranking", "Suggested"];

pub const USER_OPTIONS: [&str; 3] = ["Stats", "AnimeList", "MangaList"];

pub const GENERAL_OPTIONS: [&str; 3] = ["Help", "About", "Quit"];

pub const USER_WATCH_STATUS: [&str; 5] = [
    "Watching",
    "Completed",
    "On Hold",
    "Dropped",
    "Plan to Watch",
];
pub const USER_READ_STATUS: [&str; 5] =
    ["Reading", "Completed", "On Hold", "Dropped", "Plan To Read"];

pub const ANIME_OPTIONS_RANGE: std::ops::Range<usize> = 0..3;

pub const USER_OPTIONS_RANGE: std::ops::Range<usize> = 3..6;

pub const GENERAL_OPTIONS_RANGE: std::ops::Range<usize> = 6..9;

pub const RATING_OPTIONS: [&str; 11] = [
    "None",
    "(1) Appalling",
    "(2) Horrible",
    "(3) Very Bad",
    "(4) Bad",
    "(5) Average",
    "(6) Fi\ne",
    "(7) Good",
    "(8) Very Good",
    "(9) Great",
    "(10) Masterpiece",
];

pub const ANIME_RANKING_TYPES: [&str; 9] = [
    "All",
    "Airing",
    "Upcoming",
    "Movie",
    "Popularity",
    "Special",
    "TV",
    "OVA",
    "Favorite",
];

pub const MANGA_RANKING_TYPES: [&str; 9] = [
    "All",
    "Manga",
    "Manhwa",
    "Popularity",
    "Novels",
    "Oneshots",
    "Doujin",
    "Manhua",
    "Favorite",
];

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ActiveBlock {
    Input,
    DisplayBlock,
    Anime,
    Option,
    User,
    TopThree,
    Error,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ActiveDisplayBlock {
    SearchResultBlock,
    Help,
    UserInfo,
    UserAnimeList,
    UserMangaList,
    Suggestions,
    Seasonal,
    AnimeRanking,
    MangaRanking,
    Loading,
    Error,
    Empty,
    AnimeDetails,
    MangaDetails,
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SelectedSearchTab {
    Anime,
    Manga,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub anime: Option<Page<Anime>>,
    pub manga: Option<Page<Manga>>,
    pub selected_tab: SelectedSearchTab,
    pub selected_display_card_index: Option<usize>,
    pub max_index: u16,
    pub max_page: u16,
}

#[derive(Clone)]
pub struct ScrollablePages<T> {
    index: usize,
    pages: Vec<T>,
}

impl<T> ScrollablePages<T> {
    pub fn new() -> Self {
        Self {
            index: 0,
            pages: vec![],
        }
    }

    pub fn get_results(&self, at_index: Option<usize>) -> Option<&T> {
        self.pages.get(at_index.unwrap_or(self.index))
    }

    pub fn get_mut_results(&mut self, at_index: Option<usize>) -> Option<&mut T> {
        self.pages.get_mut(at_index.unwrap_or(self.index))
    }

    pub fn add_pages(&mut self, new_pages: T) {
        self.pages.push(new_pages);
        self.index = self.pages.len() - 1;
    }
}

pub struct Library {
    pub selected_index: usize,
    pub saved_anime: ScrollablePages<Page<Anime>>,
    pub saved_manga: ScrollablePages<Page<Manga>>,
}

#[derive(Debug)]
pub struct Navigator {
    pub history: Vec<u16>,
    pub index: usize,
    pub data: HashMap<u16, Route>,
    pub last_id: u16,
}

impl Navigator {
    // explain every thing about the navigation system:
    /*
    navigator has the home route at initialization
    it has a history of routes, where the first route is always the home route
    each page has an id and it is mapped to its data in the data hashmap
    */
    pub fn new() -> Self {
        let mut data = HashMap::new();
        data.insert(0, DEFAULT_ROUTE);
        Self {
            history: vec![0],
            index: 0,
            data,
            last_id: 0,
        }
    }

    pub fn add_existing_route(&mut self, id: u16) {
        self.history.push(id);
        self.index = self.history.len() - 1;
    }

    pub fn add_route(&mut self, r: Route) {
        self.last_id += 1;
        self.data.insert(self.last_id, r);
        self.history.push(self.last_id);
        self.index = self.history.len() - 1;
    }
    pub fn validate_state(&self) -> bool {
        // Check if index is within bounds
        if self.index >= self.history.len() {
            warn!(
                "Navigation state invalid: index {} >= history length {}",
                self.index,
                self.history.len()
            );
            return false;
        }

        // // Check if current route ID exists in data
        // let current_id = self.history[self.index];
        // if !self.data.contains_key(&current_id) {
        //     println!("Navigation state invalid: current route ID {} not in data map",
        //              current_id);
        //     return false;
        // }

        // Check if all history route IDs exist in data
        for &route_id in &self.history {
            if !self.data.contains_key(&route_id) {
                warn!(
                    "Navigation state invalid: history route ID {} not in data map",
                    route_id
                );
                return false;
            }
        }

        true
    }
    pub fn remove_old_history(&mut self) {
        // when the history length exceeds the limit, we remove the oldest page which is 1 (0 is the home page)

        self.history.remove(1);
        self.clear_unused_data();
    }

    /// Removes route data for routes that are no longer in the navigation history.
    /// This prevents memory leaks by cleaning up orphaned route data after history modifications.
    /// Should be called after operations that remove entries from the history vector.
    pub fn clear_unused_data(&mut self) {
        let active_routes: HashSet<u16> = self.history.iter().copied().collect();
        self.data.retain(|k, _| active_routes.contains(k));
    }

    pub fn get_current_title(&self) -> &String {
        let id = self.history[self.index];
        &self.data[&id].title
    }

    pub fn get_current_block(&self) -> ActiveDisplayBlock {
        let id = self.history[self.index];
        self.data[&id].block
    }
}

pub struct App {
    pub io_tx: Option<Sender<IoEvent>>,
    pub app_config: AppConfig,
    pub is_loading: bool,
    pub api_error: String,
    pub search_results: SearchResult,
    pub size: Rect,
    pub input: Vec<char>,
    pub input_cursor_position: u16,
    pub input_idx: usize,
    pub library: Library,
    pub help_menu_offset: u32,
    pub help_menu_page: u32,
    pub help_menu_max_lines: u32,
    pub help_docs_size: u32,
    // logger:
    pub logger_state: TuiWidgetState,
    // exit:
    pub exit_flag: bool,
    pub exit_confirmation_popup: bool,
    // image:
    pub picker: Option<Picker>,
    pub media_image: Option<(String, u32, u32)>,
    pub image_state: Option<StatefulProtocol>,
    // state:
    pub active_block: ActiveBlock,
    pub active_display_block: ActiveDisplayBlock,
    pub navigator: Navigator,
    pub display_block_title: String,
    pub popup: bool,
    pub anime_details_synopsys_scroll_view_state: ScrollViewState,
    pub anime_details_info_scroll_view_state: ScrollViewState,
    pub manga_details_info_scroll_view_state: ScrollViewState,
    pub manga_details_synopsys_scroll_view_state: ScrollViewState,
    // top three bar:
    pub top_three_anime: TopThreeAnime,
    pub top_three_manga: TopThreeManga,
    pub active_top_three: TopThreeBlock,
    pub active_top_three_anime: Option<AnimeRankingType>,
    pub active_top_three_manga: Option<MangaRankingType>,
    pub selected_top_three: u32,
    pub available_anime_ranking_types: Vec<AnimeRankingType>,
    pub available_manga_ranking_types: Vec<MangaRankingType>,
    pub active_anime_rank_index: u32,
    pub active_manga_rank_index: u32,
    // detail
    pub anime_details: Option<Anime>,
    pub manga_details: Option<Manga>,
    pub active_detail_popup: DetailPopup,
    pub active_anime_detail_block: ActiveAnimeDetailBlock,
    pub active_manga_detail_block: ActiveMangaDetailBlock,
    // detail popup
    pub popup_post_req_success: bool,
    pub result_popup: bool,
    pub popup_is_loading: bool,
    pub popup_post_req_success_message: Option<String>,
    pub selected_popup_status: u8,
    pub selected_popup_rate: u8,
    pub temp_popup_num: u16,
    // seasonal
    pub anime_season: Seasonal,
    //ranking
    pub anime_ranking_data: Option<Ranking<RankingAnimePair>>,
    pub anime_ranking_type: AnimeRankingType,
    pub manga_ranking_data: Option<Ranking<RankingMangaPair>>,
    pub manga_ranking_type: MangaRankingType,
    pub anime_ranking_type_index: u8,
    pub manga_ranking_type_index: u8,
    //profile:
    pub user_profile: Option<UserInfo>,
    // use UserWatchStatus to determine the current tab
    pub anime_list_status: Option<UserWatchStatus>,
    // use UserReadStatus to determine the current tab
    pub manga_list_status: Option<UserReadStatus>,
    // to track pagination (with local data)
    pub start_card_list_index: u16,
}
#[derive(Debug, Clone)]
pub enum DetailPopup {
    AddToList,
    Rate,
    Episodes,
    Chapters,
    Volumes,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ActiveAnimeDetailBlock {
    Synopsis,
    SideInfo,
    AddToList,
    Rate,
    Episodes,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ActiveMangaDetailBlock {
    Synopsis,
    SideInfo,
    AddToList,
    Rate,
    Chapters,
    Volumes,
}

pub struct Seasonal {
    pub anime_season: AnimeSeason,
    pub popup_season_highlight: bool,
    pub anime_sort: SortStyle,
    pub selected_season: u8,
    pub selected_year: u16,
}

#[derive(Debug, Clone, IntoStaticStr)]
pub enum TopThreeBlock {
    Anime(AnimeRankingType),
    Manga(MangaRankingType),
    Loading(RankingType),
    Error(RankingType),
}

#[derive(Debug, Clone, Default)]
pub struct TopThreeManga {
    pub all: Option<[Manga; 3]>,
    pub manga: Option<[Manga; 3]>,
    pub novels: Option<[Manga; 3]>,
    pub oneshots: Option<[Manga; 3]>,
    pub doujin: Option<[Manga; 3]>,
    pub manhwa: Option<[Manga; 3]>,
    pub manhua: Option<[Manga; 3]>,
    pub popular: Option<[Manga; 3]>,
    pub favourite: Option<[Manga; 3]>,
}

#[derive(Debug, Clone, Default)]
pub struct TopThreeAnime {
    pub airing: Option<[Anime; 3]>,
    pub upcoming: Option<[Anime; 3]>,
    pub popular: Option<[Anime; 3]>,
    pub all: Option<[Anime; 3]>,
    pub tv: Option<[Anime; 3]>,
    pub ova: Option<[Anime; 3]>,
    pub movie: Option<[Anime; 3]>,
    pub special: Option<[Anime; 3]>,
    pub favourite: Option<[Anime; 3]>,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum Data {
    SearchResult(SearchResult),
    Suggestions(SearchResult),
    UserInfo(UserInfo),
    Anime(Anime),
    Manga(Manga),
    UserAnimeList(UserAnimeList),
    UserMangaList(UserMangaList),
    AnimeRanking(Ranking<RankingAnimePair>),
    MangaRanking(Ranking<RankingMangaPair>),
}

#[derive(Debug, Clone)]
pub struct UserAnimeList {
    pub anime_list: Page<Anime>,
    pub status: Option<UserWatchStatus>,
}
#[derive(Debug, Clone)]
pub struct UserMangaList {
    pub manga_list: Page<Manga>,
    pub status: Option<UserReadStatus>,
}

#[derive(Debug, Clone)]
pub struct Route {
    pub data: Option<Data>,
    pub block: ActiveDisplayBlock,
    pub title: String,
    pub image: Option<(String, u32, u32)>,
}

impl App {
    pub fn new(io_tx: Sender<IoEvent>, app_config: AppConfig) -> Self {
        // let can_render =

        let year = chrono::Utc::now().year();
        let season = get_season();
        let selected_season = get_selected_season(&season);
        let picker_res = Picker::from_query_stdio();
        let mut picker: Option<Picker> = None;
        if picker_res.is_ok() {
            picker = Some(picker_res.unwrap());
        }
        Self {
            io_tx: Some(io_tx),
            anime_season: Seasonal {
                anime_season: AnimeSeason {
                    year: year as u64,
                    season,
                },
                anime_sort: SortStyle::ListScore,
                popup_season_highlight: true,
                selected_season,
                selected_year: year as u16,
            },
            // logger:
            logger_state: TuiWidgetState::default().set_default_display_level(app_config.log_level),

            available_anime_ranking_types: app_config.top_three_anime_types.clone(),
            active_top_three: TopThreeBlock::Anime(app_config.top_three_anime_types[0].clone()),
            available_manga_ranking_types: app_config.top_three_manga_types.clone(),
            app_config,
            is_loading: false,
            api_error: String::new(),
            search_results: SearchResult {
                anime: None,
                manga: None,
                selected_display_card_index: Some(0),
                selected_tab: SelectedSearchTab::Anime,
                max_index: 15,
                max_page: 0,
            },
            size: Rect::default(),
            input: vec![],
            input_cursor_position: 0,
            input_idx: 0,
            library: Library {
                saved_anime: ScrollablePages::new(),
                saved_manga: ScrollablePages::new(),
                selected_index: 9, // out of range to show nothing
            },
            help_menu_offset: 0,
            help_menu_page: 0,
            help_menu_max_lines: 0,
            help_docs_size: 0,
            active_block: ActiveBlock::DisplayBlock,
            active_display_block: DEFAULT_ROUTE.block,
            navigator: Navigator::new(),
            // top three
            top_three_anime: TopThreeAnime::default(),
            top_three_manga: TopThreeManga::default(),
            selected_top_three: 0, // out of index to select nothing
            active_top_three_anime: None,
            active_top_three_manga: None,
            active_anime_rank_index: 0,
            active_manga_rank_index: 0,
            // ranking page
            anime_ranking_data: None,
            anime_ranking_type: AnimeRankingType::All,
            anime_ranking_type_index: 0,
            manga_ranking_data: None,
            manga_ranking_type: MangaRankingType::All,
            manga_ranking_type_index: 0,
            // anime list
            anime_list_status: None,
            // manga list
            manga_list_status: None,
            // detail
            active_detail_popup: DetailPopup::AddToList,
            active_anime_detail_block: ActiveAnimeDetailBlock::Synopsis,
            active_manga_detail_block: ActiveMangaDetailBlock::Synopsis,
            anime_details: None,
            manga_details: None,
            user_profile: None,
            display_block_title: String::new(),
            // detail popup
            selected_popup_status: 0,
            selected_popup_rate: 0,
            temp_popup_num: 0,
            popup_post_req_success: false,
            popup_post_req_success_message: None,
            popup_is_loading: false,
            result_popup: false,
            popup: false,
            // image:
            media_image: None,
            picker,
            image_state: None,
            anime_details_synopsys_scroll_view_state: ScrollViewState::default(),
            anime_details_info_scroll_view_state: ScrollViewState::default(),
            manga_details_info_scroll_view_state: ScrollViewState::default(),
            manga_details_synopsys_scroll_view_state: ScrollViewState::default(),
            start_card_list_index: 0,
            // exit:
            exit_flag: false,
            exit_confirmation_popup: false,
        }
    }

    pub fn render_logs(&mut self, f: &mut Frame, area: ratatui::layout::Rect) {
        let logs = TuiLoggerWidget::default()
            .block(Block::default().title("Logs").borders(Borders::ALL))
            .style(Style::default().fg(self.app_config.theme.text))
            .state(&self.logger_state);
        f.render_widget(logs, area);
    }

    pub fn write_error(&mut self, e: api::Error) {
        match e {
            api::Error::NoAuth => {
                self.api_error = "Auth Error, Please reload the App".to_string();
            }
            api::Error::TimedOut => {
                self.api_error = "Conntection Timed Out, Please try again".to_string();
            }
            api::Error::Unknown => {
                self.api_error = "Check you internet connection".to_string();
            }
            api::Error::NoBody => {
                self.api_error = "there is No Body".to_string();
            }
            api::Error::ParseError(e) => {
                self.api_error = format!("Parse Error: {}", e);
            }
            api::Error::QuerySerializeError(e) => {
                self.api_error = format!("Query Serialize Error: {}", e);
            }
            api::Error::HttpError(e) => {
                self.api_error = format!("Http Error: {}", e);
            }
        }
    }

    pub fn get_top_three(&mut self) {
        let _ = &self.dispatch(IoEvent::GetTopThree(self.active_top_three.clone()));
    }

    pub fn dispatch(&mut self, event: IoEvent) {
        self.is_loading = true;
        if let Some(io_tx) = &self.io_tx {
            if let Err(e) = io_tx.send(event) {
                self.is_loading = false;
                warn!("Error from dispatch {}", e);
            }
        };
    }

    pub fn clear_route_before_push(&mut self) {
        // here we take the current index (position) and delete everything after it in the history
        let index = self.navigator.index;

        if index < self.navigator.history.len() - 1 {
            for _ in index + 1..self.navigator.history.len() {
                self.navigator.history.pop();
            }
        }
        self.navigator.clear_unused_data();
    }

    fn push_existing_route(&mut self, id: u16) {
        //
        if !self.navigator.data.contains_key(&id) {
            warn!("Route ID {} does not exist in data map, cannot push", id);
            self.navigator.index = 0; // reset index to home
            return;
        }
        self.clear_route_before_push();
        self.navigator.add_existing_route(id);
    }

    pub fn push_navigation_stack(&mut self, r: Route) {
        self.clear_route_before_push();
        self.navigator.add_route(r);
        self.remove_old_history();
    }

    fn remove_old_history(&mut self) {
        // when the history length exceeds the limit, we remove the oldest page wich is 1 (0 is the home page)
        if self.navigator.history.len() - 1 > self.app_config.navigation_stack_limit as usize {
            self.navigator.remove_old_history();
        }
    }

    pub fn get_current_route(&self) -> Option<&Route> {
        let index = self.navigator.index;

        // Ensure the index is within bounds
        if index >= self.navigator.history.len() {
            warn!("Error: Navigation index {} is out of bounds", index);
            return None;
        }

        let id = self.navigator.history[index];

        // Ensure the route ID exists in the data map
        match self.navigator.data.get(&id) {
            Some(route) => Some(route),
            None => {
                warn!("Error: Route ID {} not found in data map", id);
                None
            }
        }
    }

    pub fn calculate_help_menu_offset(&mut self) {
        let old_offset = self.help_menu_offset;
        if self.help_menu_max_lines < self.help_docs_size {
            self.help_menu_offset = self.help_menu_page * self.help_menu_max_lines;
        }
        if self.help_menu_offset > self.help_docs_size {
            self.help_menu_offset = old_offset;
            self.help_menu_page -= 1;
        }
    }

    pub fn load_previous_route(&mut self) {
        if self.popup {
            // reset everything
            self.popup = false;
            self.result_popup = false;
            self.popup_post_req_success = false;
            self.popup_post_req_success_message = None;
            return;
        }

        if self.navigator.index == 1 {
            self.active_display_block = ActiveDisplayBlock::Empty;
            self.display_block_title = "Home".to_string();
            self.navigator.index = 0;
            return;
        }

        if self.active_display_block == ActiveDisplayBlock::Loading {
            return;
        }

        if self.active_display_block == ActiveDisplayBlock::Error
            || self.active_display_block == ActiveDisplayBlock::Help
        {
            self.active_display_block = self.navigator.get_current_block();
            return;
        }
        if self.navigator.index == 0 {
            return;
        }
        let i = self.navigator.index.saturating_sub(1);
        self.load_state_data(i);
    }

    pub fn load_next_route(&mut self) {
        if self.popup {
            return;
        }
        if self.navigator.index >= self.navigator.history.len() {
            // if we exceeded the history length, we reset the index to the last route
            warn!("Navigator index exceeded history length, resetting to last route");
            self.navigator.index = self.navigator.history.len().saturating_sub(2);
        }

        if self.navigator.index == self.navigator.history.len() - 1 {
            // if we are at the last route, we do nothing
            return;
        }

        self.load_state_data(self.navigator.index + 1);
    }

    pub fn load_route(&mut self, id: u16) {
        self.push_existing_route(id);
        self.load_state_data(self.navigator.history.len() - 1);
    }

    fn load_state_data(&mut self, i: usize) {
        warn!("{:?}", &self.navigator.history);
        if !self.navigator.validate_state() {
            warn!("Invalid navigation state");
            self.navigator.index = 0; // reset index to home
            return;
        }
        if i >= self.navigator.history.len() {
            return;
        }
        let route_id = self.navigator.history[i];
        if !self.navigator.data.contains_key(&route_id) {
            warn!(
                "Error: Route ID {} not found in data map when loading state",
                route_id
            );
            self.navigator.index = 0; // reset index to home
            self.navigator.history.remove(i);
            return;
        }
        self.navigator.index = i;
        let route = match self.get_current_route() {
            Some(route) => route.clone(),
            None => return,
        };

        let data = route.data.clone();
        match data {
            Some(data) => {
                match data {
                    Data::SearchResult(d) => {
                        self.search_results.anime = d.anime.clone();
                        self.search_results.manga = d.manga.clone();
                    }

                    Data::Suggestions(d) => {
                        self.search_results = d.clone();
                    }

                    Data::Anime(d) => {
                        // self.set_image_from_route(route.as_ref().unwrap(), Some(d.clone()));
                        self.anime_details = Some(d.clone());

                        if let Some(image) = &route.image {
                            self.media_image = Some(image.clone());
                            self.image_state = Some(
                                self.picker
                                    .as_ref()
                                    .unwrap()
                                    .new_resize_protocol(self.get_picture_from_cache().unwrap()),
                            );
                        }
                    }

                    Data::Manga(d) => {
                        self.manga_details = Some(d.clone());
                        if let Some(image) = &route.image {
                            self.media_image = Some(image.clone());
                            self.image_state = Some(
                                self.picker
                                    .as_ref()
                                    .unwrap()
                                    .new_resize_protocol(self.get_picture_from_cache().unwrap()),
                            );
                        }
                    }

                    Data::AnimeRanking(d) => {
                        self.anime_ranking_data = Some(d.clone());
                    }

                    Data::MangaRanking(d) => {
                        self.manga_ranking_data = Some(d.clone());
                    }

                    Data::UserInfo(d) => self.user_profile = Some(d.clone()),

                    Data::UserAnimeList(d) => {
                        self.anime_list_status = d.status.clone();
                        self.search_results.anime = Some(d.anime_list.clone());
                    }

                    Data::UserMangaList(d) => {
                        self.manga_list_status = d.status.clone();
                        self.search_results.manga = Some(d.manga_list.clone());
                    }
                }

                self.active_display_block = self.navigator.get_current_block();
                self.display_block_title = self.navigator.get_current_title().clone();
                self.active_block = ActiveBlock::DisplayBlock;
            }

            None => {
                self.active_display_block = ActiveDisplayBlock::Empty;
                self.display_block_title = "No data".to_string();
            }
        }
    }

    pub fn next_anime_list_status(&self) -> Option<UserWatchStatus> {
        match &self.anime_list_status {
            Some(s) => match s {
                UserWatchStatus::Watching => Some(UserWatchStatus::Completed),
                UserWatchStatus::Completed => Some(UserWatchStatus::OnHold),
                UserWatchStatus::OnHold => Some(UserWatchStatus::Dropped),
                UserWatchStatus::Dropped => Some(UserWatchStatus::PlanToWatch),
                UserWatchStatus::PlanToWatch => None,
                UserWatchStatus::Other(_) => None,
            },
            None => Some(UserWatchStatus::Watching),
        }
    }

    pub fn previous_anime_list_status(&self) -> Option<UserWatchStatus> {
        match &self.anime_list_status {
            Some(s) => match s {
                UserWatchStatus::Watching => None,
                UserWatchStatus::Completed => Some(UserWatchStatus::Watching),
                UserWatchStatus::OnHold => Some(UserWatchStatus::Completed),
                UserWatchStatus::Dropped => Some(UserWatchStatus::OnHold),
                UserWatchStatus::PlanToWatch => Some(UserWatchStatus::Dropped),
                UserWatchStatus::Other(_) => Some(UserWatchStatus::PlanToWatch),
            },
            None => Some(UserWatchStatus::Watching),
        }
    }

    pub fn get_picture_from_cache(&self) -> Result<DynamicImage, ImageError> {
        // all images are stored in $HOME?/.cache/mal-cli/images/
        let file_name = self.media_image.as_ref().unwrap().0.clone();
        let file_path = self.app_config.paths.picture_cache_dir_path.join(file_name);
        let image = image::ImageReader::open(file_path)?.decode()?;
        Ok(image)
    }
    pub fn reset_result_index(&mut self) {
        // reset the selected index in the search results
        self.search_results.selected_display_card_index = Some(0);
        self.start_card_list_index = 0;
    }
}

fn get_season() -> Season {
    let month = chrono::Utc::now().month();
    match month {
        3..=5 => Season::Spring,
        6..=8 => Season::Summer,
        9..=11 => Season::Fall,
        _ => Season::Winter,
    }
}

fn get_selected_season(season: &Season) -> u8 {
    match *season {
        Season::Winter => 0,
        Season::Spring => 1,
        Season::Summer => 2,
        Season::Fall => 3,
        Season::Other(_) => panic!("no season selected"),
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::config::app_config::AppConfig;
    pub fn get_app() -> App {
        let config = AppConfig::load();
        let (sync_io_tx, _) = std::sync::mpsc::channel::<IoEvent>();

        let mut app = App::new(sync_io_tx, config.unwrap());
        let route = Route {
            data: None,
            block: ActiveDisplayBlock::Empty,
            title: "Home".to_string(),
            image: None,
        };
        app.push_navigation_stack(route.clone());
        app.push_navigation_stack(route.clone());
        app.push_navigation_stack(route.clone());
        app.push_navigation_stack(route);
        app
    }
    #[test]
    fn test_navigation_push() {
        let app = get_app();

        assert_eq!(app.navigator.history.len(), 5);
        assert_eq!(app.navigator.index, 4);
    }

    #[test]
    fn test_backward_navigation() {
        let mut app = get_app();
        assert_eq!(app.navigator.index, 4);
        app.load_previous_route();
        assert_eq!(app.navigator.index, 3);
        app.load_previous_route();
        assert_eq!(app.navigator.index, 2);
        app.load_previous_route();
        assert_eq!(app.navigator.index, 1);
        app.load_previous_route();
        assert_eq!(app.navigator.index, 0);
    }
    #[test]
    fn test_forward_navigation() {
        let mut app = get_app();
        app.navigator.index = 0;
        app.load_next_route();
        assert_eq!(app.navigator.index, 1);
        app.load_next_route();
        assert_eq!(app.navigator.index, 2);
        app.load_next_route();
        assert_eq!(app.navigator.index, 3);
        app.load_next_route();
        assert_eq!(app.navigator.index, 4);
    }
}
