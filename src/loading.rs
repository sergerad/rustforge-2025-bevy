use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{GameState, MainCamera};

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "bevy_bird_dark.png")]
    pub bevy: Handle<Image>,
}

pub struct LoadingPlugin;
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), on_enter)
            .add_loading_state(
                LoadingState::new(GameState::Loading)
                    .continue_to_state(GameState::SplashScreen)
                    .load_collection::<GameAssets>(),
            );
    }
}

fn on_enter(mut commands: Commands) {
    info!("loading");
    commands.spawn((Camera2d, MainCamera, Msaa::Off));
}
