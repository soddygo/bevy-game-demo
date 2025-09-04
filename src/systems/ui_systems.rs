use bevy::prelude::*;
use bevy::ecs::system::ParamSet;
use crate::components::*;

pub struct UISystems;

impl UISystems {
    pub fn setup_ui(mut commands: Commands) {
        // UI camera - removed to avoid conflicts with main camera
        // The main camera from camera.rs will handle both 2D and UI rendering

        // Root node filling the whole screen
        commands
            .spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexEnd,
                ..default()
            })
            .with_children(|parent| {
                // Bottom status bar container
                parent
                    .spawn(Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(120.0),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceBetween,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    })
                    .with_children(|parent| {
                        // Left player status
                        parent
                            .spawn(Node {
                                width: Val::Px(200.0),
                                height: Val::Px(100.0),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Start,
                                justify_content: JustifyContent::Center,
                                row_gap: Val::Px(5.0),
                                ..default()
                            })
                            .with_children(|parent| {
                                // Hunger label
                                parent.spawn((
                                    Text::new("饥饿值:"),
                                    TextFont {
                                        font_size: 16.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(1.0, 1.0, 1.0)),
                                    PlayerStatusText { status_type: StatusType::Hunger },
                                ));
                                
                                // Hunger progress bar background
                                parent.spawn(Node {
                                    width: Val::Px(180.0),
                                    height: Val::Px(20.0),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    // Hunger progress bar
                                    parent.spawn((
                                        Node {
                                            width: Val::Px(180.0),
                                            height: Val::Px(20.0),
                                            ..default()
                                        },
                                        BackgroundColor(Color::srgb(1.0, 0.0, 0.0)),
                                        HungerBar,
                                    ));
                                });
                                
                                // Energy label
                                parent.spawn((
                                    Text::new("能量值:"),
                                    TextFont {
                                        font_size: 16.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(1.0, 1.0, 1.0)),
                                    PlayerStatusText { status_type: StatusType::Energy },
                                ));
                                
                                // Energy progress bar background
                                parent.spawn(Node {
                                    width: Val::Px(180.0),
                                    height: Val::Px(20.0),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    // Energy progress bar
                                    parent.spawn((
                                        Node {
                                            width: Val::Px(180.0),
                                            height: Val::Px(20.0),
                                            ..default()
                                        },
                                        BackgroundColor(Color::srgb(0.0, 1.0, 0.0)),
                                        EnergyBar,
                                    ));
                                });
                            });
                        
                        // Middle inventory
                        parent
                            .spawn(Node {
                                width: Val::Px(480.0),
                                height: Val::Px(100.0),
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                column_gap: Val::Px(8.0),
                                ..default()
                            })
                            .with_children(|parent| {
                                // Create 8 inventory slots
                                for i in 0..8 {
                                    parent
                                        .spawn((
                                            Node {
                                                width: Val::Px(55.0),
                                                height: Val::Px(85.0),
                                                flex_direction: FlexDirection::Column,
                                                align_items: AlignItems::Center,
                                                justify_content: JustifyContent::Center,
                                                border: UiRect::all(Val::Px(2.0)),
                                                ..default()
                                            },
                                            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                                            BorderColor(Color::srgb(0.8, 0.8, 0.8)),
                                        ))
                                        .with_children(|parent| {
                                            // Item icon container with better layout
                                            parent.spawn(Node {
                                                width: Val::Px(50.0),
                                                height: Val::Px(70.0),
                                                flex_direction: FlexDirection::Column,
                                                align_items: AlignItems::Center,
                                                justify_content: JustifyContent::SpaceBetween,
                                                row_gap: Val::Px(2.0),
                                                ..default()
                                            })
                                            .with_children(|parent| {
                                                // Item icon background
                                                parent.spawn((
                                                    Node {
                                                        width: Val::Px(40.0),
                                                        height: Val::Px(40.0),
                                                        align_items: AlignItems::Center,
                                                        justify_content: JustifyContent::Center,
                                                        border: UiRect::all(Val::Px(1.0)),
                                                        ..default()
                                                    },
                                                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                                                    BorderColor(Color::srgb(0.6, 0.6, 0.6)),
                                                    InventoryItemColor { slot_index: i },
                                                ))
                                                .with_children(|parent| {
                                                    // Item symbol
                                                    parent.spawn((
                                                        Text::new("□"),
                                                        TextFont {
                                                            font_size: 20.0,
                                                            ..default()
                                                        },
                                                        TextColor(Color::srgb(1.0, 1.0, 1.0)),
                                                        InventorySlot { slot_index: i },
                                                    ));
                                                });
                                                
                                                // Item name
                                                parent.spawn((
                                                    Text::new("物品"),
                                                    TextFont {
                                                        font_size: 10.0,
                                                        ..default()
                                                    },
                                                    TextColor(Color::srgb(1.0, 1.0, 1.0)),
                                                    InventoryItemName { slot_index: i },
                                                ));
                                                
                                                // Item count
                                                parent.spawn((
                                                    Text::new("0"),
                                                    TextFont {
                                                        font_size: 12.0,
                                                        ..default()
                                                    },
                                                    TextColor(Color::srgb(1.0, 1.0, 0.0)),
                                                    InventoryItemCount { slot_index: i },
                                                ));
                                            });
                                        });
                                }
                            });
                        
                        // Right time and weather
                        parent
                            .spawn(Node {
                                width: Val::Px(150.0),
                                height: Val::Px(100.0),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::End,
                                justify_content: JustifyContent::Center,
                                row_gap: Val::Px(5.0),
                                ..default()
                            })
                            .with_children(|parent| {
                                // Time display
                                parent.spawn((
                                    Text::new("08:00"),
                                    TextFont {
                                        font_size: 24.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(1.0, 1.0, 1.0)),
                                    TimeDisplay,
                                ));
                                
                                // Weather display
                                parent.spawn((
                                    Text::new("☀️ 晴天"),
                                    TextFont {
                                        font_size: 20.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(1.0, 1.0, 0.0)),
                                    WeatherDisplay,
                                ));
                                
                                // Day display
                                parent.spawn((
                                    Text::new("第1天"),
                                    TextFont {
                                        font_size: 16.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                                    DayDisplay,
                                ));
                            });
                    });
            });
    }
    
    pub fn update_player_status_ui(
        player_stats: Res<PlayerStats>,
        mut hunger_bar_query: Query<&mut Node, With<HungerBar>>,
        mut energy_bar_query: Query<&mut Node, (With<EnergyBar>, Without<HungerBar>)>,
    ) {
        // Update hunger progress bar
        if let Ok(mut hunger_node) = hunger_bar_query.single_mut() {
            hunger_node.width = Val::Px(player_stats.hunger * 1.8); // 100 * 1.8 = 180px
        }
        
        // Update energy progress bar
        if let Ok(mut energy_node) = energy_bar_query.single_mut() {
            energy_node.width = Val::Px(player_stats.energy * 1.8); // 100 * 1.8 = 180px
        }
    }
    
    pub fn update_inventory_ui(
        inventory: Res<Inventory>,
        mut inventory_query: ParamSet<(
            Query<(&mut Text, &InventorySlot)>,
            Query<(&mut Text, &InventoryItemCount)>,
            Query<(&mut BackgroundColor, &InventoryItemColor)>,
            Query<(&mut Text, &InventoryItemName)>,
        )>,
    ) {
        // Update inventory icons
        for (mut text, slot) in inventory_query.p0().iter_mut() {
            if let Some(item) = inventory.items.get(slot.slot_index) {
                let symbol = item.get_item_symbol();
                *text = Text::new(symbol);
            } else {
                *text = Text::new("□");
            }
        }
        
        // Update item counts
        for (mut text, count) in inventory_query.p1().iter_mut() {
            if let Some(item) = inventory.items.get(count.slot_index) {
                *text = Text::new(item.quantity.to_string());
            } else {
                *text = Text::new("0".to_string());
            }
        }
        
        // Update item colors
        for (mut bg_color, color_slot) in inventory_query.p2().iter_mut() {
            if let Some(item) = inventory.items.get(color_slot.slot_index) {
                bg_color.0 = item.get_item_color();
            } else {
                bg_color.0 = Color::srgb(0.2, 0.2, 0.2); // Default dark gray
            }
        }
        
        // Update item names
        for (mut text, name_slot) in inventory_query.p3().iter_mut() {
            if let Some(item) = inventory.items.get(name_slot.slot_index) {
                let name = item.get_item_name();
                *text = Text::new(name);
            } else {
                *text = Text::new("Empty");
            }
        }
    }
    
    pub fn update_time_weather_ui(
        time_of_day: Res<TimeOfDay>,
        weather: Res<Weather>,
        mut text_queries: ParamSet<(
            Query<&mut Text, With<TimeDisplay>>,
            Query<&mut Text, With<WeatherDisplay>>,
            Query<&mut Text, With<DayDisplay>>,
        )>,
    ) {
        // Update time display
        if let Ok(mut time_text) = text_queries.p0().single_mut() {
            *time_text = Text::new(format!("{:02}:{:02}", time_of_day.hour, time_of_day.minute));
        }
        
        // Update weather display
        if let Ok(mut weather_text) = text_queries.p1().single_mut() {
            let weather_icon = match weather.current_weather {
                WeatherType::Sunny => "☀️",
                WeatherType::Cloudy => "☁️",
                WeatherType::Rainy => "🌧️",
                WeatherType::Snowy => "❄️",
            };
            *weather_text = Text::new(format!("{} {}", weather_icon, weather.get_weather_name()));
        }
        
        // Update day display
        if let Ok(mut day_text) = text_queries.p2().single_mut() {
            *day_text = Text::new(format!("第{}天", time_of_day.hour / 24 + 1));
        }
    }
    
    pub fn handle_inventory_input(
        keyboard: Res<ButtonInput<KeyCode>>,
        mut inventory: ResMut<Inventory>,
        mut inventory_slot_query: Query<(&InventorySlot, &mut BackgroundColor)>,
    ) {
        // Number keys 1-8 to select inventory slots
        for i in 1..=8 {
            if keyboard.just_pressed(KeyCode::Digit1) && i == 1 {
                inventory.selected_index = 0;
                highlight_selected_slot(&mut inventory_slot_query, 0);
            } else if keyboard.just_pressed(KeyCode::Digit2) && i == 2 {
                inventory.selected_index = 1;
                highlight_selected_slot(&mut inventory_slot_query, 1);
            } else if keyboard.just_pressed(KeyCode::Digit3) && i == 3 {
                inventory.selected_index = 2;
                highlight_selected_slot(&mut inventory_slot_query, 2);
            } else if keyboard.just_pressed(KeyCode::Digit4) && i == 4 {
                inventory.selected_index = 3;
                highlight_selected_slot(&mut inventory_slot_query, 3);
            } else if keyboard.just_pressed(KeyCode::Digit5) && i == 5 {
                inventory.selected_index = 4;
                highlight_selected_slot(&mut inventory_slot_query, 4);
            } else if keyboard.just_pressed(KeyCode::Digit6) && i == 6 {
                inventory.selected_index = 5;
                highlight_selected_slot(&mut inventory_slot_query, 5);
            } else if keyboard.just_pressed(KeyCode::Digit7) && i == 7 {
                inventory.selected_index = 6;
                highlight_selected_slot(&mut inventory_slot_query, 6);
            } else if keyboard.just_pressed(KeyCode::Digit8) && i == 8 {
                inventory.selected_index = 7;
                highlight_selected_slot(&mut inventory_slot_query, 7);
            }
        }
    }
}

