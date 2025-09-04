use bevy::prelude::*;
use crate::components::*;
use crate::traits::*;

pub struct InventorySystems;

impl InventorySystems {
    pub fn setup_inventory(mut commands: Commands) {
        let initial_items = vec![
            Item {
                item_type: ItemType::Apple,
                quantity: 5,
                name: "苹果".to_string(),
                category: ItemCategory::Food,
                value: 5,
                description: "新鲜的红苹果".to_string(),
            },
            Item {
                item_type: ItemType::Carrot,
                quantity: 3,
                name: "胡萝卜".to_string(),
                category: ItemCategory::Food,
                value: 3,
                description: "脆嫩的胡萝卜".to_string(),
            },
            Item {
                item_type: ItemType::Bone,
                quantity: 2,
                name: "骨头".to_string(),
                category: ItemCategory::Food,
                value: 8,
                description: "小狗最爱的骨头".to_string(),
            },
            Item {
                item_type: ItemType::Fish,
                quantity: 4,
                name: "鱼".to_string(),
                category: ItemCategory::Food,
                value: 6,
                description: "新鲜的鱼".to_string(),
            },
            Item {
                item_type: ItemType::Milk,
                quantity: 2,
                name: "牛奶".to_string(),
                category: ItemCategory::Food,
                value: 4,
                description: "新鲜的牛奶".to_string(),
            },
            Item {
                item_type: ItemType::Flower,
                quantity: 6,
                name: "花".to_string(),
                category: ItemCategory::Decoration,
                value: 3,
                description: "美丽的花".to_string(),
            },
            Item {
                item_type: ItemType::Shovel,
                quantity: 1,
                name: "铲子".to_string(),
                category: ItemCategory::Tool,
                value: 15,
                description: "园艺铲子".to_string(),
            },
            Item {
                item_type: ItemType::Egg,
                quantity: 3,
                name: "鸡蛋".to_string(),
                category: ItemCategory::Special,
                value: 4,
                description: "新鲜的鸡蛋".to_string(),
            },
        ];

        commands.insert_resource(Inventory {
            items: initial_items,
            selected_index: 0,
            capacity: 12,
            is_open: false,
        });
    }

    pub fn toggle_inventory(
        keyboard: Res<ButtonInput<KeyCode>>,
        mut inventory: ResMut<Inventory>,
    ) {
        if keyboard.just_pressed(KeyCode::Tab) {
            inventory.is_open = !inventory.is_open;
            if inventory.is_open {
                println!("打开物品栏 - 当前物品: {}", inventory.items.len());
                for (i, item) in inventory.items.iter().enumerate() {
                    let selected = if i == inventory.selected_index { " [选中]" } else { "" };
                    println!("  {}. {}x{}{}", i + 1, item.name, item.quantity, selected);
                }
            } else {
                println!("关闭物品栏");
            }
        }
    }

    pub fn handle_inventory_input(
        keyboard: Res<ButtonInput<KeyCode>>,
        mut inventory: ResMut<Inventory>,
    ) {
        if !inventory.is_open {
            return;
        }

        let mut changed = false;
        
        if keyboard.just_pressed(KeyCode::ArrowLeft) {
            if inventory.selected_index > 0 {
                inventory.selected_index -= 1;
                changed = true;
            }
        } else if keyboard.just_pressed(KeyCode::ArrowRight) {
            if inventory.selected_index < inventory.items.len().saturating_sub(1) {
                inventory.selected_index += 1;
                changed = true;
            }
        } else if keyboard.just_pressed(KeyCode::Digit1) {
            if inventory.items.len() > 0 {
                inventory.selected_index = 0;
                changed = true;
            }
        } else if keyboard.just_pressed(KeyCode::Digit2) {
            if inventory.items.len() > 1 {
                inventory.selected_index = 1;
                changed = true;
            }
        } else if keyboard.just_pressed(KeyCode::Digit3) {
            if inventory.items.len() > 2 {
                inventory.selected_index = 2;
                changed = true;
            }
        }

        if changed {
            if let Some(item) = inventory.get_selected_item() {
                println!("选中: {}x{}", item.name, item.quantity);
            }
        }
    }

    pub fn use_selected_item(
        keyboard: Res<ButtonInput<KeyCode>>,
        mut inventory: ResMut<Inventory>,
        mut animal_query: Query<(Entity, &Transform, &mut Animal, &mut Interactable)>,
        player_query: Query<&Transform, (With<Player>, Without<Animal>)>,
    ) {
        if !keyboard.just_pressed(KeyCode::KeyE) {
            return;
        }

        let player_transform = match player_query.single() {
            Ok(transform) => transform,
            Err(_) => return,
        };

        let player_pos = Vec2::new(player_transform.translation.x, player_transform.translation.y);

        let selected_item = inventory.get_selected_item().cloned();
        
        if let Some(item) = selected_item {
            if item.quantity <= 0 {
                println!("物品数量不足");
                return;
            }

            // Find nearby animals
            for (entity, transform, mut animal, mut interactable) in animal_query.iter_mut() {
                let animal_pos = Vec2::new(transform.translation.x, transform.translation.y);
                let distance = (player_pos - animal_pos).length();

                if distance <= interactable.interaction_range {
                    if item.can_use_on_animal(animal.animal_type) {
                        // Use the item
                        let friendship_bonus = item.get_friendship_bonus();
                        let hunger_reduction = item.get_hunger_reduction();
                        
                        animal.friendship_level += friendship_bonus;
                        animal.hunger -= hunger_reduction;
                        animal.hunger = animal.hunger.max(0.0);

                        // Remove item from inventory
                        inventory.remove_item(&item.item_type, 1);

                        let favorite_food = animal.get_favorite_food();
                        if item.item_type == favorite_food {
                            println!("{}最喜欢吃{}! 好感度+{}, 饥饿度-{}", 
                                Self::get_animal_name(animal.animal_type),
                                item.name,
                                friendship_bonus,
                                hunger_reduction);
                        } else {
                            println!("{}吃了{}! 好感度+{}, 饥饿度-{}", 
                                Self::get_animal_name(animal.animal_type),
                                item.name,
                                friendship_bonus,
                                hunger_reduction);
                        }
                        break;
                    } else {
                        println!("{}不喜欢吃{}", 
                            Self::get_animal_name(animal.animal_type),
                            item.name);
                    }
                }
            }
        }
    }

    pub fn add_random_items(
        time: Res<Time>,
        mut inventory: ResMut<Inventory>,
    ) {
        // This could be called periodically or through events
        // For now, we'll add a simple timer-based item addition
        // In a real game, items would be acquired through gameplay
    }

    pub fn show_inventory_status(inventory: Res<Inventory>) {
        // Show inventory status periodically
        // This could be replaced with a proper UI system
        if inventory.is_open {
            if let Some(item) = inventory.get_selected_item() {
                println!("当前选中: {}x{} (按E使用)", item.name, item.quantity);
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
}