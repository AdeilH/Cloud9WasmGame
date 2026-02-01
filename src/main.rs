mod app;

use bevy::prelude::*;
use bevy::asset::AssetMetaCheck;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "League WASM Game".into(),
                fit_canvas_to_parent: true,
                canvas: Some("#bevy".to_string()),
                ..default()
            }),
            ..default()
        }).set(AssetPlugin {
            meta_check: AssetMetaCheck::Never,
            ..default()
        }))
        .add_plugins(app::GamePlugin)
        .run();
}
