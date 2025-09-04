use bevy::prelude::*;
use rand::Rng;
use crate::components::*;
use crate::systems::animal_animation_systems as animal_animation;

pub struct AnimalSystems;

impl AnimalSystems {
    pub fn spawn_animals(mut commands: Commands) {
        let mut rng = rand::rng();
        
        // Spawn random number of each animal type
        let dog_count = rng.random_range(3..8);
        let cat_count = rng.random_range(3..8);
        let chicken_group_count = rng.random_range(4..10);
        let cow_count = rng.random_range(3..7);
        let sheep_count = rng.random_range(3..7);
        let pig_count = rng.random_range(3..7);
        let duck_count = rng.random_range(3..8);
        let horse_count = rng.random_range(2..5);
        
        println!("正在生成动物: 狗{}只, 猫{}只, 鸡{}组, 奶牛{}头, 羊{}只, 猪{}只, 鸭{}只, 马{}匹", 
            dog_count, cat_count, chicken_group_count, cow_count, sheep_count, pig_count, duck_count, horse_count);
        
        // Spawn dogs at random locations
        for _ in 0..dog_count {
            let x = rng.random_range(-400.0..400.0);
            let y = rng.random_range(-400.0..400.0);
            animal_animation::AnimalAnimationSystems::spawn_animal_with_animation(
                &mut commands,
                Vec3::new(x, y, 1.0),
                AnimalType::Dog,
            );
        }
        
        // Spawn cats at random locations
        for _ in 0..cat_count {
            let x = rng.random_range(-400.0..400.0);
            let y = rng.random_range(-400.0..400.0);
            animal_animation::AnimalAnimationSystems::spawn_animal_with_animation(
                &mut commands,
                Vec3::new(x, y, 1.0),
                AnimalType::Cat,
            );
        }
        
        // Spawn chicken groups at random locations
        for _ in 0..chicken_group_count {
            let x = rng.random_range(-400.0..400.0);
            let y = rng.random_range(-400.0..400.0);
            animal_animation::AnimalAnimationSystems::spawn_animal_with_animation(
                &mut commands,
                Vec3::new(x, y, 1.0),
                AnimalType::Chicken,
            );
        }
        
        // Spawn cows at random locations
        for _ in 0..cow_count {
            let x = rng.random_range(-400.0..400.0);
            let y = rng.random_range(-400.0..400.0);
            animal_animation::AnimalAnimationSystems::spawn_animal_with_animation(
                &mut commands,
                Vec3::new(x, y, 1.0),
                AnimalType::Cow,
            );
        }
        
        // Spawn sheep at random locations
        for _ in 0..sheep_count {
            let x = rng.random_range(-400.0..400.0);
            let y = rng.random_range(-400.0..400.0);
            animal_animation::AnimalAnimationSystems::spawn_animal_with_animation(
                &mut commands,
                Vec3::new(x, y, 1.0),
                AnimalType::Sheep,
            );
        }
        
        // Spawn pigs at random locations
        for _ in 0..pig_count {
            let x = rng.random_range(-400.0..400.0);
            let y = rng.random_range(-400.0..400.0);
            animal_animation::AnimalAnimationSystems::spawn_animal_with_animation(
                &mut commands,
                Vec3::new(x, y, 1.0),
                AnimalType::Pig,
            );
        }
        
        // Spawn ducks at random locations
        for _ in 0..duck_count {
            let x = rng.random_range(-400.0..400.0);
            let y = rng.random_range(-400.0..400.0);
            animal_animation::AnimalAnimationSystems::spawn_animal_with_animation(
                &mut commands,
                Vec3::new(x, y, 1.0),
                AnimalType::Duck,
            );
        }
        
        // Spawn horses at random locations
        for _ in 0..horse_count {
            let x = rng.random_range(-400.0..400.0);
            let y = rng.random_range(-400.0..400.0);
            animal_animation::AnimalAnimationSystems::spawn_animal_with_animation(
                &mut commands,
                Vec3::new(x, y, 1.0),
                AnimalType::Horse,
            );
        }
    }

    pub fn animal_movement(
        time: Res<Time>,
        mut query: Query<(&mut Transform, &AnimalAI, &Animal)>,
    ) {
        for (mut transform, ai, animal) in query.iter_mut() {
            let current_pos = Vec2::new(transform.translation.x, transform.translation.y);
            let distance_to_target = (ai.target_position - current_pos).length();

            if distance_to_target > 5.0 {
                let direction = (ai.target_position - current_pos).normalize();
                let movement = direction * animal.speed * time.delta_secs();
                transform.translation.x += movement.x;
                transform.translation.y += movement.y;
            }
        }
    }

