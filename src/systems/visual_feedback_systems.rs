use bevy::prelude::*;
use crate::components::*;

pub struct VisualFeedbackSystems;

impl VisualFeedbackSystems {
    pub fn spawn_interaction_effect(
        commands: &mut Commands,
        position: Vec3,
        text: String,
        color: Color,
    ) {
        commands.spawn((
            Transform::from_xyz(position.x, position.y + 50.0, position.z + 1.0),
            InteractionEffect {
                text,
                timer: Timer::from_seconds(2.0, TimerMode::Once),
                color,
            },
        ));
    }
    
    pub fn spawn_floating_text(
        commands: &mut Commands,
        position: Vec3,
        text: String,
        color: Color,
    ) {
        commands.spawn((
            Transform::from_xyz(position.x, position.y + 30.0, position.z + 1.0),
            FloatingText {
                text,
                velocity: Vec3::new(0.0, 50.0, 0.0),
                timer: Timer::from_seconds(1.5, TimerMode::Once),
                color,
            },
        ));
    }
    
    pub fn update_interaction_effects(
        time: Res<Time>,
        mut commands: Commands,
        mut query: Query<(Entity, &mut InteractionEffect, &mut Transform)>,
    ) {
        for (entity, mut effect, mut transform) in query.iter_mut() {
            effect.timer.tick(time.delta());
            
            // Fade out effect
            let alpha = 1.0 - effect.timer.fraction();
            transform.translation.y += 20.0 * time.delta_secs();
            
            if effect.timer.finished() {
                commands.entity(entity).despawn();
            }
        }
    }
    
    pub fn update_floating_text(
        time: Res<Time>,
        mut commands: Commands,
        mut query: Query<(Entity, &mut FloatingText, &mut Transform)>,
    ) {
        for (entity, mut text, mut transform) in query.iter_mut() {
            text.timer.tick(time.delta());
            
            // Move text upward
            transform.translation += text.velocity * time.delta_secs();
            
            // Fade out
            let alpha = 1.0 - text.timer.fraction();
            
            if text.timer.finished() {
                commands.entity(entity).despawn();
            }
        }
    }
    
    pub fn show_feeding_feedback(
        commands: &mut Commands,
        animal_transform: &Transform,
        animal_type: AnimalType,
        item_name: &str,
        friendship_increase: u32,
    ) {
        let position = animal_transform.translation;
        
        // Show feeding effect
        Self::spawn_interaction_effect(
            commands,
            position,
            format!("+{}", friendship_increase),
            Color::linear_rgb(0.0, 1.0, 0.0),
        );
        
        // Show item used
        Self::spawn_floating_text(
            commands,
            position,
            format!("喂食 {}", item_name),
            Color::linear_rgb(1.0, 1.0, 0.0),
        );
    }
    
    pub fn show_petting_feedback(
        commands: &mut Commands,
        animal_transform: &Transform,
        animal_type: AnimalType,
    ) {
        let position = animal_transform.translation;
        
        // Show heart effect
        Self::spawn_interaction_effect(
            commands,
            position,
            "❤️".to_string(),
            Color::linear_rgb(1.0, 0.0, 1.0),
        );
        
        // Show petting text
        Self::spawn_floating_text(
            commands,
            position,
            "摸摸".to_string(),
            Color::linear_rgb(1.0, 0.0, 1.0),
        );
    }
    
    pub fn show_interaction_prompts(
        mut query: Query<(&Transform, &Interactable, &Animal), Without<Player>>,
        player_query: Query<&Transform, With<Player>>,
    ) {
        let player_transform = match player_query.single() {
            Ok(transform) => transform,
            Err(_) => return,
        };
        
        let player_pos = Vec2::new(player_transform.translation.x, player_transform.translation.y);
        
        for (transform, interactable, animal) in query.iter() {
            let animal_pos = Vec2::new(transform.translation.x, transform.translation.y);
            let distance = (player_pos - animal_pos).length();
            
            if distance <= interactable.interaction_range {
                let animal_name = Self::get_animal_name(animal.animal_type);
                let prompt = match interactable.interaction_type {
                    InteractionType::Feed => format!("按空格键喂食{}", animal_name),
                    InteractionType::Pet => format!("按空格键抚摸{}", animal_name),
                    InteractionType::Play => format!("按空格键和{}玩耍", animal_name),
                };
                println!("{}", prompt);
            }
        }
    }
    
    fn get_animal_name(animal_type: AnimalType) -> &'static str {
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
}