use bevy::prelude::*;
use crate::components::*;

pub struct PlayerSystems;

impl PlayerSystems {
    pub fn spawn_player(mut commands: Commands) {
        let x = 0.0;
        let y = 0.0;
        
        // Player shadow
        commands.spawn((
            Sprite {
                color: Color::srgba(0.1, 0.1, 0.1, 0.3),
                custom_size: Some(Vec2::new(20.0, 8.0)),
                ..default()
            },
            Transform::from_xyz(x, y - 12.0, -0.5),
            PlayerPart,
        ));
        
        // Player legs (for walking animation)
        commands.spawn((
            Sprite {
                color: Color::srgb(0.2, 0.2, 0.6),
                custom_size: Some(Vec2::new(6.0, 12.0)),
                ..default()
            },
            Transform::from_xyz(x - 3.0, y - 8.0, 0.9),
            PlayerPart,
        ));
        
        commands.spawn((
            Sprite {
                color: Color::srgb(0.2, 0.2, 0.6),
                custom_size: Some(Vec2::new(6.0, 12.0)),
                ..default()
            },
            Transform::from_xyz(x + 3.0, y - 8.0, 0.9),
            PlayerPart,
        ));
        
        // Player feet
        commands.spawn((
            Sprite {
                color: Color::srgb(0.1, 0.1, 0.3),
                custom_size: Some(Vec2::new(8.0, 4.0)),
                ..default()
            },
            Transform::from_xyz(x - 3.0, y - 14.0, 0.8),
            PlayerPart,
        ));
        
        commands.spawn((
            Sprite {
                color: Color::srgb(0.1, 0.1, 0.3),
                custom_size: Some(Vec2::new(8.0, 4.0)),
                ..default()
            },
            Transform::from_xyz(x + 3.0, y - 14.0, 0.8),
            PlayerPart,
        ));
        
        // Player body (main color)
        commands.spawn((
            Sprite {
                color: Color::srgb(0.4, 0.4, 0.9),
                custom_size: Some(Vec2::new(16.0, 20.0)),
                ..default()
            },
            Transform::from_xyz(x, y, 1.0),
            PlayerPart,
        ));
        
        // Player arms
        commands.spawn((
            Sprite {
                color: Color::srgb(0.3, 0.3, 0.8),
                custom_size: Some(Vec2::new(5.0, 14.0)),
                ..default()
            },
            Transform::from_xyz(x - 8.0, y - 2.0, 0.95),
            PlayerPart,
        ));
        
        commands.spawn((
            Sprite {
                color: Color::srgb(0.3, 0.3, 0.8),
                custom_size: Some(Vec2::new(5.0, 14.0)),
                ..default()
            },
            Transform::from_xyz(x + 8.0, y - 2.0, 0.95),
            PlayerPart,
        ));
        
        // Player hands
        commands.spawn((
            Sprite {
                color: Color::srgb(0.9, 0.8, 0.7),
                custom_size: Some(Vec2::new(6.0, 6.0)),
                ..default()
            },
            Transform::from_xyz(x - 8.0, y + 6.0, 1.05),
            PlayerPart,
        ));
        
        commands.spawn((
            Sprite {
                color: Color::srgb(0.9, 0.8, 0.7),
                custom_size: Some(Vec2::new(6.0, 6.0)),
                ..default()
            },
            Transform::from_xyz(x + 8.0, y + 6.0, 1.05),
            PlayerPart,
        ));
        
        // Player head
        commands.spawn((
            Sprite {
                color: Color::srgb(0.9, 0.8, 0.7),
                custom_size: Some(Vec2::new(16.0, 16.0)),
                ..default()
            },
            Transform::from_xyz(x, y + 14.0, 1.1),
            PlayerPart,
        ));
        
        // Player hair
        commands.spawn((
            Sprite {
                color: Color::srgb(0.3, 0.2, 0.1),
                custom_size: Some(Vec2::new(14.0, 10.0)),
                ..default()
            },
            Transform::from_xyz(x, y + 20.0, 1.2),
            PlayerPart,
        ));
        
        // Player eyes
        commands.spawn((
            Sprite {
                color: Color::srgb(0.1, 0.1, 0.1),
                custom_size: Some(Vec2::new(2.0, 2.0)),
                ..default()
            },
            Transform::from_xyz(x - 3.0, y + 15.0, 1.3),
            PlayerPart,
        ));
        
        commands.spawn((
            Sprite {
                color: Color::srgb(0.1, 0.1, 0.1),
                custom_size: Some(Vec2::new(2.0, 2.0)),
                ..default()
            },
            Transform::from_xyz(x + 3.0, y + 15.0, 1.3),
            PlayerPart,
        ));
        
        // Player mouth
        commands.spawn((
            Sprite {
                color: Color::srgb(0.3, 0.2, 0.2),
                custom_size: Some(Vec2::new(4.0, 2.0)),
                ..default()
            },
            Transform::from_xyz(x, y + 11.0, 1.3),
            PlayerPart,
        ));
        
        // Player main component (for movement logic)
        commands.spawn((
            Sprite {
                color: Color::srgba(0.0, 0.0, 0.0, 0.0), // Invisible
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..default()
            },
            Transform::from_xyz(x, y, 1.0),
            Player {
                speed: 100.0,
                run_speed: 200.0,
            },
            PlayerDirection {
                direction: Vec2::ZERO,
            },
            PlayerAnimation {
                frame_timer: Timer::from_seconds(0.2, TimerMode::Repeating),
                current_frame: 0,
                is_walking: false,
                facing_left: false,
            },
        ));
    }

