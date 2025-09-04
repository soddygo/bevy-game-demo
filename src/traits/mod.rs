use bevy::prelude::*;
use crate::components::*;

pub trait AnimalBehavior {
    fn get_favorite_food(&self) -> ItemType;
    fn get_personality(&self) -> &str;
    fn get_base_speed(&self) -> f32;
    fn can_follow_player(&self, friendship_level: u32) -> bool;
}

pub trait ItemUsable {
    fn can_use_on_animal(&self, animal_type: AnimalType) -> bool;
    fn get_friendship_bonus(&self) -> u32;
    fn get_hunger_reduction(&self) -> f32;
}

pub trait InteractableEntity {
    fn can_interact(&self, player_position: Vec2, interaction_range: f32) -> bool;
    fn get_interaction_prompt(&self) -> String;
    fn interact(&mut self, interaction_type: InteractionType);
}

// Implementation for Animal
impl AnimalBehavior for Animal {
    fn get_favorite_food(&self) -> ItemType {
        match self.animal_type {
            AnimalType::Dog => ItemType::Bone,
            AnimalType::Cat => ItemType::Fish,
            AnimalType::Chicken => ItemType::Wheat,
            AnimalType::Cow => ItemType::Carrot,
            AnimalType::Sheep => ItemType::Apple,
            AnimalType::Pig => ItemType::Carrot,
            AnimalType::Duck => ItemType::Wheat,
            AnimalType::Horse => ItemType::Apple,
        }
    }

    fn get_personality(&self) -> &str {
        match self.animal_type {
            AnimalType::Dog => "活泼忠诚",
            AnimalType::Cat => "独立优雅",
            AnimalType::Chicken => "胆小勤奋",
            AnimalType::Cow => "温顺缓慢",
            AnimalType::Sheep => "温顺可爱",
            AnimalType::Pig => "贪吃懒惰",
            AnimalType::Duck => "快乐游泳",
            AnimalType::Horse => "自由奔放",
        }
    }

    fn get_base_speed(&self) -> f32 {
        self.speed
    }

    fn can_follow_player(&self, friendship_level: u32) -> bool {
        match self.animal_type {
            AnimalType::Dog => friendship_level > 2,
            AnimalType::Cat => friendship_level > 5,
            AnimalType::Chicken => friendship_level > 3,
            AnimalType::Cow => friendship_level > 1,
            AnimalType::Sheep => friendship_level > 4,
            AnimalType::Pig => friendship_level > 8,
            AnimalType::Duck => friendship_level > 2,
            AnimalType::Horse => friendship_level > 6,
        }
    }
}

// Implementation for Item
impl ItemUsable for Item {
    fn can_use_on_animal(&self, animal_type: AnimalType) -> bool {
        match (self.item_type, animal_type) {
            // Dog preferences
            (ItemType::Apple, AnimalType::Dog) => true,
            (ItemType::Bone, AnimalType::Dog) => true,
            (ItemType::Toy, AnimalType::Dog) => true,
            
            // Cat preferences
            (ItemType::Fish, AnimalType::Cat) => true,
            (ItemType::Milk, AnimalType::Cat) => true,
            (ItemType::Bell, AnimalType::Cat) => true,
            
            // Chicken preferences
            (ItemType::Wheat, AnimalType::Chicken) => true,
            (ItemType::Corn, AnimalType::Chicken) => true,
            (ItemType::Carrot, AnimalType::Chicken) => true,
            
            // Cow preferences
            (ItemType::Carrot, AnimalType::Cow) => true,
            (ItemType::Apple, AnimalType::Cow) => true,
            (ItemType::Hay, AnimalType::Cow) => true,
            
            // Sheep preferences
            (ItemType::Apple, AnimalType::Sheep) => true,
            (ItemType::Carrot, AnimalType::Sheep) => true,
            (ItemType::Hay, AnimalType::Sheep) => true,
            
            // Pig preferences
            (ItemType::Carrot, AnimalType::Pig) => true,
            (ItemType::Apple, AnimalType::Pig) => true,
            (ItemType::Corn, AnimalType::Pig) => true,
            
            // Duck preferences
            (ItemType::Wheat, AnimalType::Duck) => true,
            (ItemType::Corn, AnimalType::Duck) => true,
            (ItemType::Fish, AnimalType::Duck) => true,
            
            // Horse preferences
            (ItemType::Apple, AnimalType::Horse) => true,
            (ItemType::Carrot, AnimalType::Horse) => true,
            (ItemType::Hay, AnimalType::Horse) => true,
            
            // Decorative items for all animals
            (ItemType::Flower, _) => true,
            (ItemType::Ribbon, _) => true,
            (ItemType::Toy, _) => true,
            
            _ => false,
        }
    }

    fn get_friendship_bonus(&self) -> u32 {
        match self.item_type {
            ItemType::Bone => 3,
            ItemType::Fish => 2,
            ItemType::Apple => 1,
            ItemType::Milk => 2,
            ItemType::Carrot => 1,
            ItemType::Wheat => 1,
            ItemType::Flower => 1,
            ItemType::Corn => 1,
            ItemType::Hay => 1,
            ItemType::Toy => 2,
            ItemType::Ribbon => 1,
            ItemType::Bell => 1,
            _ => 0,
        }
    }

    fn get_hunger_reduction(&self) -> f32 {
        match self.item_type {
            ItemType::Bone => 20.0,
            ItemType::Fish => 15.0,
            ItemType::Apple => 10.0,
            ItemType::Milk => 12.0,
            ItemType::Carrot => 8.0,
            ItemType::Wheat => 5.0,
            ItemType::Corn => 7.0,
            ItemType::Hay => 15.0,
            ItemType::ApplePie => 20.0,
            ItemType::MilkBottle => 10.0,
            _ => 0.0,
        }
    }
}

// Implementation for Interactable component
impl InteractableEntity for Interactable {
    fn can_interact(&self, player_position: Vec2, interaction_range: f32) -> bool {
        // This would need the entity's position, which should be passed in
        // For now, return true as a placeholder
        true
    }

    fn get_interaction_prompt(&self) -> String {
        match self.interaction_type {
            InteractionType::Pet => "抚摸".to_string(),
            InteractionType::Feed => "喂食".to_string(),
            InteractionType::Play => "玩耍".to_string(),
        }
    }

    fn interact(&mut self, interaction_type: InteractionType) {
        // Handle interaction logic
        println!("Interaction: {:?}", interaction_type);
    }
}