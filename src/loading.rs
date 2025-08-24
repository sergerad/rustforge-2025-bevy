use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{GameState, MainCamera};

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "bevy_bird_dark.png")]
    pub bevy: Handle<Image>,

    #[asset(path = "alienPink_square.png")]
    pub piggy: Handle<Image>,

    #[asset(path = "elementWood022.png")]
    pub kiste: Handle<Image>,

    #[asset(path = "elementWood008.png")]
    pub roof: Handle<Image>,

    #[asset(path = "background/colored_grass.png")]
    pub background: Handle<Image>,
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
    commands.spawn((
        Camera2d,
        MainCamera,
        Msaa::Off,
        Transform::from_scale(Vec3::splat(1.6)),
    ));
}
