use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_world);
    }
}

fn create_world(mut commands: Commands) {
    // Create background first
    create_background(&mut commands);
    
    // Create layered ground with more detail
    create_detailed_ground(&mut commands);
    
    // Create improved trees
    create_detailed_trees(&mut commands);
    
    // Create better houses
    create_detailed_houses(&mut commands);
    
    // Create water with effects
    create_detailed_water(&mut commands);
    
    // Add decorative elements
    create_decorations(&mut commands);
}

fn create_background(commands: &mut Commands) {
    // Create a far background layer
    for x in -25..25 {
        for y in -25..25 {
            commands.spawn((
                Sprite {
                    color: Color::srgb(0.6, 0.7, 0.9),
                    custom_size: Some(Vec2::new(64.0, 64.0)),
                    ..default()
                },
                Transform::from_xyz(x as f32 * 64.0, y as f32 * 64.0, -10.0),
            ));
        }
    }
}

fn create_detailed_ground(commands: &mut Commands) {
    for x in -20..20 {
        for y in -20..20 {
            // Create grass base
            let grass_color = match (x + y) % 4 {
                0 => Color::srgb(0.3, 0.5, 0.2),
                1 => Color::srgb(0.35, 0.55, 0.25),
                2 => Color::srgb(0.4, 0.6, 0.3),
                _ => Color::srgb(0.25, 0.45, 0.15),
            };
            
            commands.spawn((
                Sprite {
                    color: grass_color,
                    custom_size: Some(Vec2::new(32.0, 32.0)),
                    ..default()
                },
                Transform::from_xyz(x as f32 * 32.0, y as f32 * 32.0, 0.0),
            ));
            
            // Add grass details randomly
            if (x * y) % 5 == 0 {
                commands.spawn((
                    Sprite {
                        color: Color::srgb(0.2, 0.4, 0.1),
                        custom_size: Some(Vec2::new(8.0, 8.0)),
                        ..default()
                    },
                    Transform::from_xyz(x as f32 * 32.0 + 8.0, y as f32 * 32.0 + 8.0, 0.1),
                ));
            }
        }
    }
}

fn create_detailed_trees(commands: &mut Commands) {
    let tree_positions = [
        (-5.0, 3.0), (8.0, -2.0), (-12.0, 7.0), (15.0, 12.0),
        (-8.0, -5.0), (3.0, 9.0), (-15.0, -8.0), (12.0, -10.0),
        (0.0, 15.0), (-20.0, 0.0), (20.0, 5.0), (5.0, -15.0),
        (-3.0, -12.0), (18.0, -7.0), (-17.0, 14.0), (7.0, 18.0),
    ];
    
    for (x, y) in tree_positions.iter() {
        create_single_tree(commands, x * 32.0, y * 32.0);
    }
}