    pub fn player_movement(
        keyboard: Res<ButtonInput<KeyCode>>,
        time: Res<Time>,
        mut player_query: Query<(&Player, &mut Transform, &mut PlayerDirection, &mut PlayerAnimation), Without<PlayerPart>>,
        mut parts_query: Query<&mut Transform, With<PlayerPart>>,
    ) {
        for (player, mut transform, mut direction, mut animation) in player_query.iter_mut() {
            let mut movement = Vec2::ZERO;
            
            if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
                movement.y += 1.0;
            }
            if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
                movement.y -= 1.0;
            }
            if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
                movement.x -= 1.0;
            }
            if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
                movement.x += 1.0;
            }
            
            // Update walking state and facing direction
            animation.is_walking = movement != Vec2::ZERO;
            if movement.x < 0.0 {
                animation.facing_left = true;
            } else if movement.x > 0.0 {
                animation.facing_left = false;
            }
            
            if movement != Vec2::ZERO {
                movement = movement.normalize();
                
                let current_speed = if keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight) {
                    player.run_speed
                } else {
                    player.speed
                };
                
                let movement_vector = movement * current_speed * time.delta_secs();
                let old_pos = transform.translation;
                transform.translation.x += movement_vector.x;
                transform.translation.y += movement_vector.y;
                
                // Move all player parts
                let delta = transform.translation - old_pos;
                for mut part_transform in parts_query.iter_mut() {
                    part_transform.translation.x += delta.x;
                    part_transform.translation.y += delta.y;
                }
                
                direction.direction = movement;
            } else {
                direction.direction = Vec2::ZERO;
            }
        }
    }

    pub fn animate_player(
        time: Res<Time>,
        mut player_query: Query<(&mut PlayerAnimation, &Transform), Without<PlayerPart>>,
        mut parts_query: Query<&mut Transform, With<PlayerPart>>,
    ) {
        for (mut animation, player_transform) in player_query.iter_mut() {
            // Get the player parts for animation
            let mut legs = Vec::new();
            let mut feet = Vec::new();
            let mut arms = Vec::new();
            
            for part_transform in parts_query.iter_mut() {
                let rel_x = part_transform.translation.x - player_transform.translation.x;
                let rel_y = part_transform.translation.y - player_transform.translation.y;
                
                // Identify which part this is based on relative position
                if (rel_y - (-8.0)).abs() < 1.0 { // Legs at y = -8
                    if (rel_x - (-3.0)).abs() < 1.0 { // Left leg
                        legs.push((part_transform, -3.0));
                    } else if (rel_x - 3.0).abs() < 1.0 { // Right leg
                        legs.push((part_transform, 3.0));
                    }
                } else if (rel_y - (-14.0)).abs() < 1.0 { // Feet at y = -14
                    if (rel_x - (-3.0)).abs() < 1.0 { // Left foot
                        feet.push((part_transform, -3.0));
                    } else if (rel_x - 3.0).abs() < 1.0 { // Right foot
                        feet.push((part_transform, 3.0));
                    }
                } else if (rel_y - (-2.0)).abs() < 1.0 { // Arms at y = -2
                    if (rel_x - (-8.0)).abs() < 1.0 { // Left arm
                        arms.push((part_transform, -8.0));
                    } else if (rel_x - 8.0).abs() < 1.0 { // Right arm
                        arms.push((part_transform, 8.0));
                    }
                }
            }
            
            // Animate based on walking state
            if animation.is_walking {
                // Update animation frame
                animation.frame_timer.tick(time.delta());
                if animation.frame_timer.just_finished() {
                    animation.current_frame = (animation.current_frame + 1) % 4;
                }
                
                // Walking animation offsets
                let frame = animation.current_frame;
                let left_leg_offset = match frame {
                    0 => 0.0,
                    1 => 2.0,
                    2 => 0.0,
                    _ => -2.0,
                };
                let right_leg_offset = match frame {
                    0 => 0.0,
                    1 => -2.0,
                    2 => 0.0,
                    _ => 2.0,
                };
                
                // Animate legs
                for (leg_transform, base_x) in legs.iter_mut() {
                    let offset = if *base_x < 0.0 { left_leg_offset } else { right_leg_offset };
                    leg_transform.translation.x = player_transform.translation.x + *base_x + offset;
                }
                
                // Animate feet (following legs)
                for (foot_transform, base_x) in feet.iter_mut() {
                    let offset = if *base_x < 0.0 { left_leg_offset } else { right_leg_offset };
                    foot_transform.translation.x = player_transform.translation.x + *base_x + offset;
                }
                
                // Animate arms (swinging opposite to legs)
                let arm_swing = match frame {
                    0 => 0.0,
                    1 => -1.5,
                    2 => 0.0,
                    _ => 1.5,
                };
                
                for (arm_transform, base_x) in arms.iter_mut() {
                    let offset = if (*base_x < 0.0) != animation.facing_left {
                        arm_swing
                    } else {
                        -arm_swing
                    };
                    arm_transform.translation.x = player_transform.translation.x + *base_x + offset;
                }
                
            } else {
                // Reset to idle position
                animation.current_frame = 0;
                
                // Reset all parts to base positions
                for (leg_transform, base_x) in legs.iter_mut() {
                    leg_transform.translation.x = player_transform.translation.x + *base_x;
                }
                
                for (foot_transform, base_x) in feet.iter_mut() {
                    foot_transform.translation.x = player_transform.translation.x + *base_x;
                }
                
                for (arm_transform, base_x) in arms.iter_mut() {
                    arm_transform.translation.x = player_transform.translation.x + *base_x;
                }
            }
        }
    }
}