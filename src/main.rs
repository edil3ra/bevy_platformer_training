use bevy::{prelude::*, render::camera::ScalingMode, sprite::Rect, sprite::TextureAtlas};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
struct TextureAtlasXML {
    width: f32,
    height: f32,
    #[serde(rename = "$value")]
    subs: Vec<Subs>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Subs {
    name: String,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

pub struct PlayersSheet(pub Handle<TextureAtlas>);

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

fn main() {
    let height: f32 = 1920.0;
    let width: f32 = 1080.0;
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width,
            height,
            title: "Bevy Tutorial".to_string(),
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_startup_system(load_assets)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    //Set the camera to have normalized coordinates of y values -1 to 1
    // camera.orthographic_projection.top = 1.0;
    // camera.orthographic_projection.bottom = -1.0;

    // camera.orthographic_projection.right = 1.0 * RESOLUTION;
    // camera.orthographic_projection.left = -1.0 * RESOLUTION;

    //Force the camera to use our settings
    camera.orthographic_projection.scaling_mode = ScalingMode::WindowSize;

    commands.spawn_bundle(camera);
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let players_image_handle: Handle<Image> = asset_server.load("spritesheet_players.png");
    let players_xml: TextureAtlasXML = {
        let players_string = fs::read_to_string("assets/spritesheet_players.xml")
            .expect("could not read spritesheet_players.png");
        serde_xml_rs::from_str(&players_string).unwrap()
    };

    let mut players_spritesheet = TextureAtlas::new_empty(
        players_image_handle,
        Vec2::new(players_xml.width, players_xml.height),
    );

    for sub in players_xml.subs.iter() {
        players_spritesheet.add_texture(Rect {
            min: Vec2::new(sub.x, sub.y),
            max: Vec2::new(sub.width, sub.height),
        });
    }

    println!("{:?}", players_spritesheet);
    let player_handle = texture_atlases.add(players_spritesheet);
    commands.insert_resource(PlayersSheet(player_handle))
}