    pub fn animal_idle_behavior(
        time: Res<Time>,
        mut query: Query<(&mut AnimalAI, &Transform, &Animal)>,
    ) {
        for (mut ai, transform, animal) in query.iter_mut() {
            ai.idle_timer.tick(time.delta());
            
            if ai.idle_timer.just_finished() {
                match ai.state {
                    AnimalState::Idle => {
                        // Decide what to do next based on animal type and randomness
                        let mut rng = rand::rng();
                        let action_choice = rng.random_range(0..100);
                        
                        match action_choice {
                            0..=40 => {
                                // 40% chance to start wandering
                                ai.state = AnimalState::Wandering;
                                let current_pos = Vec2::new(transform.translation.x, transform.translation.y);
                                let wander_distance = match animal.animal_type {
                                    AnimalType::Dog | AnimalType::Cat => 150.0, // Dogs and cats wander farther
                                    AnimalType::Horse => 200.0, // Horses wander the farthest
                                    AnimalType::Chicken | AnimalType::Duck => 80.0, // Birds stay closer
                                    _ => 120.0, // Other animals
                                };
                                let random_offset = Vec2::new(
                                    rng.random_range(-wander_distance..wander_distance),
                                    rng.random_range(-wander_distance..wander_distance),
                                );
                                ai.target_position = current_pos + random_offset;
                                
                                // Set wandering duration based on animal type
                                let wander_duration = match animal.animal_type {
                                    AnimalType::Dog | AnimalType::Cat => rng.random_range(3.0..8.0),
                                    AnimalType::Horse => rng.random_range(4.0..10.0),
                                    AnimalType::Chicken | AnimalType::Duck => rng.random_range(2.0..5.0),
                                    _ => rng.random_range(3.0..7.0),
                                };
                                ai.idle_timer.set_duration(std::time::Duration::from_secs_f32(wander_duration));
                            },
                            41..=70 => {
                                // 30% chance to rest (stay idle longer)
                                ai.state = AnimalState::Idle;
                                let rest_duration = match animal.animal_type {
                                    AnimalType::Cat => rng.random_range(4.0..12.0), // Cats rest longer
                                    AnimalType::Dog => rng.random_range(3.0..8.0),
                                    AnimalType::Cow | AnimalType::Sheep => rng.random_range(5.0..10.0), // Farm animals rest
                                    _ => rng.random_range(2.0..6.0),
                                };
                                ai.idle_timer.set_duration(std::time::Duration::from_secs_f32(rest_duration));
                            },
                            71..=85 => {
                                // 15% chance to look for food
                                ai.state = AnimalState::Eating;
                                let eat_duration = match animal.animal_type {
                                    AnimalType::Pig => rng.random_range(4.0..8.0), // Pigs eat longer
                                    AnimalType::Cow | AnimalType::Sheep => rng.random_range(3.0..7.0),
                                    _ => rng.random_range(2.0..5.0),
                                };
                                ai.idle_timer.set_duration(std::time::Duration::from_secs_f32(eat_duration));
                            },
                            _ => {
                                // 15% chance to continue idle briefly
                                ai.state = AnimalState::Idle;
                                ai.idle_timer.set_duration(std::time::Duration::from_secs_f32(2.0));
                            }
                        }
                    },
                    AnimalState::Wandering => {
                        // Finished wandering, return to idle
                        ai.state = AnimalState::Idle;
                        let mut rng = rand::rng();
                        let rest_after_wander = rng.random_range(2.0..6.0);
                        ai.idle_timer.set_duration(std::time::Duration::from_secs_f32(rest_after_wander));
                    },
                    AnimalState::Following => {
                        // Continue following, reset timer
                        ai.idle_timer.set_duration(std::time::Duration::from_secs_f32(2.0));
                    },
                    AnimalState::Eating => {
                        // Finished eating, return to idle
                        ai.state = AnimalState::Idle;
                        let mut rng = rand::rng();
                        let rest_after_eating = rng.random_range(3.0..7.0);
                        ai.idle_timer.set_duration(std::time::Duration::from_secs_f32(rest_after_eating));
                    },
                }
            }
        }
    }

    pub fn keep_animals_in_bounds(
        mut query: Query<(&mut Transform, &mut AnimalAI, &Animal)>,
    ) {
        for (mut transform, mut ai, animal) in query.iter_mut() {
            let current_pos = Vec2::new(transform.translation.x, transform.translation.y);
            
            // Define boundaries based on animal type
            let boundary = match animal.animal_type {
                AnimalType::Chicken | AnimalType::Duck => 450.0, // Birds stay closer to center
                AnimalType::Dog | AnimalType::Cat => 500.0,     // Pets can roam farther
                AnimalType::Horse => 600.0,                    // Horses have largest range
                _ => 500.0,                                    // Default range
            };
            
            // If animal is too far from center, gently guide it back
            if current_pos.length() > boundary {
                let direction_to_center = -current_pos.normalize();
                let push_strength = (current_pos.length() - boundary) * 0.1;
                let push_vector = direction_to_center * push_strength;
                
                transform.translation.x += push_vector.x;
                transform.translation.y += push_vector.y;
                
                // Update target position to be within bounds
                if ai.state == AnimalState::Wandering {
                    let max_target_distance = boundary * 0.8;
                    if ai.target_position.length() > max_target_distance {
                        ai.target_position = ai.target_position.normalize() * max_target_distance;
                    }
                }
            }
        }
    }

    pub fn debug_animal_behavior(
        query: Query<(&AnimalAI, &Animal)>,
        time: Res<Time>,
    ) {
        // Log animal behavior every 5 seconds
        static mut LAST_LOG: f32 = 0.0;
        unsafe {
            if time.elapsed_secs() - LAST_LOG > 5.0 {
                LAST_LOG = time.elapsed_secs();
                
                let mut state_counts = std::collections::HashMap::new();
                let mut animal_counts = std::collections::HashMap::new();
                
                for (ai, animal) in query.iter() {
                    // Count animals by type
                    *animal_counts.entry(animal.animal_type).or_insert(0) += 1;
                    
                    // Count states
                    *state_counts.entry(ai.state).or_insert(0) += 1;
                }
                
                println!("动物行为统计 - {:?}", state_counts);
                println!("动物数量统计 - {:?}", animal_counts);
            }
        }
    }
}