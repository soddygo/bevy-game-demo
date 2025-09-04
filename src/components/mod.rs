use bevy::prelude::*;

// Player components
#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub run_speed: f32,
}

#[derive(Component)]
pub struct PlayerDirection {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct PlayerAnimation {
    pub frame_timer: Timer,
    pub current_frame: usize,
    pub is_walking: bool,
    pub facing_left: bool,
}

#[derive(Component)]
pub struct PlayerPart;

#[derive(Resource)]
pub struct PlayerStats {
    pub hunger: f32,
    pub fatigue: f32,
    pub energy: f32,
    pub coins: u32,
    pub day_survived: u32,
}

impl PlayerStats {
    pub fn new() -> Self {
        Self {
            hunger: 100.0,
            fatigue: 0.0,
            energy: 100.0,
            coins: 50,
            day_survived: 1,
        }
    }
    
    pub fn update_hunger(&mut self, delta: f32) {
        self.hunger = (self.hunger + delta).clamp(0.0, 100.0);
    }
    
    pub fn update_fatigue(&mut self, delta: f32) {
        self.fatigue = (self.fatigue + delta).clamp(0.0, 100.0);
    }
    
    pub fn update_energy(&mut self, delta: f32) {
        self.energy = (self.energy + delta).clamp(0.0, 100.0);
    }
    
    pub fn is_exhausted(&self) -> bool {
        self.hunger <= 0.0 || self.fatigue >= 100.0 || self.energy <= 0.0
    }
}

// Animal components
#[derive(Component)]
pub struct Animal {
    pub animal_type: AnimalType,
    pub speed: f32,
    pub friendship_level: u32,
    pub hunger: f32,
}

#[derive(Component)]
pub struct AnimalPart;

#[derive(Component)]
pub struct AnimalAnimation {
    pub frame_timer: Timer,
    pub current_frame: usize,
    pub is_eating: bool,
    pub is_happy: bool,
    pub is_sleeping: bool,
    pub animation_timer: Timer,
    pub facing_left: bool,
}

#[derive(Component)]
pub struct AnimalAI {
    pub state: AnimalState,
    pub target_position: Vec2,
    pub idle_timer: Timer,
}

#[derive(Component)]
pub struct AnimalProduction {
    pub production_type: ProductionType,
    pub production_timer: Timer,
    pub can_produce: bool,
    pub last_production_time: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProductionType {
    Eggs,
    Milk,
    Wool,
    None,
}

impl AnimalProduction {
    pub fn new(animal_type: AnimalType) -> Self {
        let production_type = match animal_type {
            AnimalType::Chicken => ProductionType::Eggs,
            AnimalType::Cow => ProductionType::Milk,
            AnimalType::Sheep => ProductionType::Wool,
            _ => ProductionType::None,
        };
        
        let production_time = match production_type {
            ProductionType::Eggs => std::time::Duration::from_secs(300), // 5 minutes
            ProductionType::Milk => std::time::Duration::from_secs(600),  // 10 minutes
            ProductionType::Wool => std::time::Duration::from_secs(1800), // 30 minutes
            ProductionType::None => std::time::Duration::from_secs(u64::MAX),
        };
        
        Self {
            production_type,
            production_timer: Timer::new(production_time, TimerMode::Repeating),
            can_produce: false,
            last_production_time: 0,
        }
    }
    
    pub fn update(&mut self, delta: std::time::Duration) -> bool {
        self.production_timer.tick(delta);
        if self.production_timer.just_finished() {
            self.can_produce = true;
            return true;
        }
        false
    }
    
    pub fn collect_production(&mut self) -> Option<ItemType> {
        if self.can_produce {
            self.can_produce = false;
            self.production_timer.reset();
            match self.production_type {
                ProductionType::Eggs => Some(ItemType::Egg),
                ProductionType::Milk => Some(ItemType::Milk),
                ProductionType::Wool => Some(ItemType::Wool),
                ProductionType::None => None,
            }
        } else {
            None
        }
    }
}

#[derive(Component)]
pub struct Interactable {
    pub interaction_type: InteractionType,
    pub interaction_range: f32,
}

// Animal types and states
#[derive(PartialEq, Clone, Copy, Debug, Eq, Hash)]
pub enum AnimalType {
    Dog,
    Cat,
    Chicken,
    Cow,
    Sheep,
    Pig,
    Duck,
    Horse,
}

#[derive(PartialEq, Debug, Eq, Hash, Clone, Copy)]
pub enum AnimalState {
    Idle,
    Wandering,
    Following,
    Eating,
}

#[derive(PartialEq, Debug)]
pub enum InteractionType {
    Pet,
    Feed,
    Play,
}

// Time system components
#[derive(Resource)]
pub struct TimeOfDay {
    pub hour: u32,
    pub minute: u32,
    pub time_scale: f32,
    pub current_phase: DayPhase,
}

#[derive(Resource)]
pub struct Weather {
    pub current_weather: WeatherType,
    pub temperature: f32,
    pub change_timer: Timer,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WeatherType {
    Sunny,
    Cloudy,
    Rainy,
    Snowy,
}

impl Weather {
    pub fn new() -> Self {
        Self {
            current_weather: WeatherType::Sunny,
            temperature: 20.0,
            change_timer: Timer::from_seconds(300.0, TimerMode::Repeating), // Change weather every 5 minutes
        }
    }
    