fn highlight_selected_slot(
    inventory_slot_query: &mut Query<(&InventorySlot, &mut BackgroundColor)>,
    selected_index: usize,
) {
    for (slot, mut background_color) in inventory_slot_query.iter_mut() {
        if slot.slot_index == selected_index {
            background_color.0 = Color::srgb(0.5, 0.5, 0.8); // Highlight color when selected
        } else {
            background_color.0 = Color::srgb(0.2, 0.2, 0.2); // Default color
        }
    }
}

// UI component markers
#[derive(Component)]
pub struct PlayerStatusText {
    pub status_type: StatusType,
}

#[derive(Component)]
pub enum StatusType {
    Hunger,
    Energy,
}

#[derive(Component)]
pub struct HungerBar;

#[derive(Component)]
pub struct EnergyBar;

#[derive(Component)]
pub struct InventorySlot {
    pub slot_index: usize,
}

#[derive(Component)]
pub struct InventoryItemCount {
    pub slot_index: usize,
}

#[derive(Component)]
pub struct InventoryItemColor {
    pub slot_index: usize,
}

#[derive(Component)]
pub struct InventoryItemName {
    pub slot_index: usize,
}

#[derive(Component)]
pub struct TimeDisplay;

#[derive(Component)]
pub struct WeatherDisplay;

#[derive(Component)]
pub struct DayDisplay;