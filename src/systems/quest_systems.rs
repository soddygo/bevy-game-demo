use bevy::prelude::*;
use crate::components::*;

pub struct QuestSystems;

impl QuestSystems {
    pub fn setup_quest_manager(mut commands: Commands) {
        commands.insert_resource(QuestManager::new());
    }
    
    pub fn initialize_daily_quests(
        mut quest_manager: ResMut<QuestManager>,
        time_of_day: Res<TimeOfDay>,
    ) {
        // Initialize quests at the start of each day
        if time_of_day.hour == 6 && time_of_day.minute == 0 && quest_manager.daily_quests.is_empty() {
            Self::generate_daily_quests(&mut quest_manager);
        }
    }
    
    pub fn generate_daily_quests(quest_manager: &mut QuestManager) {
        let daily_quests = vec![
            Quest {
                id: "daily_feed_5_animals".to_string(),
                title: "喂食小动物".to_string(),
                description: "喂食5只农场动物".to_string(),
                quest_type: QuestType::FeedAnimals,
                required_amount: 5,
                progress: 0,
                reward: QuestReward {
                    coins: 20,
                    items: vec![(ItemType::Apple, 2)],
                    friendship_bonus: 1,
                },
                is_daily: true,
                deadline: Some(1), // Due by end of day
            },
            Quest {
                id: "daily_collect_3_items".to_string(),
                title: "收集物品".to_string(),
                description: "收集3个任意物品".to_string(),
                quest_type: QuestType::CollectItems,
                required_amount: 3,
                progress: 0,
                reward: QuestReward {
                    coins: 15,
                    items: vec![(ItemType::Carrot, 1)],
                    friendship_bonus: 0,
                },
                is_daily: true,
                deadline: Some(1),
            },
            Quest {
                id: "daily_pet_3_animals".to_string(),
                title: "抚摸动物".to_string(),
                description: "抚摸3只动物增加好感度".to_string(),
                quest_type: QuestType::InteractWithAnimals,
                required_amount: 3,
                progress: 0,
                reward: QuestReward {
                    coins: 10,
                    items: vec![(ItemType::Flower, 1)],
                    friendship_bonus: 2,
                },
                is_daily: true,
                deadline: Some(1),
            },
        ];
        
        quest_manager.daily_quests = daily_quests.clone();
        for quest in daily_quests {
            quest_manager.add_quest(quest);
        }
        
        println!("新的每日任务已生成！");
    }
    
    pub fn track_feeding_quests(
        mut quest_manager: ResMut<QuestManager>,
        animal_query: Query<&Animal>,
    ) {
        // Count animals with high hunger (recently fed)
        let recently_fed_count = animal_query.iter()
            .filter(|animal| animal.hunger > 80.0)
            .count() as u32;
        
        // Update feeding quests
        let mut quests_to_update = Vec::new();
        for quest in &quest_manager.active_quests {
            if quest.quest_type == QuestType::FeedAnimals {
                quests_to_update.push((quest.id.clone(), recently_fed_count));
            }
        }
        
        for (quest_id, progress) in quests_to_update {
            quest_manager.update_quest_progress(&quest_id, progress);
        }
    }
    
    pub fn track_interaction_quests(
        mut quest_manager: ResMut<QuestManager>,
        interaction_query: Query<&Interactable>,
    ) {
        // Count successful interactions
        let interaction_count = interaction_query.iter().count() as u32;
        
        // Update interaction quests
        let mut quests_to_update = Vec::new();
        for quest in &quest_manager.active_quests {
            if quest.quest_type == QuestType::InteractWithAnimals {
                quests_to_update.push((quest.id.clone(), interaction_count));
            }
        }
        
        for (quest_id, progress) in quests_to_update {
            quest_manager.update_quest_progress(&quest_id, progress);
        }
    }
    
    pub fn check_quest_completion(
        mut quest_manager: ResMut<QuestManager>,
        mut player_stats: ResMut<PlayerStats>,
        mut inventory: ResMut<Inventory>,
    ) {
        let mut completed_quests = Vec::new();
        
        for quest in &quest_manager.active_quests {
            if quest.progress >= quest.required_amount {
                completed_quests.push(quest.id.clone());
            }
        }
        
        for quest_id in completed_quests {
            if let Some(quest_index) = quest_manager.active_quests.iter()
                .position(|q| q.id == quest_id) {
                let quest = quest_manager.active_quests[quest_index].clone();
                
                // Give rewards
                player_stats.coins += quest.reward.coins;
                
                for (item_type, quantity) in quest.reward.items {
                    let item = Item {
                        item_type,
                        quantity,
                        name: Self::get_item_name(item_type),
                        category: Self::get_item_category(item_type),
                        value: Self::get_item_value(item_type),
                        description: Self::get_item_description(item_type),
                    };
                    inventory.add_item(item);
                }
                
                println!("任务完成：{}！获得 {} 金币", quest.title, quest.reward.coins);
                quest_manager.complete_quest(&quest_id);
            }
        }
    }
    
