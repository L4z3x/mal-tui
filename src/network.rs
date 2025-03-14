use crate::{
    api::{self, model::*, GetAnimeRankingQuery, GetMangaRankingQuery, GetSeasonalAnimeQuery},
    app::{ActiveDisplayBlock, App, Route, SelectedSearchTab, TopThreeBlock},
    auth::OAuth,
};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub enum IoEvent {
    GetSearchResults(String),
    GetAnimeSearchResults(String),
    GetMangaSearchResults(String),
    GetAnime(String),
    GetAnimeRanking(String),
    GetSeasonalAnime(String),
    GetSuggestedAnime(String),
    UpdateAnimeListStatus(String),
    DeleteAnimeListStatus(String),
    GetAnimeList(String),
    GetManga(String),
    GetMangaRanking(String),
    UpdateMangaListStatus(String),
    DeleteMangaListStatus(String),
    GetMangaList(String),
    GetUserInfo(String),
    GetTopThree(TopThreeBlock),
}

#[derive(Clone)]
pub struct Network<'a> {
    oauth: OAuth,
    large_search_limit: u64,
    small_search_limit: u64,
    app: &'a Arc<Mutex<App>>,
}

impl<'a> Network<'a> {
    pub fn new(oauth: OAuth, app: &'a Arc<Mutex<App>>) -> Self {
        Self {
            oauth,
            large_search_limit: 20,
            small_search_limit: 4,
            app,
        }
    }

    pub async fn handle_network_event(&mut self, io_event: IoEvent) {
        match io_event {
            IoEvent::GetSearchResults(q) => {
                self.get_search_results(q).await;
            }
            // IoEvent::GetAnimeSearchResults(String) => {}
            // IoEvent::GetMangaSearchResults(String) => {}
            // IoEvent::GetAnime(String) => {}
            // IoEvent::GetAnimeRanking(String) => {}
            // IoEvent::GetSeasonalAnime(String) => {}
            // IoEvent::GetSuggestedAnime(String) => {}
            // IoEvent::UpdateAnimeListStatus(String) => {}
            // IoEvent::DeleteAnimeListStatus(String) => {}
            // IoEvent::GetAnimeList(String) => {}
            // IoEvent::GetManga(String) => {}
            // IoEvent::GetMangaRanking(String) => {}
            // IoEvent::UpdateMangaListStatus(String) => {}
            // IoEvent::DeleteMangaListStatus(String) => {}
            // IoEvent::GetMangaList(String) => {}
            // IoEvent::GetUserInfo(String) => {}
            IoEvent::GetTopThree(r) => self.get_top_three(r).await,
            _ => (),
        }

        let mut app = self.app.lock().await;
        app.is_loading = false
    }

    async fn get_top_three(&mut self, ranking_type: TopThreeBlock) {
        match ranking_type {
            TopThreeBlock::Anime(r) => self.get_anime_top_three(r).await,
            TopThreeBlock::Manga(r) => self.get_manga_top_three(r).await,
            _ => (),
        }
    }

    async fn get_anime_top_three(&mut self, rank_type: AnimeRankingType) {
        self.oauth.refresh().unwrap();
        let mut app = self.app.lock().await;
        let query = GetAnimeRankingQuery {
            ranking_type: rank_type.clone(),
            fields: Some(ALL_ANIME_AND_MANGA_FIELDS.to_string()),
            limit: 3,
            nsfw: app.app_config.nsfw,
            offset: 0,
        };
        match api::get_anime_ranking(&query, &self.oauth).await {
            Ok(result) => {
                match &rank_type {
                    AnimeRankingType::Airing => {
                        app.top_three_anime.airing = Some([
                            result.data[0].node.clone(),
                            result.data[1].node.clone(),
                            result.data[2].node.clone(),
                        ]);
                    }
                    AnimeRankingType::All => {
                        app.top_three_anime.all = Some([
                            result.data[0].node.clone(),
                            result.data[1].node.clone(),
                            result.data[2].node.clone(),
                        ])
                    }
                    AnimeRankingType::Upcoming => {
                        app.top_three_anime.upcoming = Some([
                            result.data[0].node.clone(),
                            result.data[1].node.clone(),
                            result.data[2].node.clone(),
                        ]);
                    }
                    AnimeRankingType::ByPopularity => {
                        app.top_three_anime.popular = Some([
                            result.data[0].node.clone(),
                            result.data[1].node.clone(),
                            result.data[2].node.clone(),
                        ]);
                    }
                    AnimeRankingType::Favorite => {
                        app.top_three_anime.favourite = Some([
                            result.data[0].node.clone(),
                            result.data[1].node.clone(),
                            result.data[2].node.clone(),
                        ]);
                    }
                    AnimeRankingType::Movie => {
                        app.top_three_anime.movie = Some([
                            result.data[0].node.clone(),
                            result.data[1].node.clone(),
                            result.data[2].node.clone(),
                        ]);
                    }
                    AnimeRankingType::OVA => {
                        app.top_three_anime.ova = Some([
                            result.data[0].node.clone(),
                            result.data[1].node.clone(),
                            result.data[2].node.clone(),
                        ]);
                    }
                    AnimeRankingType::TV => {
                        app.top_three_anime.tv = Some([
                            result.data[0].node.clone(),
                            result.data[1].node.clone(),
                            result.data[2].node.clone(),
                        ]);
                    }
                    AnimeRankingType::Special => {
                        app.top_three_anime.special = Some([
                            result.data[0].node.clone(),
                            result.data[1].node.clone(),
                            result.data[2].node.clone(),
                        ]);
                    }
                    AnimeRankingType::Other(_s) => {}
                }
                app.active_top_three = TopThreeBlock::Anime(
                    app.active_top_three_anime
                        .as_ref()
                        .unwrap_or(&app.anime_ranking_types[0])
                        .clone(),
                );
            }
            Err(e) => {
                app.write_error(e);
                app.active_top_three = TopThreeBlock::Error(RankingType::AnimeRankingType(
                    app.active_top_three_anime
                        .as_ref()
                        .unwrap_or(&app.anime_ranking_types[0])
                        .clone(),
                ));
                return;
            }
        }
    }

