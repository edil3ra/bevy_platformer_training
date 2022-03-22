use bevy::{prelude::*, render::camera::ScalingMode};

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
    

