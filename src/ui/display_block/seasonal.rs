use super::center_area;
use super::{draw_keys_bar, results};
use crate::app::{App, SEASONS};
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Clear, List, ListState, Padding, Paragraph};
use ratatui::{
    layout::Rect,
    widgets::{Block, BorderType, Borders},
    Frame,
};

pub fn draw_seasonal_anime(f: &mut Frame, app: &App, chunk: Rect) {
    let chunk = draw_keys_bar(f, app, chunk);
    results::draw_results(f, app, chunk);
    if app.popup {
        let area = center_area(chunk, 30, 30);

        // draw popup:

        let popup = Block::default()
            .title("Select Season")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Double);

        f.render_widget(Clear, area);
        f.render_widget(popup, area);

        let is_popup_season_block = app.anime_season.popup_season_highlight;

        let [season_chunk, year_chunk] = Layout::default()
            .constraints(vec![Constraint::Percentage(50); 2])
            .margin(2)
            .direction(Direction::Horizontal)
            .areas(area);

        // ===> season
        let mut season_block = Block::default()
            .title_alignment(Alignment::Center)
            .borders(Borders::NONE)
            .padding(Padding::symmetric(1, 1));

        if is_popup_season_block {
            season_block = season_block
                .title_style(
                    Style::default()
                        .add_modifier(Modifier::UNDERLINED)
                        .add_modifier(Modifier::BOLD),
                )
                .title("Season")
        } else {
            season_block = season_block.title("Season")
        }

        let list: Vec<Line> = SEASONS
            .iter()
            .map(|s| {
                Line::from(*s)
                    .alignment(Alignment::Center)
                    .style(Style::default().fg(app.app_config.theme.text))
            })
            .collect();
        let season_selected: Option<usize> = Some(app.anime_season.selected_season.into());

        let mut state = ListState::default();
        state.select(season_selected);

        let season_list = List::new(list).block(season_block).highlight_style(
            Style::default()
                .fg(app.app_config.theme.selected)
                .add_modifier(Modifier::BOLD), // .add_modifier(Modifier::UNDERLINED),
        );
        // .highlight_symbol("> ");
        // .scroll_padding(2);

        let centered_season_chunk = center_area(season_chunk, 60, 90);
        f.render_stateful_widget(season_list, centered_season_chunk, &mut state);

        // ===> year

        let mut year_block = Block::default()
            .title_alignment(Alignment::Center)
            .borders(Borders::NONE)
            .padding(Padding::symmetric(1, 2));

        if !is_popup_season_block {
            year_block = year_block
                .title_style(
                    Style::default()
                        .add_modifier(Modifier::UNDERLINED)
                        .add_modifier(Modifier::BOLD),
                )
                .title("Year")
        } else {
            year_block = year_block.title("Year")
        }

        let year_paragraph = Paragraph::new(app.anime_season.selected_year.to_string())
            .block(year_block)
            .alignment(Alignment::Center);

        let centered_year_chunk = center_area(year_chunk, 60, 90);
        f.render_widget(year_paragraph, centered_year_chunk);
    }
}
