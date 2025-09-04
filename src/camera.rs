use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
           .add_systems(Update, camera_follow);
    }
}

#[derive(Component)]
pub struct MainCamera;

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        MainCamera,
        Projection::Orthographic(OrthographicProjection {
            far: 1000.0,
            near: -1000.0,
            scaling_mode: bevy::render::camera::ScalingMode::FixedVertical { viewport_height: 800.0 },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

fn camera_follow(
    player_query: Query<&Transform, With<crate::components::Player>>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<crate::components::Player>)>,
) {
    if let Ok(player_transform) = player_query.single() {
        if let Ok(mut camera_transform) = camera_query.single_mut() {
            let target_position = Vec3::new(
                player_transform.translation.x,
                player_transform.translation.y,
                camera_transform.translation.z,
            );
            
            camera_transform.translation = camera_transform.translation.lerp(target_position, 0.1);
        }
    }
}