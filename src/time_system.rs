use bevy::prelude::*;

pub struct TimeOfDayPlugin;

impl Plugin for TimeOfDayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_time_of_day)
           .add_systems(Update, update_time_of_day);
    }
}

#[derive(Resource)]
pub struct TimeOfDay {
    pub hour: u32,
    pub minute: u32,
    pub time_scale: f32,
    pub current_phase: DayPhase,
}

#[derive(PartialEq, Debug)]
pub enum DayPhase {
    Morning,
    Noon,
    Evening,
    Night,
}

fn setup_time_of_day(mut commands: Commands) {
    commands.insert_resource(TimeOfDay {
        hour: 8,
        minute: 0,
        time_scale: 60.0, // 1 real second = 60 game minutes
        current_phase: DayPhase::Morning,
    });
}

fn update_time_of_day(
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

impl TimeOfDay {
    pub fn get_time_string(&self) -> String {
        format!("{:02}:{:02}", self.hour, self.minute)
    }

    pub fn get_hour_f32(&self) -> f32 {
        self.hour as f32 + (self.minute as f32 / 60.0)
    }
}