    pub fn update(&mut self, delta: std::time::Duration) {
        self.change_timer.tick(delta);
        if self.change_timer.just_finished() {
            self.change_weather();
        }
    }
    
    fn change_weather(&mut self) {
        use WeatherType::*;
        let weather_types = [Sunny, Cloudy, Rainy];
        self.current_weather = weather_types[fastrand::usize(..) % weather_types.len()];
        
        // Adjust temperature based on weather
        match self.current_weather {
            Sunny => self.temperature = 20.0 + fastrand::f32() * 10.0,
            Cloudy => self.temperature = 15.0 + fastrand::f32() * 10.0,
            Rainy => self.temperature = 10.0 + fastrand::f32() * 10.0,
            Snowy => self.temperature = -5.0 + fastrand::f32() * 10.0,
        }
    }
    
    pub fn get_weather_name(&self) -> &'static str {
        match self.current_weather {
            WeatherType::Sunny => "晴天",
            WeatherType::Cloudy => "多云",
            WeatherType::Rainy => "雨天",
            WeatherType::Snowy => "雪天",
        }
    }
}

// Quest system components
#[derive(Resource)]
pub struct QuestManager {
    pub active_quests: Vec<Quest>,
    pub completed_quests: Vec<Quest>,
    pub daily_quests: Vec<Quest>,
}

impl QuestManager {
    pub fn new() -> Self {
        Self {
            active_quests: Vec::new(),
            completed_quests: Vec::new(),
            daily_quests: Vec::new(),
        }
    }
    
    pub fn add_quest(&mut self, quest: Quest) {
        self.active_quests.push(quest);
    }
    
    pub fn complete_quest(&mut self, quest_id: &str) {
        if let Some(index) = self.active_quests.iter().position(|q| q.id == quest_id) {
            let quest = self.active_quests.remove(index);
            self.completed_quests.push(quest);
        }
    }
    