    async fn get_manga_top_three(&mut self, rank_type: MangaRankingType) {
        self.oauth.refresh().unwrap();
        let mut app = self.app.lock().await;
        let query = GetMangaRankingQuery {
            ranking_type: rank_type.clone(),
            fields: Some(ALL_ANIME_AND_MANGA_FIELDS.to_string()),
            limit: 3,
            nsfw: app.app_config.nsfw,
            offset: 0,
        };

        match api::get_manga_ranking(&query, &self.oauth).await {
            Ok(results) => {
                match &rank_type {
                    MangaRankingType::All => {
                        app.top_three_manga.all = Some([
                            results.data[0].node.clone(),
                            results.data[1].node.clone(),
                            results.data[2].node.clone(),
                        ]);
                    }
                    MangaRankingType::Manga => {
                        app.top_three_manga.manga = Some([
                            results.data[0].node.clone(),
                            results.data[1].node.clone(),
                            results.data[2].node.clone(),
                        ]);
                    }
                    MangaRankingType::Novels => {
                        app.top_three_manga.novels = Some([
                            results.data[0].node.clone(),
                            results.data[1].node.clone(),
                            results.data[2].node.clone(),
                        ]);
                    }
                    MangaRankingType::OneShots => {
                        app.top_three_manga.oneshots = Some([
                            results.data[0].node.clone(),
                            results.data[1].node.clone(),
                            results.data[2].node.clone(),
                        ]);
                    }
                    MangaRankingType::Doujinshi => {
                        app.top_three_manga.doujin = Some([
                            results.data[0].node.clone(),
                            results.data[1].node.clone(),
                            results.data[2].node.clone(),
                        ]);
                    }
                    MangaRankingType::Manhwa => {
                        app.top_three_manga.manhwa = Some([
                            results.data[0].node.clone(),
                            results.data[1].node.clone(),
                            results.data[2].node.clone(),
                        ]);
                    }
                    MangaRankingType::Manhua => {
                        app.top_three_manga.manhua = Some([
                            results.data[0].node.clone(),
                            results.data[1].node.clone(),
                            results.data[2].node.clone(),
                        ]);
                    }
                    MangaRankingType::ByPopularity => {
                        app.top_three_manga.popular = Some([
                            results.data[0].node.clone(),
                            results.data[1].node.clone(),
                            results.data[2].node.clone(),
                        ]);
                    }
                    MangaRankingType::Other(_) => {}
                }
                app.active_top_three = TopThreeBlock::Manga(
                    app.active_top_three_manga
                        .as_ref()
                        .unwrap_or(&app.manga_ranking_types[0])
                        .clone(),
                );
            }

            Err(e) => {
                app.write_error(e);
                app.active_top_three = TopThreeBlock::Error(RankingType::MangaRankingType(
                    app.active_top_three_manga
                        .as_ref()
                        .unwrap_or(&app.manga_ranking_types[0])
                        .clone(),
                ));
                return;
            }
        }
    }

    async fn get_seasonal(&mut self, season: &AnimeSeason, query: GetSeasonalAnimeQuery) {
        self.oauth.refresh().unwrap();
        let mut app = self.app.lock().await;

        match api::get_seasonal_anime(season, &query, &self.oauth).await {
            Ok(result) => {}
            Err(e) => {}
        }
    }

    // TODO: Add actual error handling
    // async fn handle_error(&mut self, e: api::Error, app: &mut App) {
    //     app.write_error(e);
    // }

    async fn get_search_results(&mut self, q: String) {
        self.oauth.refresh().unwrap();
        let mut app = self.app.lock().await;

        let anime_query = api::GetAnimeListQuery {
            q: q.clone(),
            limit: self.large_search_limit,
            offset: 0,
            nsfw: app.app_config.nsfw,
            fields: Some(ALL_ANIME_AND_MANGA_FIELDS.to_string()),
        };

        let manga_query = api::GetMangaListQuery {
            q: q.clone(),
            limit: self.large_search_limit,
            offset: 0,
            nsfw: app.app_config.nsfw,
            fields: Some(ALL_ANIME_AND_MANGA_FIELDS.to_string()),
        };

        match api::get_anime_list(&anime_query, &self.oauth).await {
            Ok(results) => {
                app.search_results.anime = Some(results);
            }
            Err(e) => {
                app.write_error(e);
                return;
            }
        };

        match api::get_manga_list(&manga_query, &self.oauth).await {
            Ok(results) => {
                app.search_results.manga = Some(results);
            }
            Err(e) => {
                app.write_error(e);
                return;
            }
        };
        app.navigation_index += 1;
        let route = Route {
            anime: None,
            manga: None,
            results: Some(app.search_results.clone()),
            user: None,
            block: ActiveDisplayBlock::SearchResultBlock,
            title: format!("Search Results: {}", q.clone()).to_string(),
        };
        app.push_navigation_stack(route);

        app.search_results.selected_tab = SelectedSearchTab::Anime;
        app.active_display_block = ActiveDisplayBlock::SearchResultBlock;
        app.display_block_title = format!("Search Results: {}", q).to_string()
    }
}
