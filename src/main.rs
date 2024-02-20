//! # [Ratatui] Hello World example
//!
//! The latest version of this example is available in the [examples] folder in the repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui-org/ratatui
//! [examples]: https://github.com/ratatui-org/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui-org/ratatui/blob/main/examples/README.md

use bevy::app::AppExit;
use bevy::prelude::*;

use ratatui::{
    prelude::{BevyBackend, RatatuiPlugin, Stylize, Terminal,Color,Style,*},
    widgets::{Block,Paragraph,Borders,Gauge}
};

/// This is a bare minimum example. There are many approaches to running an application loop, so
/// this is not meant to be prescriptive. It is only meant to demonstrate the basic setup and
/// teardown of a terminal application.
///
/// A more robust application would probably want to handle errors and ensure that the terminal is
/// restored to a sane state before exiting. This example does not do that. It also does not handle
/// events or update the application state. It just draws a greeting and exits when the user
/// presses 'q'.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((RatatuiPlugin))
        .add_systems(Startup, camera_setup)
        .add_systems(PreUpdate, terminal_draw)
        .add_systems(Update, (keyboard_input))
        .run();
}


fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let mut my_terminal = Terminal::new(BevyBackend::new(60, 20,40)).unwrap();

    my_terminal.clear();

    commands.spawn(my_terminal);
}

fn terminal_draw(mut terminal_query:  Query<(&mut Terminal<BevyBackend>)>,) {

    let mut rat_term = terminal_query.get_single_mut().expect("More than one terminal with a bevybackend");
    let _ = rat_term.draw(|frame| {
        let area = frame.size();
        frame.render_widget(
            Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                .white()
                .on_blue(),
            area,
        );

        frame.render_widget(
            Gauge::default()
    .block(Block::default().borders(Borders::ALL).title("Progress"))
    .gauge_style(
        Style::default()
            .fg(Color::White)
            .bg(Color::Black)
            .add_modifier(Modifier::ITALIC),
    )
    .percent(20),
            area,
        );


    });
}

fn keyboard_input(keys: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::KeyQ) {
        exit.send(AppExit);
    }
}
