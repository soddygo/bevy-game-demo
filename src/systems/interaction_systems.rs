use bevy::prelude::*;
use crate::components::*;
use crate::traits::*;
use crate::systems::visual_feedback_systems as visual_feedback;
use crate::systems::animal_animation_systems as animal_animation;

pub struct InteractionSystems;

impl InteractionSystems {
    pub fn handle_player_interactions(
        keyboard: Res<ButtonInput<KeyCode>>,
        mut commands: Commands,
        mut query: Query<(Entity, &Transform, &mut Interactable, &mut Animal, &mut AnimalAnimation)>,
        player_query: Query<&Transform, (With<Player>, Without<Animal>)>,
        mut inventory: ResMut<Inventory>,
    ) {
        if !keyboard.just_pressed(KeyCode::Space) {
            return;
        }

        let player_transform = match player_query.single() {
            Ok(transform) => transform,
            Err(_) => return,
        };

        let player_pos = Vec2::new(player_transform.translation.x, player_transform.translation.y);

        for (entity, transform, mut interactable, mut animal, mut animation) in query.iter_mut() {
            let animal_pos = Vec2::new(transform.translation.x, transform.translation.y);
            let distance = (player_pos - animal_pos).length();

            if distance <= interactable.interaction_range {
                Self::interact_with_animal(&mut animal, &mut inventory, &interactable.interaction_type);
                
                // Increase friendship based on interaction type
                match interactable.interaction_type {
                    InteractionType::Pet => {
                        animal.friendship_level += 1;
                        println!("抚摸了{}! 好感度+1", Self::get_animal_name(animal.animal_type));
                        
                        // Trigger happy animation
                        animation.is_happy = true;
                        animation.animation_timer.reset();
                        
                        // Add visual feedback
                        visual_feedback::VisualFeedbackSystems::show_petting_feedback(
                            &mut commands,
                            &transform,
                            animal.animal_type,
                        );
                        
                        // Spawn heart effects
                        animal_animation::AnimalAnimationSystems::trigger_happy_animation(
                            &mut commands,
                            &transform,
                        );
                    }
                    InteractionType::Feed => {
                        let selected_item = inventory.get_selected_item().cloned();
                        
                        if let Some(item) = selected_item {
                            if item.can_use_on_animal(animal.animal_type) {
                                let friendship_increase = item.get_friendship_bonus();
                                animal.friendship_level += friendship_increase;
                                animal.hunger -= item.get_hunger_reduction();
                                animal.hunger = animal.hunger.max(0.0);
                                
                                // 消耗物品
                                inventory.remove_item(&item.item_type, 1);
                                
                                println!("喂食了{} {}! 好感度+{}, 饥饿度-{}", 
                                    Self::get_animal_name(animal.animal_type),
                                    item.name,
                                    friendship_increase,
                                    item.get_hunger_reduction());
                                
                                // Trigger feeding animation
                                animation.is_eating = true;
                                animation.animation_timer.reset();
                                
                                // Add visual feedback
                                visual_feedback::VisualFeedbackSystems::show_feeding_feedback(
                                    &mut commands,
                                    &transform,
                                    animal.animal_type,
                                    &item.name,
                                    friendship_increase,
                                );
                                
                                // Spawn feeding effects
                                animal_animation::AnimalAnimationSystems::trigger_feeding_animation(
                                    &mut commands,
                                    &transform,
                                );
                            } else {
                                println!("{}不喜欢吃{}", Self::get_animal_name(animal.animal_type), item.name);
                            }
                        } else {
                            println!("没有选择任何物品！");
                        }
                    }
                    InteractionType::Play => {
                        animal.friendship_level += 2;
                        println!("和{}玩耍! 好感度+2", Self::get_animal_name(animal.animal_type));
                        
                        // Trigger happy animation
                        animation.is_happy = true;
                        animation.animation_timer.reset();
                        
                        // Add visual feedback
                        visual_feedback::VisualFeedbackSystems::spawn_interaction_effect(
                            &mut commands,
                            transform.translation,
                            "+2".to_string(),
                            Color::linear_rgb(0.0, 0.0, 1.0),
                        );
                        
                        // Spawn heart effects
                        animal_animation::AnimalAnimationSystems::trigger_happy_animation(
                            &mut commands,
                            &transform,
                        );
                    }
                }
            }
        }
    }