    pub fn update_quest_progress(&mut self, quest_id: &str, progress: u32) {
        if let Some(quest) = self.active_quests.iter_mut().find(|q| q.id == quest_id) {
            quest.progress = progress.min(quest.required_amount);
        }
    }
}

#[derive(Clone, Debug)]
pub struct Quest {
    pub id: String,
    pub title: String,
    pub description: String,
    pub quest_type: QuestType,
    pub required_amount: u32,
    pub progress: u32,
    pub reward: QuestReward,
    pub is_daily: bool,
    pub deadline: Option<u32>, // Day number
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QuestType {
    FeedAnimals,
    CollectItems,
    FindItems,
    InteractWithAnimals,
    PlantFlowers,
    WaterPlants,
}

#[derive(Clone, Debug)]
pub struct QuestReward {
    pub coins: u32,
    pub items: Vec<(ItemType, u32)>,
    pub friendship_bonus: u32,
}

#[derive(PartialEq, Debug)]
pub enum DayPhase {
    Morning,
    Noon,
    Evening,
    Night,
}

// Inventory components
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
    pub category: ItemCategory,
    pub value: u32,
    pub description: String,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ItemCategory {
    Food,
    Tool,
    Decoration,
    Special,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ItemType {
    // 食物类
    Apple,
    Carrot,
    Bone,
    Fish,
    Milk,
    Wheat,
    Flower,
    Corn,
    Hay,
    
    // 工具类
    WaterBucket,
    Basket,
    Shovel,
    Rake,
    
    // 装饰类
    Toy,
    Ribbon,
    Bell,
    
    // 特殊物品
    Egg,
    Wool,
    ApplePie,
    MilkBottle,
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

impl Item {
    pub fn get_item_color(&self) -> Color {
        match self.item_type {
            // 食物类 - 更鲜艳、更区分的颜色
            ItemType::Apple => Color::srgb(0.9, 0.2, 0.2),      // 鲜红色
            ItemType::Carrot => Color::srgb(1.0, 0.6, 0.0),    // 亮橙色
            ItemType::Bone => Color::srgb(0.95, 0.95, 0.95),   // 纯白色
            ItemType::Fish => Color::srgb(0.0, 0.6, 1.0),      // 亮蓝色
            ItemType::Milk => Color::srgb(1.0, 1.0, 0.9),      // 纯白色
            ItemType::Wheat => Color::srgb(1.0, 0.9, 0.3),      // 金黄色
            ItemType::Flower => Color::srgb(1.0, 0.4, 1.0),     // 亮粉色
            ItemType::Corn => Color::srgb(1.0, 0.8, 0.0),       // 金黄色
            ItemType::Hay => Color::srgb(0.8, 0.6, 0.2),        // 金棕色
            
            // 工具类 - 金属色和工具色
            ItemType::WaterBucket => Color::srgb(0.3, 0.6, 0.8), // 蓝灰色
            ItemType::Basket => Color::srgb(0.6, 0.3, 0.1),     // 深棕色
            ItemType::Shovel => Color::srgb(0.6, 0.6, 0.6),     // 银色
            ItemType::Rake => Color::srgb(0.5, 0.3, 0.1),       // 木色
            
            // 装饰类 - 鲜艳的装饰色
            ItemType::Toy => Color::srgb(1.0, 0.2, 0.6),        // 粉红色
            ItemType::Ribbon => Color::srgb(0.8, 0.2, 0.8),     // 紫色
            ItemType::Bell => Color::srgb(1.0, 0.8, 0.2),       // 金色
            
            // 特殊物品 - 特殊颜色
            ItemType::Egg => Color::srgb(1.0, 1.0, 0.9),        // 蛋白色
            ItemType::Wool => Color::srgb(0.9, 0.9, 1.0),       // 浅蓝色
            ItemType::ApplePie => Color::srgb(0.8, 0.3, 0.1),   // 深红色
            ItemType::MilkBottle => Color::srgb(0.8, 0.9, 1.0), // 浅蓝色
        }
    }
    
    pub fn get_item_symbol(&self) -> &'static str {
        match self.item_type {
            // 食物类 - 使用英文字母
            ItemType::Apple => "A",      // Apple
            ItemType::Carrot => "C",     // Carrot
            ItemType::Bone => "B",       // Bone
            ItemType::Fish => "F",       // Fish
            ItemType::Milk => "M",       // Milk
            ItemType::Wheat => "W",      // Wheat
            ItemType::Flower => "L",     // Flower (L for Lily)
            ItemType::Corn => "O",       // Corn (O for O-shaped)
            ItemType::Hay => "H",        // Hay
            
            // 工具类 - 工具相关字母
            ItemType::WaterBucket => "P", // Water Bucket (P for Pail)
            ItemType::Basket => "K",     // Basket (K for Basket)
            ItemType::Shovel => "S",     // Shovel
            ItemType::Rake => "R",      // Rake
            
            // 装饰类 - 装饰相关字母
            ItemType::Toy => "T",       // Toy
            ItemType::Ribbon => "R",    // Ribbon
            ItemType::Bell => "L",       // Bell
            
            // 特殊物品 - 特殊字母
            ItemType::Egg => "E",       // Egg
            ItemType::Wool => "W",      // Wool
            ItemType::ApplePie => "P",   // Apple Pie
            ItemType::MilkBottle => "B", // Milk Bottle
        }
    }
    
    pub fn get_item_name(&self) -> &'static str {
        match self.item_type {
            ItemType::Apple => "Apple",
            ItemType::Carrot => "Carrot",
            ItemType::Bone => "Bone",
            ItemType::Fish => "Fish",
            ItemType::Milk => "Milk",
            ItemType::Wheat => "Wheat",
            ItemType::Flower => "Flower",
            ItemType::Corn => "Corn",
            ItemType::Hay => "Hay",
            ItemType::WaterBucket => "Bucket",
            ItemType::Basket => "Basket",
            ItemType::Shovel => "Shovel",
            ItemType::Rake => "Rake",
            ItemType::Toy => "Toy",
            ItemType::Ribbon => "Ribbon",
            ItemType::Bell => "Bell",
            ItemType::Egg => "Egg",
            ItemType::Wool => "Wool",
            ItemType::ApplePie => "Pie",
            ItemType::MilkBottle => "Bottle",
        }
    }
}

// Visual feedback components
#[derive(Component)]
pub struct InteractionEffect {
    pub text: String,
    pub timer: Timer,
    pub color: Color,
}

#[derive(Component)]
pub struct FloatingText {
    pub text: String,
    pub velocity: Vec3,
    pub timer: Timer,
    pub color: Color,
}