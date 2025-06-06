use super::common;
use crate::app::{ActiveDisplayBlock, App, Data, ANIME_OPTIONS, ANIME_OPTIONS_RANGE};

use crate::event::Key;
use crate::network::IoEvent;

pub fn handler(key: Key, app: &mut App) {
    match key {
        // k if common::right_event(k) => common::handle_right_event(app),
        k if common::down_event(k) => {
            // calculate the next index in the list
            let next_index = ANIME_OPTIONS_RANGE.start
                + common::on_down_press(
                    &ANIME_OPTIONS,
                    Some(app.library.selected_index % (ANIME_OPTIONS.len())),
                );
            app.library.selected_index = next_index;
        }
        k if common::up_event(k) => {
            // calculate the next index in the list
            let next_index = ANIME_OPTIONS_RANGE.start
                + common::on_up_press(
                    &ANIME_OPTIONS,
                    Some(app.library.selected_index % (ANIME_OPTIONS.len())),
                );
            app.library.selected_index = next_index;
        }

        Key::Enter => {
            match app.library.selected_index {
                // Seasonal
                0 => get_seasonal(app),
                // Ranking
                1 => get_anime_ranking(app),
                // Suggested
                2 => get_suggestion(app),
                // This is required because Rust can't tell if this pattern in exhaustive
                _ => {}
            };
            app.library.selected_index = 9;
        }

        _ => (),
    };
}

fn get_seasonal(app: &mut App) {
    let (is_data_availabe, is_next, index) = is_seasonal_data_available(app);
    let is_current_route = app
        .get_current_route()
        .is_some_and(|r| r.block == ActiveDisplayBlock::Seasonal);

    if is_current_route {
        return;
    }
    app.reset_result_index();

    if is_next {
        app.load_next_route();
        return;
    }

    if is_data_availabe {
        app.load_route(index.unwrap());
    } else {
        app.active_display_block = ActiveDisplayBlock::Loading;

        app.dispatch(IoEvent::GetSeasonalAnime);
    }
}

fn is_seasonal_data_available(app: &mut App) -> (bool, bool, Option<u16>) {
    for i in 0..(app.navigator.history.len() - 1) {
        let id = app.navigator.history[i];
        if app.navigator.data[&id].block == ActiveDisplayBlock::Seasonal
            && app.navigator.data[&id].data.is_some()
        {
            let is_next = app.navigator.index + 1 == i;
            return (true, is_next, Some(id));
        }
    }
    (false, false, None)
}

pub fn get_anime_ranking(app: &mut App) {
    let (is_data_available, is_next, index) = is_anime_ranking_data_available(app);

    let is_current_route = app
        .get_current_route()
        .is_some_and(|r| r.block == ActiveDisplayBlock::AnimeRanking);

    if is_current_route {
        return;
    }

    app.reset_result_index();
    if is_next {
        app.load_next_route();
        return;
    }

    if is_data_available {
        app.load_route(index.unwrap());
    } else {
        app.active_display_block = ActiveDisplayBlock::Loading;

        app.dispatch(IoEvent::GetAnimeRanking(app.anime_ranking_type.clone()));
    }
}

pub fn get_manga_ranking(app: &mut App) {
    let (is_data_available, is_next, index) = is_manga_ranking_data_available(app);

    let is_current_route = app
        .get_current_route()
        .is_some_and(|r| r.block == ActiveDisplayBlock::MangaRanking);
    if is_current_route {
        return;
    }
    app.reset_result_index();

    if is_next {
        app.load_next_route();
        return;
    }

    if is_data_available {
        app.load_route(index.unwrap());
    } else {
        app.active_display_block = ActiveDisplayBlock::Loading;

        app.dispatch(IoEvent::GetMangaRanking(app.manga_ranking_type.clone()));
    }
}

fn is_anime_ranking_data_available(app: &App) -> (bool, bool, Option<u16>) {
    for i in 0..(app.navigator.history.len()) {
        let id = app.navigator.history[i];
        if app.navigator.data[&id].block == ActiveDisplayBlock::AnimeRanking
            && app.navigator.data[&id].data.is_some()
        {
            if let Data::AnimeRanking(_) = app.navigator.data[&id].data.as_ref().unwrap() {
                let is_next = app.navigator.index + 1 == i;
                return (true, is_next, Some(id));
            }
        }
    }
    (false, false, None)
}

fn is_manga_ranking_data_available(app: &App) -> (bool, bool, Option<u16>) {
    for i in 0..(app.navigator.history.len()) {
        let id = app.navigator.history[i];
        if app.navigator.data[&id].block == ActiveDisplayBlock::MangaRanking
            && app.navigator.data[&id].data.is_some()
        {
            if let Data::MangaRanking(_) = app.navigator.data[&id].data.as_ref().unwrap() {
                let is_next = app.navigator.index + 1 == i;
                return (true, is_next, Some(id));
            }
        }
    }
    (false, false, None)
}

fn get_suggestion(app: &mut App) {
    app.reset_result_index();

    let (is_data_available, is_next, index) = is_suggestion_data_available(app);

    let is_current_route = app
        .get_current_route()
        .is_some_and(|r| r.block == ActiveDisplayBlock::Suggestions);

    if is_current_route {
        return;
    }

    app.start_card_list_index = 0;
    app.search_results.selected_display_card_index = Some(0);

    if is_next {
        app.load_next_route();
        return;
    }

    if is_data_available {
        app.load_route(index.unwrap());
    } else {
        app.active_display_block = ActiveDisplayBlock::Loading;

        app.dispatch(IoEvent::GetSuggestedAnime);
    }
}

fn is_suggestion_data_available(app: &App) -> (bool, bool, Option<u16>) {
    for i in 0..(app.navigator.history.len() - 1) {
        let id = app.navigator.history[i];
        if app.navigator.data[&id].block == ActiveDisplayBlock::Suggestions
            && app.navigator.data[&id].data.is_some()
        {
            let is_next = app.navigator.index + 1 == i;
            return (true, is_next, Some(id));
        }
    }
    (false, false, None)
}
