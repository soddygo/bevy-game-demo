use bevy::prelude::*;

mod components;
mod traits;
mod systems;
mod camera;

use systems::*;
use camera::CameraPlugin;
use systems::player_systems as player_base;
use systems::player_stats_systems as player_stats;
use systems::quest_systems as quests;
use systems::animal_production_systems as animal_production;
use systems::visual_feedback_systems as visual_feedback;
use systems::ui_systems as ui;
use systems::animal_animation_systems as animal_animation;
use components::Weather;

// Plugin structures
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
                player_base::PlayerSystems::spawn_player,
                player_stats::PlayerSystems::setup_player_stats,
            ))
           .add_systems(Update, (
                player_base::PlayerSystems::player_movement,
                player_base::PlayerSystems::animate_player,
                player_stats::PlayerSystems::update_player_stats,
                player_stats::PlayerSystems::handle_player_eating,
                player_stats::PlayerSystems::handle_player_sleeping,
                player_stats::PlayerSystems::display_player_stats,
            ));
    }
}

pub struct AnimalsPlugin;

impl Plugin for AnimalsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, animal_systems::AnimalSystems::spawn_animals)
           .add_systems(Update, (
                animal_systems::AnimalSystems::animal_movement,
                animal_systems::AnimalSystems::animal_idle_behavior,
                animal_systems::AnimalSystems::keep_animals_in_bounds,
                animal_systems::AnimalSystems::debug_animal_behavior,
                animal_animation::AnimalAnimationSystems::update_animal_animations,
                animal_animation::update_feeding_effects,
                animal_animation::update_heart_effects,
                animal_production::AnimalProductionSystems::update_animal_production,
                animal_production::AnimalProductionSystems::handle_production_collection,
                animal_production::AnimalProductionSystems::update_production_indicators,
                animal_production::AnimalProductionSystems::enhance_animal_spawning,
            ));
    }
}

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
                InteractionSystems::handle_player_interactions,
                InteractionSystems::show_interaction_prompts,
                InteractionSystems::update_animal_behavior,
            ));
    }
}

pub struct TimeOfDayPlugin;

impl Plugin for TimeOfDayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
                TimeSystems::setup_time_of_day,
                quests::QuestSystems::setup_quest_manager,
                setup_weather,
            ))
           .add_systems(Update, (
                TimeSystems::update_time_of_day,
                TimeSystems::apply_time_visual_effects,
                TimeSystems::update_animal_behavior_by_time,
                TimeSystems::display_time_info,
                update_weather_system,
                quests::QuestSystems::initialize_daily_quests,
                quests::QuestSystems::track_feeding_quests,
                quests::QuestSystems::track_interaction_quests,
                quests::QuestSystems::check_quest_completion,
                quests::QuestSystems::display_quests,
            ));
    }
}

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, InventorySystems::setup_inventory)
           .add_systems(Update, (
                InventorySystems::toggle_inventory,
                InventorySystems::handle_inventory_input,
                InventorySystems::use_selected_item,
                InventorySystems::show_inventory_status,
            ));
    }
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, WorldSystems::spawn_world);
    }
}

pub struct VisualFeedbackPlugin;

impl Plugin for VisualFeedbackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            visual_feedback::VisualFeedbackSystems::update_interaction_effects,
            visual_feedback::VisualFeedbackSystems::update_floating_text,
        ));
    }
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, ui::UISystems::setup_ui)
           .add_systems(Update, (
                ui::UISystems::update_player_status_ui,
                ui::UISystems::update_inventory_ui,
                ui::UISystems::update_time_weather_ui,
                ui::UISystems::handle_inventory_input,
            ));
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.5, 0.8, 1.0)))
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(AnimalsPlugin)
        .add_plugins(InteractionPlugin)
        .add_plugins(TimeOfDayPlugin)
        .add_plugins(InventoryPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(VisualFeedbackPlugin)
        .add_plugins(UIPlugin)
        .run();
}