    pub fn display_quests(quest_manager: Res<QuestManager>) {
        // Display active quests periodically
        if !quest_manager.active_quests.is_empty() {
            println!("当前活跃任务：");
            for quest in &quest_manager.active_quests {
                println!("  - {}: {}/{}", quest.title, quest.progress, quest.required_amount);
            }
        }
    }
    
    fn get_item_name(item_type: ItemType) -> String {
        match item_type {
            ItemType::Apple => "苹果".to_string(),
            ItemType::Carrot => "胡萝卜".to_string(),
            ItemType::Bone => "骨头".to_string(),
            ItemType::Fish => "鱼".to_string(),
            ItemType::Milk => "牛奶".to_string(),
            ItemType::Wheat => "小麦".to_string(),
            ItemType::Flower => "花朵".to_string(),
            ItemType::Corn => "玉米".to_string(),
            ItemType::Hay => "干草".to_string(),
            ItemType::Toy => "玩具".to_string(),
            ItemType::Ribbon => "丝带".to_string(),
            ItemType::Bell => "铃铛".to_string(),
            ItemType::Egg => "鸡蛋".to_string(),
            ItemType::Wool => "羊毛".to_string(),
            ItemType::ApplePie => "苹果派".to_string(),
            ItemType::MilkBottle => "奶瓶".to_string(),
            _ => "未知物品".to_string(),
        }
    }
    
    fn get_item_category(item_type: ItemType) -> ItemCategory {
        match item_type {
            ItemType::Apple | ItemType::Carrot | ItemType::Fish | ItemType::Milk | 
            ItemType::Wheat | ItemType::Corn | ItemType::Hay | ItemType::ApplePie | 
            ItemType::MilkBottle => ItemCategory::Food,
            ItemType::WaterBucket | ItemType::Basket | ItemType::Shovel | ItemType::Rake => ItemCategory::Tool,
            ItemType::Toy | ItemType::Ribbon | ItemType::Bell | ItemType::Flower => ItemCategory::Decoration,
            ItemType::Egg | ItemType::Wool => ItemCategory::Special,
            _ => ItemCategory::Food,
        }
    }
    
    fn get_item_value(item_type: ItemType) -> u32 {
        match item_type {
            ItemType::Apple => 5,
            ItemType::Carrot => 3,
            ItemType::Bone => 8,
            ItemType::Fish => 12,
            ItemType::Milk => 10,
            ItemType::Wheat => 2,
            ItemType::Flower => 1,
            ItemType::Corn => 4,
            ItemType::Hay => 3,
            ItemType::Toy => 15,
            ItemType::Ribbon => 6,
            ItemType::Bell => 10,
            ItemType::Egg => 8,
            ItemType::Wool => 20,
            ItemType::ApplePie => 25,
            ItemType::MilkBottle => 12,
            _ => 1,
        }
    }
    
    fn get_item_description(item_type: ItemType) -> String {
        match item_type {
            ItemType::Apple => "新鲜的红苹果".to_string(),
            ItemType::Carrot => "脆嫩的胡萝卜".to_string(),
            ItemType::Bone => "小狗最爱的骨头".to_string(),
            ItemType::Fish => "新鲜的鱼".to_string(),
            ItemType::Milk => "营养丰富的牛奶".to_string(),
            ItemType::Wheat => "金黄的小麦".to_string(),
            ItemType::Flower => "美丽的花朵".to_string(),
            ItemType::Corn => "甜玉米".to_string(),
            ItemType::Hay => "干草，动物的食物".to_string(),
            ItemType::Toy => "有趣的玩具".to_string(),
            ItemType::Ribbon => "漂亮的丝带".to_string(),
            ItemType::Bell => "清脆的铃铛".to_string(),
            ItemType::Egg => "新鲜的鸡蛋".to_string(),
            ItemType::Wool => "柔软的羊毛".to_string(),
            ItemType::ApplePie => "美味的苹果派".to_string(),
            ItemType::MilkBottle => "装在瓶子里的牛奶".to_string(),
            _ => "普通物品".to_string(),
        }
    }
}