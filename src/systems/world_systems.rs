use bevy::prelude::*;
use rand::Rng;

pub struct WorldSystems;

impl WorldSystems {
    pub fn spawn_world(mut commands: Commands) {
        // Spawn terrain
        Self::spawn_terrain(&mut commands);
        
        // Spawn trees
        Self::spawn_trees(&mut commands);
        
        // Spawn houses
        Self::spawn_houses(&mut commands);
        
        // Spawn water features
        Self::spawn_water(&mut commands);
        
        // Spawn decorative elements
        Self::spawn_decorations(&mut commands);
    }

    fn spawn_terrain(commands: &mut Commands) {
        // Ground - 10x larger
        commands.spawn((
            Sprite {
                color: Color::srgb(0.4, 0.6, 0.3),
                custom_size: Some(Vec2::new(8000.0, 8000.0)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, -10.0),
        ));

        // Dense grass coverage with variety
        Self::spawn_dense_grass(commands);
        
        // Various flower patches
        Self::spawn_flower_fields(commands);
        
        // Small bushes and shrubs
        Self::spawn_shrubs(commands);
        
        // Mushroom patches
        Self::spawn_mushrooms(commands);
    }

    fn spawn_dense_grass(commands: &mut Commands) {
        let mut rng = rand::rng();
        
        // Generate dense grass coverage across the entire map
        for x in (-3800..=3800).step_by(80) {
            for y in (-3800..=3800).step_by(80) {
                // Add some randomness to positions
                let offset_x = rng.random_range(-30.0..30.0);
                let offset_y = rng.random_range(-30.0..30.0);
                let pos_x = x as f32 + offset_x;
                let pos_y = y as f32 + offset_y;
                
                // Different grass types and colors
                let grass_type = rng.random_range(0_u32..5);
                
                match grass_type {
                    0 => {
                        // Tall dark green grass
                        commands.spawn((
                            Sprite {
                                color: Color::srgb(0.3, 0.6, 0.2),
                                custom_size: Some(Vec2::new(8.0, 25.0)),
                                ..default()
                            },
                            Transform::from_xyz(pos_x, pos_y, -9.8),
                        ));
                    }
                    1 => {
                        // Medium green grass clumps
                        commands.spawn((
                            Sprite {
                                color: Color::srgb(0.4, 0.7, 0.3),
                                custom_size: Some(Vec2::new(15.0, 20.0)),
                                ..default()
                            },
                            Transform::from_xyz(pos_x, pos_y, -9.7),
                        ));
                    }
                    2 => {
                        // Light green grass
                        commands.spawn((
                            Sprite {
                                color: Color::srgb(0.5, 0.8, 0.4),
                                custom_size: Some(Vec2::new(12.0, 18.0)),
                                ..default()
                            },
                            Transform::from_xyz(pos_x, pos_y, -9.6),
                        ));
                    }
                    3 => {
                        // Yellow-tipped grass
                        commands.spawn((
                            Sprite {
                                color: Color::srgb(0.6, 0.7, 0.3),
                                custom_size: Some(Vec2::new(10.0, 22.0)),
                                ..default()
                            },
                            Transform::from_xyz(pos_x, pos_y, -9.7),
                        ));
                    }
                    4 => {
                        // Small grass patches
                        commands.spawn((
                            Sprite {
                                color: Color::srgb(0.45, 0.65, 0.35),
                                custom_size: Some(Vec2::new(20.0, 15.0)),
                                ..default()
                            },
                            Transform::from_xyz(pos_x, pos_y, -9.5),
                        ));
                    }
                    _ => {} // Default case
                }
            }
        }
    }

    fn spawn_flower_fields(commands: &mut Commands) {
        let mut rng = rand::rng();
        
        // Generate flowers completely randomly across the entire map
        for _ in 0..3000 {
            let pos_x = rng.random_range(-3500.0..3500.0);
            let pos_y = rng.random_range(-3500.0..3500.0);
            
            Self::spawn_random_flower(commands, pos_x, pos_y, &mut rng);
        }
        
        // Add some small flower clusters for variety
        for _ in 0..15 {
            let cluster_center_x = rng.random_range(-3000.0..3000.0);
            let cluster_center_y = rng.random_range(-3000.0..3000.0);
            
            for _ in 0..30 {
                let angle = rng.random_range(0.0..std::f32::consts::TAU);
                let radius = rng.random_range(0.0..150.0);
                let pos_x = cluster_center_x + angle.cos() * radius;
                let pos_y = cluster_center_y + angle.sin() * radius;
                
                Self::spawn_random_flower(commands, pos_x, pos_y, &mut rng);
            }
        }
    }

    fn spawn_random_flower(commands: &mut Commands, x: f32, y: f32, rng: &mut rand::rngs::ThreadRng) {
        let flower_type = rng.random_range(0_u32..8);
        
        match flower_type {
            0 => {
                // Red daisy-like flowers
                Self::spawn_flower(commands, x, y, Color::srgb(1.0, 0.2, 0.2), Color::srgb(1.0, 1.0, 0.3), 8.0);
            }
            1 => {
                // Blue cornflowers
                Self::spawn_flower(commands, x, y, Color::srgb(0.2, 0.4, 1.0), Color::srgb(1.0, 1.0, 0.0), 6.0);
            }
            2 => {
                // Yellow dandelions
                Self::spawn_flower(commands, x, y, Color::srgb(1.0, 0.9, 0.0), Color::srgb(0.8, 0.6, 0.2), 10.0);
            }
            3 => {
                // Purple wildflowers
                Self::spawn_flower(commands, x, y, Color::srgb(0.6, 0.2, 0.8), Color::srgb(1.0, 1.0, 0.2), 7.0);
            }
            4 => {
                // Pink flowers
                Self::spawn_flower(commands, x, y, Color::srgb(1.0, 0.5, 0.8), Color::srgb(0.9, 0.9, 0.1), 6.0);
            }
            5 => {
                // White daisies
                Self::spawn_flower(commands, x, y, Color::srgb(1.0, 1.0, 1.0), Color::srgb(1.0, 1.0, 0.0), 8.0);
            }
            6 => {
                // Orange flowers
                Self::spawn_flower(commands, x, y, Color::srgb(1.0, 0.6, 0.2), Color::srgb(0.9, 0.7, 0.1), 7.0);
            }
            7 => {
                // Lavender flowers
                Self::spawn_flower(commands, x, y, Color::srgb(0.7, 0.4, 0.9), Color::srgb(0.8, 0.8, 0.1), 5.0);
            }
            _ => {} // Default case
        }
    }

    fn spawn_flower(commands: &mut Commands, x: f32, y: f32, petal_color: Color, center_color: Color, size: f32) {
        // Flower stem
        commands.spawn((
            Sprite {
                color: Color::srgb(0.2, 0.5, 0.2),
                custom_size: Some(Vec2::new(2.0, 12.0)),
                ..default()
            },
            Transform::from_xyz(x, y, -9.4),
        ));

        // Flower petals
        commands.spawn((
            Sprite {
                color: petal_color,
                custom_size: Some(Vec2::new(size, size)),
                ..default()
            },
            Transform::from_xyz(x, y + 6.0, -9.3),
        ));

        // Flower center
        commands.spawn((
            Sprite {
                color: center_color,
                custom_size: Some(Vec2::new(size * 0.4, size * 0.4)),
                ..default()
            },
            Transform::from_xyz(x, y + 6.0, -9.2),
        ));
    }

    fn spawn_shrubs(commands: &mut Commands) {
        let mut rng = rand::rng();
        
        // Random shrub distribution across the entire map
        for _ in 0..300 {
            let x = rng.random_range(-3700.0..3700.0);
            let y = rng.random_range(-3700.0..3700.0);
            
            // Different shrub types
            let shrub_type = rng.random_range(0_u32..3);
            
            match shrub_type {
                0 => {
                    // Round green shrub
                    commands.spawn((
                        Sprite {
                            color: Color::srgb(0.3, 0.5, 0.2),
                            custom_size: Some(Vec2::new(25.0, 20.0)),
                            ..default()
                        },
                        Transform::from_xyz(x, y, -9.0),
                    ));
                }
                1 => {
                    // Dark green shrub
                    commands.spawn((
                        Sprite {
                            color: Color::srgb(0.2, 0.4, 0.1),
                            custom_size: Some(Vec2::new(30.0, 25.0)),
                            ..default()
                        },
                        Transform::from_xyz(x, y, -9.1),
                    ));
                }
                2 => {
                    // Light green shrub
                    commands.spawn((
                        Sprite {
                            color: Color::srgb(0.4, 0.6, 0.3),
                            custom_size: Some(Vec2::new(20.0, 18.0)),
                            ..default()
                        },
                        Transform::from_xyz(x, y, -9.2),
                    ));
                }
                _ => {} // Default case
            }
        }
        
        // Add some shrub clusters near trees for natural grouping
        for _ in 0..20 {
            let cluster_center_x = rng.random_range(-3500.0..3500.0);
            let cluster_center_y = rng.random_range(-3500.0..3500.0);
            
            for _ in 0..8 {
                let angle = rng.random_range(0.0..std::f32::consts::TAU);
                let radius = rng.random_range(0.0..100.0);
                let pos_x = cluster_center_x + angle.cos() * radius;
                let pos_y = cluster_center_y + angle.sin() * radius;
                
                let shrub_type = rng.random_range(0_u32..3);
                
                match shrub_type {
                    0 => {
                        commands.spawn((
                            Sprite {
                                color: Color::srgb(0.3, 0.5, 0.2),
                                custom_size: Some(Vec2::new(25.0, 20.0)),
                                ..default()
                            },
                            Transform::from_xyz(pos_x, pos_y, -9.0),
                        ));
                    }
                    1 => {
                        commands.spawn((
                            Sprite {
                                color: Color::srgb(0.2, 0.4, 0.1),
                                custom_size: Some(Vec2::new(30.0, 25.0)),
                                ..default()
                            },
                            Transform::from_xyz(pos_x, pos_y, -9.1),
                        ));
                    }
                    2 => {
                        commands.spawn((
                            Sprite {
                                color: Color::srgb(0.4, 0.6, 0.3),
                                custom_size: Some(Vec2::new(20.0, 18.0)),
                                ..default()
                            },
                            Transform::from_xyz(pos_x, pos_y, -9.2),
                        ));
                    }
                    _ => {}
                }
            }
        }
    }

    fn spawn_mushrooms(commands: &mut Commands) {
        let mut rng = rand::rng();
        
        // Random mushroom distribution across the map
        for _ in 0..150 {
            let x = rng.random_range(-3500.0..3500.0);
            let y = rng.random_range(-3500.0..3500.0);
            
            // Different mushroom types
            let mushroom_type = rng.random_range(0_u32..3);
            
            match mushroom_type {
                0 => {
                    // Red mushroom with white spots
                    commands.spawn((
                        Sprite {
                            color: Color::srgb(0.8, 0.2, 0.2),
                            custom_size: Some(Vec2::new(8.0, 6.0)),
                            ..default()
                        },
                        Transform::from_xyz(x, y + 4.0, -9.3),
                    ));
                    
                    // Stem
                    commands.spawn((
                        Sprite {
                            color: Color::srgb(0.9, 0.9, 0.9),
                            custom_size: Some(Vec2::new(3.0, 6.0)),
                            ..default()
                        },
                        Transform::from_xyz(x, y, -9.4),
                    ));
                    
                    // White spots
                    commands.spawn((
                        Sprite {
                            color: Color::srgb(1.0, 1.0, 1.0),
                            custom_size: Some(Vec2::new(2.0, 2.0)),
                            ..default()
                        },
                        Transform::from_xyz(x - 2.0, y + 5.0, -9.2),
                    ));
                }
                1 => {
                    // Brown mushroom
                    commands.spawn((
                        Sprite {
                            color: Color::srgb(0.4, 0.3, 0.2),
                            custom_size: Some(Vec2::new(10.0, 8.0)),
                            ..default()
                        },
                        Transform::from_xyz(x, y + 5.0, -9.3),
                    ));
                    
                    // Stem
                    commands.spawn((
                        Sprite {
                            color: Color::srgb(0.8, 0.7, 0.6),
                            custom_size: Some(Vec2::new(4.0, 8.0)),
                            ..default()
                        },
                        Transform::from_xyz(x, y, -9.4),
                    ));
                }
                2 => {
                    // Small beige mushrooms
                    commands.spawn((
                        Sprite {
                            color: Color::srgb(0.7, 0.6, 0.5),
                            custom_size: Some(Vec2::new(6.0, 4.0)),
                            ..default()
                        },
                        Transform::from_xyz(x, y + 3.0, -9.3),
                    ));
                    
                    // Stem
                    commands.spawn((
                        Sprite {
                            color: Color::srgb(0.8, 0.7, 0.6),
                            custom_size: Some(Vec2::new(2.0, 4.0)),
                            ..default()
                        },
                        Transform::from_xyz(x, y, -9.4),
                    ));
                }
                _ => {} // Default case
            }
        }
        
        // Add some mushroom clusters in shaded areas
        for _ in 0..25 {
            let cluster_center_x = rng.random_range(-3000.0..3000.0);
            let cluster_center_y = rng.random_range(-3000.0..3000.0);
            
            for _ in 0..6 {
                let angle = rng.random_range(0.0..std::f32::consts::TAU);
                let radius = rng.random_range(0.0..80.0);
                let pos_x = cluster_center_x + angle.cos() * radius;
                let pos_y = cluster_center_y + angle.sin() * radius;
                
                let mushroom_type = rng.random_range(0_u32..3);
                
                match mushroom_type {
                    0 => {
                        commands.spawn((
                            Sprite {
                                color: Color::srgb(0.8, 0.2, 0.2),
                                custom_size: Some(Vec2::new(8.0, 6.0)),
                                ..default()
                            },
                            Transform::from_xyz(pos_x, pos_y + 4.0, -9.3),
                        ));
                        
                        commands.spawn((
                            Sprite {
                                color: Color::srgb(0.9, 0.9, 0.9),
                                custom_size: Some(Vec2::new(3.0, 6.0)),
                                ..default()
                            },
                            Transform::from_xyz(pos_x, pos_y, -9.4),
                        ));
                        
                        commands.spawn((
                            Sprite {
                                color: Color::srgb(1.0, 1.0, 1.0),
                                custom_size: Some(Vec2::new(2.0, 2.0)),
                                ..default()
                            },
                            Transform::from_xyz(pos_x - 2.0, pos_y + 5.0, -9.2),
                        ));
                    }
                    1 => {
                        commands.spawn((
                            Sprite {
                                color: Color::srgb(0.4, 0.3, 0.2),
                                custom_size: Some(Vec2::new(10.0, 8.0)),
                                ..default()
                            },
                            Transform::from_xyz(pos_x, pos_y + 5.0, -9.3),
                        ));
                        
                        commands.spawn((
                            Sprite {
                                color: Color::srgb(0.8, 0.7, 0.6),
                                custom_size: Some(Vec2::new(4.0, 8.0)),
                                ..default()
                            },
                            Transform::from_xyz(pos_x, pos_y, -9.4),
                        ));
                    }
                    2 => {
                        commands.spawn((
                            Sprite {
                                color: Color::srgb(0.7, 0.6, 0.5),
                                custom_size: Some(Vec2::new(6.0, 4.0)),
                                ..default()
                            },
                            Transform::from_xyz(pos_x, pos_y + 3.0, -9.3),
                        ));
                        
                        commands.spawn((
                            Sprite {
                                color: Color::srgb(0.8, 0.7, 0.6),
                                custom_size: Some(Vec2::new(2.0, 4.0)),
                                ..default()
                            },
                            Transform::from_xyz(pos_x, pos_y, -9.4),
                        ));
                    }
                    _ => {}
                }
            }
        }
    }

    fn spawn_trees(commands: &mut Commands) {
        let mut rng = rand::rng();
        let mut tree_positions = Vec::new();
        
        // Generate random forest clusters across the map
        let forest_cluster_count = rng.random_range(8..12);
        
        for _ in 0..forest_cluster_count {
            let cluster_center_x = rng.random_range(-3000.0..3000.0);
            let cluster_center_y = rng.random_range(-3000.0..3000.0);
            let cluster_density = rng.random_range(5..10);
            
            for _ in 0..cluster_density {
                let angle = rng.random_range(0.0..std::f32::consts::TAU);
                let radius = rng.random_range(0.0..150.0);
                let pos_x = cluster_center_x + angle.cos() * radius;
                let pos_y = cluster_center_y + angle.sin() * radius;
                
                // Ensure trees stay within map boundaries and don't overlap
                if pos_x >= -3400.0 && pos_x <= 3400.0 && pos_y >= -3400.0 && pos_y <= 3400.0 {
                    if Self::is_position_valid_for_tree(pos_x, pos_y, &tree_positions) {
                        tree_positions.push((pos_x, pos_y));
                        Self::spawn_random_tree(commands, pos_x, pos_y, &mut rng, tree_positions.len());
                    }
                }
            }
        }
        
        // Add scattered individual trees
        let scattered_tree_count = rng.random_range(30..50);
        
        for _ in 0..scattered_tree_count {
            let pos_x = rng.random_range(-3300.0..3300.0);
            let pos_y = rng.random_range(-3300.0..3300.0);
            
            if Self::is_position_valid_for_tree(pos_x, pos_y, &tree_positions) {
                tree_positions.push((pos_x, pos_y));
                Self::spawn_random_tree(commands, pos_x, pos_y, &mut rng, tree_positions.len());
            }
        }
        
        // Add some edge trees along the map borders
        let edge_tree_count = rng.random_range(20..30);
        
        for _ in 0..edge_tree_count {
            let side = rng.random_range(0..4);
            let (pos_x, pos_y) = match side {
                0 => (-3300.0 + rng.random_range(0.0..100.0), rng.random_range(-3200.0..3200.0)), // Left edge
                1 => (3300.0 - rng.random_range(0.0..100.0), rng.random_range(-3200.0..3200.0)),  // Right edge
                2 => (rng.random_range(-3200.0..3200.0), -3300.0 + rng.random_range(0.0..100.0)), // Bottom edge
                _ => (rng.random_range(-3200.0..3200.0), 3300.0 - rng.random_range(0.0..100.0)),  // Top edge
            };
            
            if Self::is_position_valid_for_tree(pos_x, pos_y, &tree_positions) {
                tree_positions.push((pos_x, pos_y));
                Self::spawn_random_tree(commands, pos_x, pos_y, &mut rng, tree_positions.len());
            }
        }
    }

    fn is_position_valid_for_tree(x: f32, y: f32, existing_positions: &[(f32, f32)]) -> bool {
        const MIN_DISTANCE: f32 = 180.0; // Minimum distance between trees
            
        for &(existing_x, existing_y) in existing_positions {
            let distance = ((x - existing_x).powi(2) + (y - existing_y).powi(2)).sqrt();
            if distance < MIN_DISTANCE {
                return false;
            }
        }
        true
    }

    fn spawn_random_tree(commands: &mut Commands, x: f32, y: f32, rng: &mut rand::rngs::ThreadRng, tree_index: usize) {
        let tree_type = rng.random_range(0..3);
        
        // Use unique Z-values based on tree index to prevent overlap rendering issues
        let z_trunk = 3.0 + (tree_index as f32) * 0.01;
        let z_canopy = 2.0 + (tree_index as f32) * 0.01;
        
        // Add slight size variation for more natural look
        let size_variation = 0.8 + rng.random_range(0.0..0.4);
        
        match tree_type {
            0 => {
                // Oak Tree
                let trunk_width = 60.0 * size_variation;
                let trunk_height = 80.0 * size_variation;
                let canopy_size = 120.0 * size_variation;
                
                commands.spawn((
                    Sprite {
                        color: Color::srgb(0.4, 0.3, 0.2),
                        custom_size: Some(Vec2::new(trunk_width, trunk_height)),
                        ..default()
                    },
                    Transform::from_xyz(x, y + 40.0 * size_variation, z_trunk),
                ));
                
                commands.spawn((
                    Sprite {
                        color: Color::srgb(0.2, 0.6, 0.2),
                        custom_size: Some(Vec2::new(canopy_size, canopy_size)),
                        ..default()
                    },
                    Transform::from_xyz(x, y + 80.0 * size_variation, z_canopy),
                ));
            },
            1 => {
                // Pine Tree
                let trunk_width = 40.0 * size_variation;
                let trunk_height = 100.0 * size_variation;
                let canopy_width = 80.0 * size_variation;
                let canopy_height = 140.0 * size_variation;
                
                commands.spawn((
                    Sprite {
                        color: Color::srgb(0.3, 0.2, 0.1),
                        custom_size: Some(Vec2::new(trunk_width, trunk_height)),
                        ..default()
                    },
                    Transform::from_xyz(x, y + 50.0 * size_variation, z_trunk),
                ));
                
                commands.spawn((
                    Sprite {
                        color: Color::srgb(0.1, 0.4, 0.1),
                        custom_size: Some(Vec2::new(canopy_width, canopy_height)),
                        ..default()
                    },
                    Transform::from_xyz(x, y + 100.0 * size_variation, z_canopy),
                ));
            },
            _ => {
                // Apple Tree
                let trunk_width = 50.0 * size_variation;
                let trunk_height = 70.0 * size_variation;
                let canopy_size = 100.0 * size_variation;
                
                commands.spawn((
                    Sprite {
                        color: Color::srgb(0.5, 0.3, 0.2),
                        custom_size: Some(Vec2::new(trunk_width, trunk_height)),
                        ..default()
                    },
                    Transform::from_xyz(x, y + 35.0 * size_variation, z_trunk),
                ));
                
                commands.spawn((
                    Sprite {
                        color: Color::srgb(0.3, 0.7, 0.3),
                        custom_size: Some(Vec2::new(canopy_size, canopy_size)),
                        ..default()
                    },
                    Transform::from_xyz(x, y + 70.0 * size_variation, z_canopy),
                ));
            }
        }
    }

    fn spawn_houses(commands: &mut Commands) {
        let mut rng = rand::rng();
        
        // Generate random number of houses (4-8 houses)
        let house_count = rng.random_range(4..9);
        
        for i in 0..house_count {
            // Generate random position within map boundaries
            let x = rng.random_range(-3200.0..3200.0);
            let y = rng.random_range(-3200.0..3200.0);
            
            // Random house color
            let r = rng.random_range(0.4..0.9);
            let g = rng.random_range(0.3..0.7);
            let b = rng.random_range(0.2..0.6);
            
            // 30% chance to spawn a barn, 70% chance for a house
            if rng.random_bool(0.3) {
                Self::spawn_barn(commands, x, y);
            } else {
                Self::spawn_house(commands, x, y, r, g, b);
            }
        }
    }

    fn spawn_house(commands: &mut Commands, x: f32, y: f32, r: f32, g: f32, b: f32) {
        // House base
        commands.spawn((
            Sprite {
                color: Color::srgb(r, g, b),
                custom_size: Some(Vec2::new(120.0, 80.0)),
                ..default()
            },
            Transform::from_xyz(x, y, -6.0),
        ));

        // House roof
        commands.spawn((
            Sprite {
                color: Color::srgb(0.6, 0.3, 0.2),
                custom_size: Some(Vec2::new(140.0, 40.0)),
                ..default()
            },
            Transform::from_xyz(x, y + 50.0, -5.5),
        ));

        // Door
        commands.spawn((
            Sprite {
                color: Color::srgb(0.4, 0.2, 0.1),
                custom_size: Some(Vec2::new(20.0, 35.0)),
                ..default()
            },
            Transform::from_xyz(x, y - 20.0, -5.0),
        ));

        // Windows
        commands.spawn((
            Sprite {
                color: Color::srgb(0.7, 0.9, 1.0),
                custom_size: Some(Vec2::new(15.0, 15.0)),
                ..default()
            },
            Transform::from_xyz(x - 25.0, y + 5.0, -5.0),
        ));

        commands.spawn((
            Sprite {
                color: Color::srgb(0.7, 0.9, 1.0),
                custom_size: Some(Vec2::new(15.0, 15.0)),
                ..default()
            },
            Transform::from_xyz(x + 25.0, y + 5.0, -5.0),
        ));

        // Chimney
        commands.spawn((
            Sprite {
                color: Color::srgb(0.5, 0.3, 0.2),
                custom_size: Some(Vec2::new(15.0, 25.0)),
                ..default()
            },
            Transform::from_xyz(x + 30.0, y + 35.0, -5.0),
        ));
    }

    fn spawn_barn(commands: &mut Commands, x: f32, y: f32) {
        // Barn base (larger)
        commands.spawn((
            Sprite {
                color: Color::srgb(0.7, 0.5, 0.3),
                custom_size: Some(Vec2::new(160.0, 100.0)),
                ..default()
            },
            Transform::from_xyz(x, y, -6.0),
        ));

        // Barn roof (triangular shape simulated with rectangle)
        commands.spawn((
            Sprite {
                color: Color::srgb(0.5, 0.3, 0.2),
                custom_size: Some(Vec2::new(180.0, 50.0)),
                ..default()
            },
            Transform::from_xyz(x, y + 60.0, -5.5),
        ));

        // Large barn door
        commands.spawn((
            Sprite {
                color: Color::srgb(0.4, 0.2, 0.1),
                custom_size: Some(Vec2::new(40.0, 60.0)),
                ..default()
            },
            Transform::from_xyz(x, y - 15.0, -5.0),
        ));

        // Loft window
        commands.spawn((
            Sprite {
                color: Color::srgb(0.7, 0.9, 1.0),
                custom_size: Some(Vec2::new(25.0, 20.0)),
                ..default()
            },
            Transform::from_xyz(x, y + 20.0, -5.0),
        ));
    }

    fn spawn_water(commands: &mut Commands) {
        // Pond
        commands.spawn((
            Sprite {
                color: Color::srgba(0.3, 0.5, 0.8, 0.7),
                custom_size: Some(Vec2::new(100.0, 80.0)),
                ..default()
            },
            Transform::from_xyz(100.0, 50.0, -8.0),
        ));

        // Water sparkles/decorations
        let sparkle_positions = [
            (80.0, 40.0), (120.0, 45.0), (90.0, 60.0),
            (110.0, 55.0), (100.0, 35.0),
        ];

        for pos in sparkle_positions.iter() {
            commands.spawn((
                Sprite {
                    color: Color::srgba(0.8, 0.9, 1.0, 0.8),
                    custom_size: Some(Vec2::new(4.0, 4.0)),
                    ..default()
                },
                Transform::from_xyz(pos.0, pos.1, -7.5),
            ));
        }
    }

    fn spawn_decorations(commands: &mut Commands) {
        // Additional decorative elements around buildings and paths
        // Note: Flowers are now handled by the main vegetation system

        // Rocks
        let rock_positions = [
            (-320.0, -80.0), (280.0, -90.0), (-50.0, 150.0),
            (150.0, -200.0), (320.0, 120.0),
        ];

        for pos in rock_positions.iter() {
            commands.spawn((
                Sprite {
                    color: Color::srgb(0.5, 0.5, 0.5),
                    custom_size: Some(Vec2::new(20.0, 15.0)),
                    ..default()
                },
                Transform::from_xyz(pos.0, pos.1, -7.0),
            ));
        }

        // Fence segments
        let fence_positions = [
            (-200.0, 0.0), (-180.0, 0.0), (-160.0, 0.0),
            (-140.0, 0.0), (-120.0, 0.0), (120.0, 0.0),
            (140.0, 0.0), (160.0, 0.0), (180.0, 0.0), (200.0, 0.0),
        ];

        for pos in fence_positions.iter() {
            commands.spawn((
                Sprite {
                    color: Color::srgb(0.6, 0.4, 0.2),
                    custom_size: Some(Vec2::new(15.0, 25.0)),
                    ..default()
                },
                Transform::from_xyz(pos.0, pos.1, -7.0),
            ));
        }
    }

    
    pub fn animate_water(
        time: Res<Time>,
        mut query: Query<&mut Transform, With<WaterDecoration>>,
    ) {
        for mut transform in query.iter_mut() {
            // Gentle water animation
            let wave = time.elapsed_secs().sin() * 0.5;
            transform.translation.y += wave * 0.1;
        }
    }
}

#[derive(Component)]
pub struct WaterDecoration;