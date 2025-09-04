use bevy::prelude::*;
use crate::components::*;

pub struct TimeSystems;

impl TimeSystems {
    pub fn setup_time_of_day(mut commands: Commands) {
        commands.insert_resource(TimeOfDay {
            hour: 8,
            minute: 0,
            time_scale: 60.0, // 1 real second = 60 game minutes
            current_phase: DayPhase::Morning,
        });
    }

    pub fn update_time_of_day(
        time: Res<Time>,
        mut time_of_day: ResMut<TimeOfDay>,
    ) {
        // Update time
        let delta_minutes = time.delta_secs() * time_of_day.time_scale;
        time_of_day.minute += delta_minutes as u32;

        if time_of_day.minute >= 60 {
            time_of_day.hour += time_of_day.minute / 60;
            time_of_day.minute %= 60;

            if time_of_day.hour >= 24 {
                time_of_day.hour %= 24;
            }
        }

        // Update day phase
        time_of_day.current_phase = match time_of_day.hour {
            6..=11 => DayPhase::Morning,
            12..=16 => DayPhase::Noon,
            17..=20 => DayPhase::Evening,
            _ => DayPhase::Night,
        };
    }

    pub fn apply_time_visual_effects(
        time_of_day: Res<TimeOfDay>,
        mut clear_color: ResMut<ClearColor>,
        mut ambient_light: ResMut<AmbientLight>,
    ) {
        let (bg_color, light_intensity, light_color) = match time_of_day.current_phase {
            DayPhase::Morning => (
                Color::srgb(0.8, 0.9, 1.0), // Light blue
                0.8,
                Color::srgb(1.0, 0.9, 0.7),
            ),
            DayPhase::Noon => (
                Color::srgb(0.5, 0.8, 1.0), // Bright blue
                1.0,
                Color::srgb(1.0, 1.0, 0.9),
            ),
            DayPhase::Evening => (
                Color::srgb(1.0, 0.6, 0.4), // Orange
                0.6,
                Color::srgb(1.0, 0.7, 0.4),
            ),
            DayPhase::Night => (
                Color::srgb(0.1, 0.1, 0.3), // Dark blue
                0.3,
                Color::srgb(0.6, 0.6, 0.9),
            ),
        };

        *clear_color = ClearColor(bg_color);
        ambient_light.brightness = light_intensity;
        ambient_light.color = light_color;
    }

    pub fn update_animal_behavior_by_time(
        time_of_day: Res<TimeOfDay>,
        mut query: Query<&mut AnimalAI>,
    ) {
        for mut ai in query.iter_mut() {
            match time_of_day.current_phase {
                DayPhase::Morning => {
                    // Animals are more active in the morning
                    if ai.state == AnimalState::Idle && ai.idle_timer.finished() {
                        ai.idle_timer.set_duration(std::time::Duration::from_secs_f32(2.0));
                    }
                }
                DayPhase::Noon => {
                    // Animals rest during noon
                    if ai.state == AnimalState::Wandering {
                        ai.state = AnimalState::Idle;
                    }
                }
                DayPhase::Evening => {
                    // Animals become active again
                    if ai.state == AnimalState::Idle && ai.idle_timer.finished() {
                        ai.state = AnimalState::Wandering;
                    }
                }
                DayPhase::Night => {
                    // Animals sleep at night
                    if ai.state != AnimalState::Idle {
                        ai.state = AnimalState::Idle;
                    }
                }
            }
        }
    }

    pub fn display_time_info(time_of_day: Res<TimeOfDay>) {
        // Log time information every minute
        if time_of_day.minute % 10 == 0 {
            let time_str = time_of_day.get_time_string();
            let phase_str = match time_of_day.current_phase {
                DayPhase::Morning => "早晨",
                DayPhase::Noon => "中午",
                DayPhase::Evening => "傍晚",
                DayPhase::Night => "夜晚",
            };
            println!("游戏时间: {} ({})", time_str, phase_str);
        }
    }
}

impl TimeOfDay {
    pub fn get_time_string(&self) -> String {
        format!("{:02}:{:02}", self.hour, self.minute)
    }

    pub fn get_hour_f32(&self) -> f32 {
        self.hour as f32 + (self.minute as f32 / 60.0)
    }
}

pub fn update_weather_system(time: Res<Time>, mut weather: ResMut<Weather>) {
    weather.update(time.delta());
}

pub fn setup_weather(mut commands: Commands) {
    commands.insert_resource(Weather::new());
}