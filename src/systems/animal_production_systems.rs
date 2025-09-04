use bevy::prelude::*;
use std::collections::HashSet;
use crate::components::*;

pub struct AnimalProductionSystems;

impl AnimalProductionSystems {
    pub fn update_animal_production(
        time: Res<Time>,
        mut query: Query<(Entity, &mut AnimalProduction, &Animal)>,
        mut commands: Commands,
    ) {
        for (entity, mut production, animal) in query.iter_mut() {
            if production.update(time.delta()) {
                println!("{} 可以生产了！", Self::get_animal_name(animal.animal_type));
                
                // Add visual indicator that animal can produce
                commands.entity(entity).insert(ProductionIndicator {
                    timer: Timer::from_seconds(30.0, TimerMode::Once),
                });
            }
        }
    }
    
    pub fn handle_production_collection(
        keyboard: Res<ButtonInput<KeyCode>>,
        mut query: Query<(Entity, &Transform, &mut AnimalProduction, &Animal, Option<&ProductionIndicator>)>,
        player_query: Query<&Transform, With<Player>>,
        mut commands: Commands,
        mut inventory: ResMut<Inventory>,
    ) {
        if !keyboard.just_pressed(KeyCode::KeyF) {
            return;
        }
        
        let player_transform = match player_query.single() {
            Ok(transform) => transform,
            Err(_) => return,
        };
        
        let player_pos = Vec2::new(player_transform.translation.x, player_transform.translation.y);
        
        for (entity, transform, mut production, animal, indicator) in query.iter_mut() {
            let animal_pos = Vec2::new(transform.translation.x, transform.translation.y);
            let distance = (player_pos - animal_pos).length();
            
            if distance <= 50.0 && production.can_produce {
                if let Some(item_type) = production.collect_production() {
                    let item = Item {
                        item_type,
                        quantity: 1,
                        name: Self::get_item_name(item_type),
                        category: Self::get_item_category(item_type),
                        value: Self::get_item_value(item_type),
                        description: Self::get_item_description(item_type),
                    };
                    
                    if inventory.add_item(item) {
                        println!("收集了 {} 的 {}！", Self::get_animal_name(animal.animal_type), Self::get_item_name(item_type));
                        
                        // Remove production indicator
                        if indicator.is_some() {
                            commands.entity(entity).remove::<ProductionIndicator>();
                        }
                    } else {
                        println!("物品栏已满！");
                        production.can_produce = true; // Give back the production ability
                    }
                }
            }
        }
    }
    
    pub fn update_production_indicators(
        time: Res<Time>,
        mut query: Query<(Entity, &mut ProductionIndicator)>,
        mut commands: Commands,
    ) {
        for (entity, mut indicator) in query.iter_mut() {
            indicator.timer.tick(time.delta());
            if indicator.timer.finished() {
                commands.entity(entity).remove::<ProductionIndicator>();
            }
        }
    }
    
    pub fn enhance_animal_spawning(
        mut commands: Commands,
        animal_query: Query<(Entity, &Animal)>,
        production_query: Query<Entity, With<AnimalProduction>>,
    ) {
        // Add production components to existing animals that don't have them
        let has_production_entities: HashSet<Entity> = production_query.iter().collect();
        
        for (entity, animal) in animal_query.iter() {
            if !has_production_entities.contains(&entity) {
                commands.entity(entity).insert(AnimalProduction::new(animal.animal_type));
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
    
    fn get_item_name(item_type: ItemType) -> String {
        match item_type {
            ItemType::Egg => "鸡蛋".to_string(),
            ItemType::Milk => "牛奶".to_string(),
            ItemType::Wool => "羊毛".to_string(),
            _ => "未知物品".to_string(),
        }
    }
    
    fn get_item_category(item_type: ItemType) -> ItemCategory {
        match item_type {
            ItemType::Egg | ItemType::Milk | ItemType::Wool => ItemCategory::Special,
            _ => ItemCategory::Food,
        }
    }
    
    fn get_item_value(item_type: ItemType) -> u32 {
        match item_type {
            ItemType::Egg => 8,
            ItemType::Milk => 10,
            ItemType::Wool => 20,
            _ => 1,
        }
    }
    
    fn get_item_description(item_type: ItemType) -> String {
        match item_type {
            ItemType::Egg => "新鲜的鸡蛋".to_string(),
            ItemType::Milk => "新鲜的牛奶".to_string(),
            ItemType::Wool => "柔软的羊毛".to_string(),
            _ => "普通物品".to_string(),
        }
    }
}

#[derive(Component)]
pub struct ProductionIndicator {
    pub timer: Timer,
}