    pub fn interact_with_animal(
        animal: &mut Animal,
        inventory: &mut Inventory,
        interaction_type: &InteractionType,
    ) {
        match interaction_type {
            InteractionType::Pet => {
                println!("{}很高兴被抚摸!", Self::get_animal_name(animal.animal_type));
            }
            InteractionType::Feed => {
                if let Some(item) = inventory.get_selected_item() {
                    if item.can_use_on_animal(animal.animal_type) {
                        let favorite_food = animal.get_favorite_food();
                        if item.item_type == favorite_food {
                            println!("{}最喜欢吃{}了!", Self::get_animal_name(animal.animal_type), item.name);
                        } else {
                            println!("{}吃了{}", Self::get_animal_name(animal.animal_type), item.name);
                        }
                    } else {
                        println!("{}不喜欢吃{}", Self::get_animal_name(animal.animal_type), item.name);
                    }
                }
            }
            InteractionType::Play => {
                println!("{}很开心地玩耍!", Self::get_animal_name(animal.animal_type));
            }
        }
    }

    pub fn get_animal_name(animal_type: AnimalType) -> &'static str {
        match animal_type {
            AnimalType::Dog => "小狗",
            AnimalType::Cat => "小猫",
            AnimalType::Chicken => "小鸡",
            AnimalType::Cow => "奶牛",
            AnimalType::Sheep => "小羊",
            AnimalType::Pig => "小猪",
            AnimalType::Duck => "小鸭",
            AnimalType::Horse => "小马",
        }
    }

    pub fn show_interaction_prompts(
        mut query: Query<(Entity, &Transform, &Interactable, &Animal)>,
        player_query: Query<&Transform, (With<Player>, Without<Animal>)>,
        mut gizmos: Gizmos,
    ) {
        let player_transform = match player_query.single() {
            Ok(transform) => transform,
            Err(_) => return,
        };

        let player_pos = Vec2::new(player_transform.translation.x, player_transform.translation.y);

        for (entity, transform, interactable, animal) in query.iter_mut() {
            let animal_pos = Vec2::new(transform.translation.x, transform.translation.y);
            let distance = (player_pos - animal_pos).length();

            if distance <= interactable.interaction_range {
                // Show interaction prompt
                let prompt_pos = Vec3::new(
                    transform.translation.x,
                    transform.translation.y + 30.0,
                    2.0,
                );
                
                // Draw interaction range circle
                gizmos.circle_2d(
                    animal_pos,
                    interactable.interaction_range,
                    Color::srgba(0.3, 0.8, 0.3, 0.3),
                );
                
                // In a real implementation, you would render text here
                // For now, we'll just log the interaction
                if distance <= interactable.interaction_range * 0.8 {
                    let personality = animal.get_personality();
                    let prompt = format!("按空格键{}{}", 
                        interactable.get_interaction_prompt(),
                        Self::get_animal_name(animal.animal_type));
                    println!("{} - {}", prompt, personality);
                }
            }
        }
    }

    pub fn update_animal_behavior(
        time: Res<Time>,
        mut query: Query<(&mut AnimalAI, &mut Animal, &Transform)>,
        player_query: Query<&Transform, (With<Player>, Without<Animal>)>,
    ) {
        let player_transform = match player_query.single() {
            Ok(transform) => transform,
            Err(_) => return,
        };

        let player_pos = Vec2::new(player_transform.translation.x, player_transform.translation.y);

        for (mut ai, mut animal, transform) in query.iter_mut() {
            let animal_pos = Vec2::new(transform.translation.x, transform.translation.y);
            let distance_to_player = (player_pos - animal_pos).length();

            // Animals with high friendship may follow the player
            if animal.can_follow_player(animal.friendship_level) && distance_to_player < 100.0 {
                if ai.state != AnimalState::Following {
                    ai.state = AnimalState::Following;
                    println!("{}开始跟随你!", Self::get_animal_name(animal.animal_type));
                }
                
                // Move towards player
                if distance_to_player > 30.0 {
                    let direction = (player_pos - animal_pos).normalize();
                    ai.target_position = player_pos;
                }
            } else if ai.state == AnimalState::Following {
                ai.state = AnimalState::Idle;
                println!("{}停止跟随", Self::get_animal_name(animal.animal_type));
            }

            // Update hunger over time
            animal.hunger += time.delta_secs() * 0.5;
            if animal.hunger > 80.0 {
                // Hungry animals seek food
                if ai.state == AnimalState::Idle {
                    ai.state = AnimalState::Wandering;
                }
            }
        }
    }
}