use bevy::prelude::*;
use crate::components::*;

pub struct PlayerSystems;

impl PlayerSystems {
    pub fn setup_player_stats(mut commands: Commands) {
        commands.insert_resource(PlayerStats::new());
    }
    
    pub fn update_player_stats(
        time: Res<Time>,
        mut player_stats: ResMut<PlayerStats>,
        keyboard: Res<ButtonInput<KeyCode>>,
    ) {
        // Update hunger over time (decreases slowly)
        player_stats.update_hunger(-0.5 * time.delta_secs());
        
        // Update fatigue based on movement
        if keyboard.any_pressed([KeyCode::KeyW, KeyCode::KeyA, KeyCode::KeyS, KeyCode::KeyD]) {
            player_stats.update_fatigue(2.0 * time.delta_secs());
            player_stats.update_energy(-1.0 * time.delta_secs());
        } else {
            // Rest when not moving
            player_stats.update_fatigue(-1.0 * time.delta_secs());
            player_stats.update_energy(0.5 * time.delta_secs());
        }
        
        // Update fatigue based on time of day
        // This will be connected to the time system later
        
        // Check if player is exhausted
        if player_stats.is_exhausted() {
            println!("玩家状态不佳 - 需要休息或进食！");
        }
    }
    
    pub fn handle_player_eating(
        keyboard: Res<ButtonInput<KeyCode>>,
        mut player_stats: ResMut<PlayerStats>,
        mut inventory: ResMut<Inventory>,
    ) {
        if keyboard.just_pressed(KeyCode::KeyE) {
            let selected_item = inventory.get_selected_item().cloned();
            
            if let Some(item) = selected_item {
                if item.category == ItemCategory::Food {
                    let hunger_restored = match item.item_type {
                        ItemType::Apple => 15.0,
                        ItemType::Carrot => 10.0,
                        ItemType::ApplePie => 25.0,
                        ItemType::MilkBottle => 12.0,
                        _ => 5.0,
                    };
                    
                    player_stats.update_hunger(hunger_restored);
                    player_stats.update_energy(10.0);
                    
                    inventory.remove_item(&item.item_type, 1);
                    println!("食用了 {} - 饥饿度+{}", item.name, hunger_restored);
                }
            }
        }
    }
    
    pub fn handle_player_sleeping(
        keyboard: Res<ButtonInput<KeyCode>>,
        mut player_stats: ResMut<PlayerStats>,
        mut time_of_day: ResMut<TimeOfDay>,
    ) {
        if keyboard.just_pressed(KeyCode::KeyB) {
            // Sleep only at night
            if time_of_day.current_phase == DayPhase::Night {
                println!("玩家睡觉了...");
                
                // Restore stats
                player_stats.hunger = 100.0;
                player_stats.fatigue = 0.0;
                player_stats.energy = 100.0;
                
                // Advance to morning
                time_of_day.hour = 6;
                time_of_day.minute = 0;
                time_of_day.current_phase = DayPhase::Morning;
                
                player_stats.day_survived += 1;
                println!("新的一天开始了！第 {} 天", player_stats.day_survived);
            } else {
                println!("只能晚上睡觉！");
            }
        }
    }
    
    pub fn display_player_stats(player_stats: Res<PlayerStats>) {
        // Display stats every 30 seconds
        if player_stats.day_survived % 30 == 0 {
            println!("玩家状态 - 饥饿度: {:.1}, 疲劳度: {:.1}, 精力: {:.1}, 金币: {}", 
                player_stats.hunger, player_stats.fatigue, player_stats.energy, player_stats.coins);
        }
    }
}