fn create_single_tree(commands: &mut Commands, x: f32, y: f32) {
    // Tree shadow
    commands.spawn((
        Sprite {
            color: Color::srgba(0.1, 0.2, 0.1, 0.3),
            custom_size: Some(Vec2::new(48.0, 16.0)),
            ..default()
        },
        Transform::from_xyz(x, y - 8.0, -0.5),
    ));
    
    // Tree trunk base
    commands.spawn((
        Sprite {
            color: Color::srgb(0.35, 0.2, 0.1),
            custom_size: Some(Vec2::new(24.0, 40.0)),
            ..default()
        },
        Transform::from_xyz(x, y, 1.0),
    ));
    
    // Tree trunk details
    commands.spawn((
        Sprite {
            color: Color::srgb(0.3, 0.15, 0.05),
            custom_size: Some(Vec2::new(12.0, 32.0)),
            ..default()
        },
        Transform::from_xyz(x, y, 1.1),
    ));
    
    // Tree trunk highlights
    commands.spawn((
        Sprite {
            color: Color::srgb(0.5, 0.3, 0.2),
            custom_size: Some(Vec2::new(4.0, 20.0)),
            ..default()
        },
        Transform::from_xyz(x - 4.0, y, 1.2),
    ));
    
    // Tree roots
    commands.spawn((
        Sprite {
            color: Color::srgb(0.25, 0.15, 0.08),
            custom_size: Some(Vec2::new(32.0, 8.0)),
            ..default()
        },
        Transform::from_xyz(x, y - 16.0, 0.9),
    ));
    
    // Tree leaves (main canopy)
    for i in 0..4 {
        let leaf_size = 80.0 - i as f32 * 12.0;
        let leaf_y = y + 16.0 + i as f32 * 6.0;
        let leaf_color = match i {
            0 => Color::srgb(0.12, 0.35, 0.12),
            1 => Color::srgb(0.18, 0.45, 0.18),
            2 => Color::srgb(0.22, 0.55, 0.22),
            _ => Color::srgb(0.28, 0.65, 0.28),
        };
        
        commands.spawn((
            Sprite {
                color: leaf_color,
                custom_size: Some(Vec2::new(leaf_size, leaf_size)),
                ..default()
            },
            Transform::from_xyz(x, leaf_y, 2.0 + i as f32 * 0.1),
        ));
    }
    
    // Tree leaf clusters
    let leaf_clusters = [
        (-20.0, 24.0), (20.0, 24.0), (-16.0, 40.0), (16.0, 40.0),
        (-24.0, 32.0), (24.0, 32.0), (0.0, 48.0),
    ];
    
    for (lx, ly) in leaf_clusters.iter() {
        commands.spawn((
            Sprite {
                color: Color::srgb(0.35, 0.7, 0.35),
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            Transform::from_xyz(x + lx, y + ly, 2.5),
        ));
    }
    
    // Tree highlights
    commands.spawn((
        Sprite {
            color: Color::srgb(0.4, 0.8, 0.4),
            custom_size: Some(Vec2::new(12.0, 12.0)),
            ..default()
        },
        Transform::from_xyz(x - 16.0, y + 32.0, 2.6),
    ));
    
    commands.spawn((
        Sprite {
            color: Color::srgb(0.4, 0.8, 0.4),
            custom_size: Some(Vec2::new(8.0, 8.0)),
            ..default()
        },
        Transform::from_xyz(x + 12.0, y + 28.0, 2.6),
    ));
    
    // Tree bark texture
    for i in 0..3 {
        commands.spawn((
            Sprite {
                color: Color::srgb(0.2, 0.1, 0.05),
                custom_size: Some(Vec2::new(2.0, 8.0)),
                ..default()
            },
            Transform::from_xyz(x - 2.0 + i as f32 * 4.0, y - 4.0, 1.3),
        ));
    }
}

fn create_detailed_houses(commands: &mut Commands) {
    let house_positions = [(-10.0, -10.0), (10.0, 8.0), (-5.0, 15.0)];
    
    for (i, (x, y)) in house_positions.iter().enumerate() {
        create_single_house(commands, x * 32.0, y * 32.0, i);
    }
}

fn create_single_house(commands: &mut Commands, x: f32, y: f32, variant: usize) {
    // House shadow
    commands.spawn((
        Sprite {
            color: Color::srgba(0.1, 0.1, 0.1, 0.3),
            custom_size: Some(Vec2::new(120.0, 80.0)),
            ..default()
        },
        Transform::from_xyz(x, y - 20.0, -0.5),
    ));
    
    // House foundation
    commands.spawn((
        Sprite {
            color: Color::srgb(0.4, 0.3, 0.2),
            custom_size: Some(Vec2::new(100.0, 12.0)),
            ..default()
        },
        Transform::from_xyz(x, y - 36.0, 0.9),
    ));
    
    // House base
    let house_colors = [
        Color::srgb(0.8, 0.6, 0.4),
        Color::srgb(0.7, 0.5, 0.3),
        Color::srgb(0.75, 0.55, 0.35),
    ];
    
    commands.spawn((
        Sprite {
            color: house_colors[variant % house_colors.len()],
            custom_size: Some(Vec2::new(96.0, 80.0)),
            ..default()
        },
        Transform::from_xyz(x, y, 1.0),
    ));
    
    // House wall details
    commands.spawn((
        Sprite {
            color: Color::srgb(0.6, 0.4, 0.2),
            custom_size: Some(Vec2::new(88.0, 72.0)),
            ..default()
        },
        Transform::from_xyz(x, y, 1.1),
    ));
    
    // House trim
    commands.spawn((
        Sprite {
            color: Color::srgb(0.9, 0.7, 0.5),
            custom_size: Some(Vec2::new(100.0, 4.0)),
            ..default()
        },
        Transform::from_xyz(x, y + 40.0, 1.2),
    ));
    
    // Roof main
    commands.spawn((
        Sprite {
            color: Color::srgb(0.6, 0.2, 0.2),
            custom_size: Some(Vec2::new(112.0, 40.0)),
            ..default()
        },
        Transform::from_xyz(x, y + 52.0, 2.0),
    ));
    
    // Roof details
    commands.spawn((
        Sprite {
            color: Color::srgb(0.5, 0.15, 0.15),
            custom_size: Some(Vec2::new(104.0, 32.0)),
            ..default()
        },
        Transform::from_xyz(x, y + 52.0, 2.1),
    ));
    
    // Roof shingles
    for i in 0..5 {
        commands.spawn((
            Sprite {
                color: Color::srgb(0.4, 0.1, 0.1),
                custom_size: Some(Vec2::new(16.0, 4.0)),
                ..default()
            },
            Transform::from_xyz(x - 40.0 + i as f32 * 20.0, y + 60.0, 2.2),
        ));
    }
    
    // Chimney
    commands.spawn((
        Sprite {
            color: Color::srgb(0.7, 0.3, 0.3),
            custom_size: Some(Vec2::new(16.0, 24.0)),
            ..default()
        },
        Transform::from_xyz(x + 20.0, y + 68.0, 2.3),
    ));
    
    // Chimney top
    commands.spawn((
        Sprite {
            color: Color::srgb(0.5, 0.2, 0.2),
            custom_size: Some(Vec2::new(20.0, 8.0)),
            ..default()
        },
        Transform::from_xyz(x + 20.0, y + 80.0, 2.4),
    ));
    
    // Door
    commands.spawn((
        Sprite {
            color: Color::srgb(0.3, 0.2, 0.1),
            custom_size: Some(Vec2::new(24.0, 40.0)),
            ..default()
        },
        Transform::from_xyz(x, y - 16.0, 3.0),
    ));
    
    // Door details
    commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.15, 0.08),
            custom_size: Some(Vec2::new(20.0, 36.0)),
            ..default()
        },
        Transform::from_xyz(x, y - 16.0, 3.1),
    ));
    
    // Door handle
    commands.spawn((
        Sprite {
            color: Color::srgb(0.8, 0.8, 0.6),
            custom_size: Some(Vec2::new(3.0, 3.0)),
            ..default()
        },
        Transform::from_xyz(x + 8.0, y - 16.0, 3.2),
    ));
    
    // Windows
    let window_positions = [(-20.0, 8.0), (20.0, 8.0)];
    for (wx, wy) in window_positions.iter() {
        // Window frame
        commands.spawn((
            Sprite {
                color: Color::srgb(0.9, 0.9, 0.7),
                custom_size: Some(Vec2::new(16.0, 16.0)),
                ..default()
            },
            Transform::from_xyz(x + wx, y + wy, 3.0),
        ));
        
        // Window glass
        commands.spawn((
            Sprite {
                color: Color::srgb(0.7, 0.9, 1.0),
                custom_size: Some(Vec2::new(12.0, 12.0)),
                ..default()
            },
            Transform::from_xyz(x + wx, y + wy, 3.1),
        ));
        
        // Window cross
        commands.spawn((
            Sprite {
                color: Color::srgb(0.3, 0.2, 0.1),
                custom_size: Some(Vec2::new(2.0, 16.0)),
                ..default()
            },
            Transform::from_xyz(x + wx, y + wy, 3.2),
        ));
        
        commands.spawn((
            Sprite {
                color: Color::srgb(0.3, 0.2, 0.1),
                custom_size: Some(Vec2::new(16.0, 2.0)),
                ..default()
            },
            Transform::from_xyz(x + wx, y + wy, 3.2),
        ));
        
        // Window sill
        commands.spawn((
            Sprite {
                color: Color::srgb(0.7, 0.5, 0.3),
                custom_size: Some(Vec2::new(20.0, 4.0)),
                ..default()
            },
            Transform::from_xyz(x + wx, y + wy - 10.0, 3.1),
        ));
    }
    
    // Flower boxes under windows
    for (wx, wy) in window_positions.iter() {
        commands.spawn((
            Sprite {
                color: Color::srgb(0.6, 0.4, 0.2),
                custom_size: Some(Vec2::new(18.0, 6.0)),
                ..default()
            },
            Transform::from_xyz(x + wx, y + wy - 14.0, 3.0),
        ));
        
        // Flowers in boxes
        for i in 0..3 {
            commands.spawn((
                Sprite {
                    color: Color::srgb(1.0, 0.8, 0.8),
                    custom_size: Some(Vec2::new(4.0, 4.0)),
                    ..default()
                },
                Transform::from_xyz(x + wx - 4.0 + i as f32 * 4.0, y + wy - 12.0, 3.1),
            ));
        }
    }
}

