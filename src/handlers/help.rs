// use super::common;
// use crate::app::App;
// use crate::event::Key;

// #[derive(PartialEq)]
// enum Direction {
//     UP,
//     DOWN,
// }

// pub fn handler(key: Key, app: &mut App) {
//     match key {
//         k if common::down_event(k) => {
//             move_page(Direction::DOWN, app);
//         }
//         k if common::up_event(k) => {
//             move_page(Direction::UP, app);
//         }
//         Key::Ctrl('d') => {
//             move_page(Direction::DOWN, app);
//         }
//         Key::Ctrl('u') => {
//             move_page(Direction::UP, app);
//         }
//         _ => {}
//     };
// }

// fn move_page(direction: Direction, app: &mut App) {
//     if direction == Direction::UP {
//         if app.help_menu_page > 0 {
//             app.help_menu_page -= 1;
//         }
//     } else if direction == Direction::DOWN {
//         app.help_menu_page += 1;
//     }
//     app.calculate_help_menu_offset();
// }
