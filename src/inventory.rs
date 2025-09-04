use bevy::prelude::*;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_inventory)
           .add_systems(Update, (toggle_inventory, handle_inventory_input));
    }
}

#[derive(Resource)]
pub struct Inventory {
    pub items: Vec<Item>,
    pub selected_index: usize,
    pub capacity: usize,
    pub is_open: bool,
}

#[derive(Clone, Debug)]
pub struct Item {
    pub item_type: ItemType,
    pub quantity: u32,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ItemType {
    Apple,
    Carrot,
    Bone,
    Fish,
    Milk,
    Wheat,
    Flower,
}

fn setup_inventory(mut commands: Commands) {
    let initial_items = vec![
        Item {
            item_type: ItemType::Apple,
            quantity: 5,
            name: "Apple".to_string(),
        },
        Item {
            item_type: ItemType::Carrot,
            quantity: 3,
            name: "Carrot".to_string(),
        },
    ];

    commands.insert_resource(Inventory {
        items: initial_items,
        selected_index: 0,
        capacity: 12,
        is_open: false,
    });
}

fn toggle_inventory(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut inventory: ResMut<Inventory>,
) {
    if keyboard.just_pressed(KeyCode::Tab) {
        inventory.is_open = !inventory.is_open;
    }
}

fn handle_inventory_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut inventory: ResMut<Inventory>,
) {
    if !inventory.is_open {
        return;
    }

    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        if inventory.selected_index > 0 {
            inventory.selected_index -= 1;
        }
    } else if keyboard.just_pressed(KeyCode::ArrowRight) {
        if inventory.selected_index < inventory.items.len().saturating_sub(1) {
            inventory.selected_index += 1;
        }
    }
}

impl Inventory {
    pub fn add_item(&mut self, item: Item) -> bool {
        // Check if item already exists and stack it
        for existing_item in self.items.iter_mut() {
            if existing_item.item_type == item.item_type {
                existing_item.quantity += item.quantity;
                return true;
            }
        }

        // Add new item if there's space
        if self.items.len() < self.capacity {
            self.items.push(item);
            true
        } else {
            false
        }
    }

    pub fn remove_item(&mut self, item_type: &ItemType, quantity: u32) -> bool {
        for i in 0..self.items.len() {
            if self.items[i].item_type == *item_type {
                if self.items[i].quantity >= quantity {
                    self.items[i].quantity -= quantity;
                    if self.items[i].quantity == 0 {
                        self.items.remove(i);
                        if self.selected_index >= self.items.len() && !self.items.is_empty() {
                            self.selected_index = self.items.len() - 1;
                        }
                    }
                    return true;
                }
                break;
            }
        }
        false
    }

    pub fn get_selected_item(&self) -> Option<&Item> {
        self.items.get(self.selected_index)
    }
}