fn create_detailed_water(commands: &mut Commands) {
    let water_tiles = [
        (-15.0, 15.0), (-14.0, 15.0), (-13.0, 15.0), (-12.0, 15.0),
        (-15.0, 14.0), (-14.0, 14.0), (-13.0, 14.0), (-12.0, 14.0),
        (-15.0, 13.0), (-14.0, 13.0), (-13.0, 13.0), (-12.0, 13.0),
        (-15.0, 12.0), (-14.0, 12.0),
    ];
    
    for (i, (x, y)) in water_tiles.iter().enumerate() {
        let world_x = x * 32.0;
        let world_y = y * 32.0;
        
        // Water base
        let water_color = match i % 3 {
            0 => Color::srgb(0.3, 0.5, 0.8),
            1 => Color::srgb(0.35, 0.55, 0.85),
            _ => Color::srgb(0.25, 0.45, 0.75),
        };
        
        commands.spawn((
            Sprite {
                color: water_color,
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            Transform::from_xyz(world_x, world_y, 0.5),
        ));
        
        // Water waves
        for j in 0..3 {
            commands.spawn((
                Sprite {
                    color: Color::srgba(0.4, 0.6, 0.9, 0.5),
                    custom_size: Some(Vec2::new(24.0, 2.0)),
                    ..default()
                },
                Transform::from_xyz(world_x - 4.0 + j as f32 * 8.0, world_y - 8.0 + j as f32 * 8.0, 0.51),
            ));
        }
        
        // Water sparkles
        if i % 3 == 0 {
            commands.spawn((
                Sprite {
                    color: Color::srgb(0.9, 0.95, 1.0),
                    custom_size: Some(Vec2::new(6.0, 6.0)),
                    ..default()
                },
                Transform::from_xyz(world_x + 6.0, world_y + 6.0, 0.6),
            ));
        }
        
        // Water depth effect
        commands.spawn((
            Sprite {
                color: Color::srgba(0.2, 0.3, 0.5, 0.3),
                custom_size: Some(Vec2::new(28.0, 28.0)),
                ..default()
            },
            Transform::from_xyz(world_x, world_y, 0.52),
        ));
    }
    
    // Water edge details
    let water_edges = [
        (-11.0, 15.0), (-11.0, 14.0), (-11.0, 13.0), (-11.0, 12.0),
        (-15.0, 11.0), (-14.0, 11.0), (-13.0, 11.0), (-12.0, 11.0),
    ];
    
    for (x, y) in water_edges.iter() {
        let world_x = x * 32.0;
        let world_y = y * 32.0;
        
        // Shore/sand
        commands.spawn((
            Sprite {
                color: Color::srgb(0.8, 0.7, 0.5),
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            Transform::from_xyz(world_x, world_y, 0.4),
        ));
        
        // Sand details
        commands.spawn((
            Sprite {
                color: Color::srgb(0.9, 0.8, 0.6),
                custom_size: Some(Vec2::new(8.0, 8.0)),
                ..default()
            },
            Transform::from_xyz(world_x + 8.0, world_y + 8.0, 0.41),
        ));
    }
}

fn create_decorations(commands: &mut Commands) {
    // Create flowers
    let flower_positions = [
        (-7.0, 2.0), (12.0, -5.0), (-3.0, 12.0), (18.0, 3.0),
        (-14.0, -7.0), (6.0, -13.0), (-9.0, 18.0), (22.0, -2.0),
    ];
    
    for (x, y) in flower_positions.iter() {
        let world_x = x * 32.0;
        let world_y = y * 32.0;
        
        // Flower leaves
        commands.spawn((
            Sprite {
                color: Color::srgb(0.15, 0.3, 0.08),
                custom_size: Some(Vec2::new(4.0, 8.0)),
                ..default()
            },
            Transform::from_xyz(world_x - 2.0, world_y - 2.0, 0.15),
        ));
        
        commands.spawn((
            Sprite {
                color: Color::srgb(0.15, 0.3, 0.08),
                custom_size: Some(Vec2::new(4.0, 8.0)),
                ..default()
            },
            Transform::from_xyz(world_x + 2.0, world_y - 2.0, 0.15),
        ));
        
        // Flower stem
        commands.spawn((
            Sprite {
                color: Color::srgb(0.2, 0.4, 0.1),
                custom_size: Some(Vec2::new(3.0, 16.0)),
                ..default()
            },
            Transform::from_xyz(world_x, world_y, 0.2),
        ));
        
        // Flower center
        commands.spawn((
            Sprite {
                color: Color::srgb(1.0, 0.8, 0.2),
                custom_size: Some(Vec2::new(6.0, 6.0)),
                ..default()
            },
            Transform::from_xyz(world_x, world_y + 8.0, 0.25),
        ));
        
        // Flower petals
        let flower_colors = [
            Color::srgb(1.0, 0.8, 0.8),
            Color::srgb(1.0, 1.0, 0.8),
            Color::srgb(0.8, 0.8, 1.0),
            Color::srgb(1.0, 0.9, 0.6),
        ];
        
        let petal_color = flower_colors[(*x as usize + *y as usize) % flower_colors.len()];
        
        // Create 4 petals around the center
        let petal_positions = [
            (-4.0, 8.0), (4.0, 8.0), (0.0, 4.0), (0.0, 12.0),
        ];
        
        for (px, py) in petal_positions.iter() {
            commands.spawn((
                Sprite {
                    color: petal_color,
                    custom_size: Some(Vec2::new(5.0, 5.0)),
                    ..default()
                },
                Transform::from_xyz(world_x + px, world_y + py, 0.3),
            ));
        }
    }
    
    // Create rocks
    let rock_positions = [(-18.0, -3.0), (14.0, 11.0), (-6.0, -16.0)];
    
    for (x, y) in rock_positions.iter() {
        let world_x = x * 32.0;
        let world_y = y * 32.0;
        
        // Rock shadow
        commands.spawn((
            Sprite {
                color: Color::srgba(0.1, 0.1, 0.1, 0.3),
                custom_size: Some(Vec2::new(18.0, 10.0)),
                ..default()
            },
            Transform::from_xyz(world_x, world_y - 4.0, 0.05),
        ));
        
        // Rock base
        commands.spawn((
            Sprite {
                color: Color::srgb(0.5, 0.5, 0.5),
                custom_size: Some(Vec2::new(16.0, 12.0)),
                ..default()
            },
            Transform::from_xyz(world_x, world_y, 0.1),
        ));
        
        // Rock details
        commands.spawn((
            Sprite {
                color: Color::srgb(0.6, 0.6, 0.6),
                custom_size: Some(Vec2::new(8.0, 6.0)),
                ..default()
            },
            Transform::from_xyz(world_x + 2.0, world_y + 2.0, 0.2),
        ));
        
        // Rock highlights
        commands.spawn((
            Sprite {
                color: Color::srgb(0.8, 0.8, 0.8),
                custom_size: Some(Vec2::new(4.0, 4.0)),
                ..default()
            },
            Transform::from_xyz(world_x - 2.0, world_y - 2.0, 0.3),
        ));
        
        // Rock texture
        commands.spawn((
            Sprite {
                color: Color::srgb(0.4, 0.4, 0.4),
                custom_size: Some(Vec2::new(2.0, 3.0)),
                ..default()
            },
            Transform::from_xyz(world_x + 4.0, world_y, 0.25),
        ));
    }
    
    // Create bushes
    let bush_positions = [(-2.0, 8.0), (15.0, -8.0), (-12.0, 3.0)];
    
    for (x, y) in bush_positions.iter() {
        let world_x = x * 32.0;
        let world_y = y * 32.0;
        
        // Bush shadow
        commands.spawn((
            Sprite {
                color: Color::srgba(0.1, 0.2, 0.1, 0.3),
                custom_size: Some(Vec2::new(28.0, 12.0)),
                ..default()
            },
            Transform::from_xyz(world_x, world_y - 4.0, 0.05),
        ));
        
        // Bush base
        commands.spawn((
            Sprite {
                color: Color::srgb(0.2, 0.4, 0.15),
                custom_size: Some(Vec2::new(24.0, 16.0)),
                ..default()
            },
            Transform::from_xyz(world_x, world_y, 0.1),
        ));
        
        // Bush details
        for i in 0..3 {
            commands.spawn((
                Sprite {
                    color: Color::srgb(0.25, 0.45, 0.2),
                    custom_size: Some(Vec2::new(8.0, 8.0)),
                    ..default()
                },
                Transform::from_xyz(world_x - 6.0 + i as f32 * 6.0, world_y + 2.0, 0.15),
            ));
        }
        
        // Bush highlights
        commands.spawn((
            Sprite {
                color: Color::srgb(0.35, 0.65, 0.3),
                custom_size: Some(Vec2::new(6.0, 6.0)),
                ..default()
            },
            Transform::from_xyz(world_x - 4.0, world_y + 4.0, 0.2),
        ));
    }
}