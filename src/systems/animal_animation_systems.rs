use bevy::prelude::*;
use crate::components::*;

pub struct AnimalAnimationSystems;

impl AnimalAnimationSystems {
    pub fn spawn_animal_with_animation(
        commands: &mut Commands,
        position: Vec3,
        animal_type: AnimalType,
    ) -> Entity {
        let mut entity_commands = commands.spawn_empty();
        
        match animal_type {
            AnimalType::Chicken => {
                // 鸡 - 精细的外观设计
                let body_color = Color::srgb(1.0, 0.9, 0.7);
                let comb_color = Color::srgb(1.0, 0.2, 0.2);
                let beak_color = Color::srgb(1.0, 0.8, 0.3);
                
                // 身体
                entity_commands.with_child((
                    Sprite {
                        color: body_color,
                        custom_size: Some(Vec2::new(16.0, 14.0)),
                        ..default()
                    },
                    Transform::from_xyz(0.0, 0.0, 0.0),
                ));
                
                // 鸡冠
                entity_commands.with_child((
                    Sprite {
                        color: comb_color,
                        custom_size: Some(Vec2::new(6.0, 4.0)),
                        ..default()
                    },
                    Transform::from_xyz(-2.0, 8.0, 0.1),
                ));
                
                // 嘴巴
                entity_commands.with_child((
                    Sprite {
                        color: beak_color,
                        custom_size: Some(Vec2::new(4.0, 2.0)),
                        ..default()
                    },
                    Transform::from_xyz(-8.0, 5.0, 0.1),
                ));
                
                // 眼睛
                entity_commands.with_child((
                    Sprite {
                        color: Color::srgb(0.0, 0.0, 0.0),
                        custom_size: Some(Vec2::new(2.0, 2.0)),
                        ..default()
                    },
                    Transform::from_xyz(-6.0, 6.0, 0.1),
                ));
                
                // 翅膀
                entity_commands.with_child((
                    Sprite {
                        color: Color::srgb(0.9, 0.8, 0.6),
                        custom_size: Some(Vec2::new(6.0, 8.0)),
                        ..default()
                    },
                    Transform::from_xyz(5.0, 2.0, 0.0),
                ));
                
                // 腿
                entity_commands.with_child((
                    Sprite {
                        color: Color::srgb(1.0, 0.8, 0.3),
                        custom_size: Some(Vec2::new(2.0, 4.0)),
                        ..default()
                    },
                    Transform::from_xyz(-3.0, -8.0, 0.0),
                ));
                entity_commands.with_child((
                    Sprite {
                        color: Color::srgb(1.0, 0.8, 0.3),
                        custom_size: Some(Vec2::new(2.0, 4.0)),
                        ..default()
                    },
                    Transform::from_xyz(3.0, -8.0, 0.0),
                ));
                
                // 标识
                entity_commands.with_child((
                    Text2d::new("🐔"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.0, 0.0, 0.0)),
                    Transform::from_xyz(0.0, 0.0, 1.0),
                ));
            },
            
            AnimalType::Cow => {
                // 奶牛 - 经典的黑白花纹
                let body_color = Color::srgb(0.8, 0.8, 0.8);
                let spot_color = Color::srgb(0.2, 0.2, 0.2);
                let udder_color = Color::srgb(1.0, 0.9, 0.8);
                
                // 身体
                entity_commands.with_child((
                    Sprite {
                        color: body_color,
                        custom_size: Some(Vec2::new(28.0, 20.0)),
                        ..default()
                    },
                    Transform::from_xyz(0.0, 0.0, 0.0),
                ));
                
                // 黑色斑点
                entity_commands.with_child((
                    Sprite {
                        color: spot_color,
                        custom_size: Some(Vec2::new(6.0, 8.0)),
                        ..default()
                    },
                    Transform::from_xyz(-8.0, 4.0, 0.1),
                ));
                entity_commands.with_child((
                    Sprite {
                        color: spot_color,
                        custom_size: Some(Vec2::new(4.0, 6.0)),
                        ..default()
                    },
                    Transform::from_xyz(8.0, -2.0, 0.1),
                ));
                
                // 头部
                entity_commands.with_child((
                    Sprite {
                        color: body_color,
                        custom_size: Some(Vec2::new(14.0, 12.0)),
                        ..default()
                    },
                    Transform::from_xyz(-14.0, 6.0, 0.0),
                ));
                
                // 鼻子
                entity_commands.with_child((
                    Sprite {
                        color: Color::srgb(1.0, 0.8, 0.6),
                        custom_size: Some(Vec2::new(6.0, 4.0)),
                        ..default()
                    },
                    Transform::from_xyz(-20.0, 4.0, 0.1),
                ));
                
                // 乳房
                entity_commands.with_child((
                    Sprite {
                        color: udder_color,
                        custom_size: Some(Vec2::new(8.0, 6.0)),
                        ..default()
                    },
                    Transform::from_xyz(0.0, -12.0, 0.0),
                ));
                
                // 腿
                for x in [-8.0, -2.0, 2.0, 8.0] {
                    entity_commands.with_child((
                        Sprite {
                            color: Color::srgb(0.3, 0.3, 0.3),
                            custom_size: Some(Vec2::new(3.0, 8.0)),
                            ..default()
                        },
                        Transform::from_xyz(x, -14.0, 0.0),
                    ));
                }
                
                // 尾巴
                entity_commands.with_child((
                    Sprite {
                        color: Color::srgb(0.7, 0.6, 0.4),
                        custom_size: Some(Vec2::new(3.0, 10.0)),
                        ..default()
                    },
                    Transform::from_xyz(12.0, 0.0, 0.0),
                ));
                
                // 标识
                entity_commands.with_child((
                    Text2d::new("🐄"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.0, 0.0, 0.0)),
                    Transform::from_xyz(0.0, 0.0, 1.0),
                ));
            },
            
            AnimalType::Sheep => {
                // 绵羊 - 毛茸茸的外观
                let wool_color = Color::srgb(0.95, 0.95, 0.95);
                let face_color = Color::srgb(0.8, 0.7, 0.6);
                
                // 身体（毛茸茸的）
                entity_commands.with_child((
                    Sprite {
                        color: wool_color,
                        custom_size: Some(Vec2::new(24.0, 20.0)),
                        ..default()
                    },
                    Transform::from_xyz(0.0, 0.0, 0.0),
                ));
                
                // 毛发纹理
                for (i, pos) in [(-6.0, 4.0), (6.0, 4.0), (0.0, 6.0), (-8.0, -2.0), (8.0, -2.0)].iter().enumerate() {
                    entity_commands.with_child((
                        Sprite {
                            color: Color::srgb(0.9, 0.9, 0.9),
                            custom_size: Some(Vec2::new(4.0, 4.0)),
                            ..default()
                        },
                        Transform::from_xyz(pos.0, pos.1, 0.1),
                    ));
                }
                
                // 头部
                entity_commands.with_child((
                    Sprite {
                        color: face_color,
                        custom_size: Some(Vec2::new(10.0, 8.0)),
                        ..default()
                    },
                    Transform::from_xyz(-10.0, 4.0, 0.2),
                ));
                
                // 腿
                for x in [-6.0, -2.0, 2.0, 6.0] {
                    entity_commands.with_child((
                        Sprite {
                            color: Color::srgb(0.4, 0.3, 0.2),
                            custom_size: Some(Vec2::new(2.0, 6.0)),
                            ..default()
                        },
                        Transform::from_xyz(x, -12.0, 0.0),
                    ));
                }
                
                // 标识
                entity_commands.with_child((
                    Text2d::new("🐑"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.0, 0.0, 0.0)),
                    Transform::from_xyz(0.0, 0.0, 1.0),
                ));
            },
            
            AnimalType::Dog => {
                // 狗 - 忠诚的外观
                let fur_color = Color::srgb(0.6, 0.4, 0.2);
                let ear_color = Color::srgb(0.5, 0.3, 0.1);
                
                // 身体
                entity_commands.with_child((
                    Sprite {
                        color: fur_color,
                        custom_size: Some(Vec2::new(20.0, 14.0)),
                        ..default()
                    },
                    Transform::from_xyz(0.0, 0.0, 0.0),
                ));
                
                // 头部
                entity_commands.with_child((
                    Sprite {
                        color: fur_color,
                        custom_size: Some(Vec2::new(12.0, 10.0)),
                        ..default()
                    },
                    Transform::from_xyz(-10.0, 4.0, 0.0),
                ));
                
                // 耳朵
                entity_commands.with_child((
                    Sprite {
                        color: ear_color,
                        custom_size: Some(Vec2::new(4.0, 8.0)),
                        ..default()
                    },
                    Transform::from_xyz(-14.0, 6.0, -0.1),
                ));
                entity_commands.with_child((
                    Sprite {
                        color: ear_color,
                        custom_size: Some(Vec2::new(4.0, 8.0)),
                        ..default()
                    },
                    Transform::from_xyz(-6.0, 6.0, -0.1),
                ));
                
                // 鼻子
                entity_commands.with_child((
                    Sprite {
                        color: Color::srgb(0.1, 0.1, 0.1),
                        custom_size: Some(Vec2::new(3.0, 2.0)),
                        ..default()
                    },
                    Transform::from_xyz(-16.0, 4.0, 0.1),
                ));
                
                // 尾巴（摇动的）
                entity_commands.with_child((
                    Sprite {
                        color: fur_color,
                        custom_size: Some(Vec2::new(8.0, 3.0)),
                        ..default()
                    },
                    Transform::from_xyz(8.0, 2.0, 0.0),
                ));
                
                // 腿
                for (i, x) in [-6.0, -2.0, 2.0, 6.0].iter().enumerate() {
                    entity_commands.with_child((
                        Sprite {
                            color: fur_color,
                            custom_size: Some(Vec2::new(2.0, 6.0)),
                            ..default()
                        },
                        Transform::from_xyz(*x, -8.0, 0.0),
                    ));
                }
                
                // 标识
                entity_commands.with_child((
                    Text2d::new("🐕"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.0, 0.0, 0.0)),
                    Transform::from_xyz(0.0, 0.0, 1.0),
                ));
            },
            
            AnimalType::Cat => {
                // 猫 - 优雅的外观
                let fur_color = Color::srgb(0.7, 0.5, 0.3);
                let stripe_color = Color::srgb(0.4, 0.3, 0.2);
                
                // 身体
                entity_commands.with_child((
                    Sprite {
                        color: fur_color,
                        custom_size: Some(Vec2::new(16.0, 12.0)),
                        ..default()
                    },
                    Transform::from_xyz(0.0, 0.0, 0.0),
                ));
                
                // 条纹
                entity_commands.with_child((
                    Sprite {
                        color: stripe_color,
                        custom_size: Some(Vec2::new(2.0, 8.0)),
                        ..default()
                    },
                    Transform::from_xyz(-4.0, 0.0, 0.1),
                ));
                entity_commands.with_child((
                    Sprite {
                        color: stripe_color,
                        custom_size: Some(Vec2::new(2.0, 6.0)),
                        ..default()
                    },
                    Transform::from_xyz(4.0, 2.0, 0.1),
                ));
                
                // 头部
                entity_commands.with_child((
                    Sprite {
                        color: fur_color,
                        custom_size: Some(Vec2::new(10.0, 8.0)),
                        ..default()
                    },
                    Transform::from_xyz(-8.0, 4.0, 0.0),
                ));
                
                // 耳朵（三角形）
                entity_commands.with_child((
                    Sprite {
                        color: fur_color,
                        custom_size: Some(Vec2::new(3.0, 5.0)),
                        ..default()
                    },
                    Transform::from_xyz(-12.0, 8.0, 0.0),
                ));
                entity_commands.with_child((
                    Sprite {
                        color: fur_color,
                        custom_size: Some(Vec2::new(3.0, 5.0)),
                        ..default()
                    },
                    Transform::from_xyz(-4.0, 8.0, 0.0),
                ));
                
                // 尾巴（长而优雅）
                entity_commands.with_child((
                    Sprite {
                        color: fur_color,
                        custom_size: Some(Vec2::new(10.0, 2.0)),
                        ..default()
                    },
                    Transform::from_xyz(6.0, 0.0, 0.0),
                ));
                
                // 腿
                for x in [-4.0, -1.0, 1.0, 4.0] {
                    entity_commands.with_child((
                        Sprite {
                            color: fur_color,
                            custom_size: Some(Vec2::new(1.5, 5.0)),
                            ..default()
                        },
                        Transform::from_xyz(x, -7.0, 0.0),
                    ));
                }
                
                // 标识
                entity_commands.with_child((
                    Text2d::new("🐈"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.0, 0.0, 0.0)),
                    Transform::from_xyz(0.0, 0.0, 1.0),
                ));
            },
            
            AnimalType::Pig => {
                // 猪 - 粉色可爱
                let body_color = Color::srgb(1.0, 0.7, 0.7);
                let snout_color = Color::srgb(1.0, 0.6, 0.6);
                
                // 身体（圆胖）
                entity_commands.with_child((
                    Sprite {
                        color: body_color,
                        custom_size: Some(Vec2::new(22.0, 18.0)),
                        ..default()
                    },
                    Transform::from_xyz(0.0, 0.0, 0.0),
                ));
                
                // 头部
                entity_commands.with_child((
                    Sprite {
                        color: body_color,
                        custom_size: Some(Vec2::new(12.0, 10.0)),
                        ..default()
                    },
                    Transform::from_xyz(-10.0, 2.0, 0.0),
                ));
                
                // 鼻子
                entity_commands.with_child((
                    Sprite {
                        color: snout_color,
                        custom_size: Some(Vec2::new(6.0, 4.0)),
                        ..default()
                    },
                    Transform::from_xyz(-16.0, 2.0, 0.1),
                ));
                
                // 鼻孔
                entity_commands.with_child((
                    Sprite {
                        color: Color::srgb(0.8, 0.5, 0.5),
                        custom_size: Some(Vec2::new(1.0, 1.0)),
                        ..default()
                    },
                    Transform::from_xyz(-17.0, 2.0, 0.2),
                ));
                entity_commands.with_child((
                    Sprite {
                        color: Color::srgb(0.8, 0.5, 0.5),
                        custom_size: Some(Vec2::new(1.0, 1.0)),
                        ..default()
                    },
                    Transform::from_xyz(-15.0, 2.0, 0.2),
                ));
                
                // 腿
                for x in [-6.0, -2.0, 2.0, 6.0] {
                    entity_commands.with_child((
                        Sprite {
                            color: Color::srgb(0.9, 0.6, 0.6),
                            custom_size: Some(Vec2::new(2.0, 6.0)),
                            ..default()
                        },
                        Transform::from_xyz(x, -10.0, 0.0),
                    ));
                }
                
                // 尾巴（卷曲）
                entity_commands.with_child((
                    Sprite {
                        color: body_color,
                        custom_size: Some(Vec2::new(3.0, 3.0)),
                        ..default()
                    },
                    Transform::from_xyz(9.0, 2.0, 0.0),
                ));
                
                // 标识
                entity_commands.with_child((
                    Text2d::new("🐖"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.0, 0.0, 0.0)),
                    Transform::from_xyz(0.0, 0.0, 1.0),
                ));
            },
            
            AnimalType::Duck => {
                // 鸭子 - 黄色小鸭
                let body_color = Color::srgb(1.0, 0.8, 0.3);
                let beak_color = Color::srgb(1.0, 0.6, 0.2);
                
                // 身体
                entity_commands.with_child((
                    Sprite {
                        color: body_color,
                        custom_size: Some(Vec2::new(18.0, 14.0)),
                        ..default()
                    },
                    Transform::from_xyz(0.0, 0.0, 0.0),
                ));
                
                // 头部
                entity_commands.with_child((
                    Sprite {
                        color: body_color,
                        custom_size: Some(Vec2::new(10.0, 8.0)),
                        ..default()
                    },
                    Transform::from_xyz(-8.0, 4.0, 0.0),
                ));
                
                // 嘴巴
                entity_commands.with_child((
                    Sprite {
                        color: beak_color,
                        custom_size: Some(Vec2::new(6.0, 3.0)),
                        ..default()
                    },
                    Transform::from_xyz(-14.0, 4.0, 0.1),
                ));
                
                // 眼睛
                entity_commands.with_child((
                    Sprite {
                        color: Color::srgb(0.0, 0.0, 0.0),
                        custom_size: Some(Vec2::new(1.5, 1.5)),
                        ..default()
                    },
                    Transform::from_xyz(-10.0, 6.0, 0.1),
                ));
                
                // 翅膀
                entity_commands.with_child((
                    Sprite {
                        color: Color::srgb(0.9, 0.7, 0.2),
                        custom_size: Some(Vec2::new(6.0, 8.0)),
                        ..default()
                    },
                    Transform::from_xyz(4.0, 2.0, 0.0),
                ));
                
                // 腿
                entity_commands.with_child((
                    Sprite {
                        color: Color::srgb(1.0, 0.6, 0.2),
                        custom_size: Some(Vec2::new(1.5, 4.0)),
                        ..default()
                    },
                    Transform::from_xyz(-3.0, -8.0, 0.0),
                ));
                entity_commands.with_child((
                    Sprite {
                        color: Color::srgb(1.0, 0.6, 0.2),
                        custom_size: Some(Vec2::new(1.5, 4.0)),
                        ..default()
                    },
                    Transform::from_xyz(3.0, -8.0, 0.0),
                ));
                
                // 标识
                entity_commands.with_child((
                    Text2d::new("🦆"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.0, 0.0, 0.0)),
                    Transform::from_xyz(0.0, 0.0, 1.0),
                ));
            },
            
            AnimalType::Horse => {
                // 马 - 雄壮的外观
                let body_color = Color::srgb(0.6, 0.4, 0.2);
                let mane_color = Color::srgb(0.4, 0.3, 0.1);
                
                // 身体
                entity_commands.with_child((
                    Sprite {
                        color: body_color,
                        custom_size: Some(Vec2::new(32.0, 24.0)),
                        ..default()
                    },
                    Transform::from_xyz(0.0, 0.0, 0.0),
                ));
                
                // 头部
                entity_commands.with_child((
                    Sprite {
                        color: body_color,
                        custom_size: Some(Vec2::new(16.0, 12.0)),
                        ..default()
                    },
                    Transform::from_xyz(-16.0, 8.0, 0.0),
                ));
                
                // 鬃毛
                entity_commands.with_child((
                    Sprite {
                        color: mane_color,
                        custom_size: Some(Vec2::new(4.0, 10.0)),
                        ..default()
                    },
                    Transform::from_xyz(-12.0, 12.0, 0.1),
                ));
                
                // 脖子
                entity_commands.with_child((
                    Sprite {
                        color: body_color,
                        custom_size: Some(Vec2::new(8.0, 12.0)),
                        ..default()
                    },
                    Transform::from_xyz(-10.0, 4.0, 0.0),
                ));
                
                // 腿
                for (i, x) in [-10.0, -4.0, 4.0, 10.0].iter().enumerate() {
                    entity_commands.with_child((
                        Sprite {
                            color: body_color,
                            custom_size: Some(Vec2::new(3.0, 12.0)),
                            ..default()
                        },
                        Transform::from_xyz(*x, -14.0, 0.0),
                    ));
                }
                
                // 尾巴
                entity_commands.with_child((
                    Sprite {
                        color: mane_color,
                        custom_size: Some(Vec2::new(6.0, 16.0)),
                        ..default()
                    },
                    Transform::from_xyz(14.0, 4.0, 0.0),
                ));
                
                // 标识
                entity_commands.with_child((
                    Text2d::new("🐴"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.0, 0.0, 0.0)),
                    Transform::from_xyz(0.0, 0.0, 1.0),
                ));
            },
        }

        entity_commands.insert((
            Transform::from_xyz(position.x, position.y, position.z),
            Animal {
                animal_type,
                speed: 50.0,
                friendship_level: 0,
                hunger: 30.0,
            },
            AnimalAI {
                state: AnimalState::Idle,
                target_position: Vec2::new(position.x, position.y),
                idle_timer: Timer::from_seconds(3.0, TimerMode::Once),
            },
            AnimalAnimation {
                frame_timer: Timer::from_seconds(0.2, TimerMode::Repeating),
                current_frame: 0,
                is_eating: false,
                is_happy: false,
                is_sleeping: false,
                animation_timer: Timer::from_seconds(2.0, TimerMode::Once),
                facing_left: false,
            },
            Interactable {
                interaction_type: InteractionType::Feed,
                interaction_range: 40.0,
            },
        )).id()
    }

    pub fn update_animal_animations(
        time: Res<Time>,
        mut query: Query<(&mut AnimalAnimation, &AnimalAI, &Animal, &Children)>,
        mut sprite_query: Query<&mut Sprite>,
        mut transform_query: Query<&mut Transform>,
    ) {
        for (mut animation, ai, animal, children) in query.iter_mut() {
            // Update frame timer
            animation.frame_timer.tick(time.delta());
            if animation.frame_timer.just_finished() {
                animation.current_frame = (animation.current_frame + 1) % 4;
            }

            // Update animation states
            animation.animation_timer.tick(time.delta());
            if animation.animation_timer.just_finished() {
                animation.is_eating = false;
                animation.is_happy = false;
                animation.is_sleeping = false;
            }

            // Set animation states based on animal state
            match ai.state {
                AnimalState::Eating => {
                    animation.is_eating = true;
                    animation.animation_timer.reset();
                }
                AnimalState::Idle => {
                    if animal.hunger > 70.0 {
                        animation.is_sleeping = true;
                        animation.animation_timer.reset();
                    }
                }
                _ => {}
            }

            // Apply animation effects based on animal type and state
            match animal.animal_type {
                AnimalType::Chicken => {
                    Self::animate_chicken(&mut animation, children, &mut sprite_query, &mut transform_query);
                },
                AnimalType::Cow => {
                    Self::animate_cow(&mut animation, children, &mut sprite_query, &mut transform_query);
                },
                AnimalType::Sheep => {
                    Self::animate_sheep(&mut animation, children, &mut sprite_query, &mut transform_query);
                },
                AnimalType::Dog => {
                    Self::animate_dog(&mut animation, children, &mut sprite_query, &mut transform_query);
                },
                AnimalType::Cat => {
                    Self::animate_cat(&mut animation, children, &mut sprite_query, &mut transform_query);
                },
                AnimalType::Pig => {
                    Self::animate_pig(&mut animation, children, &mut sprite_query, &mut transform_query);
                },
                AnimalType::Duck => {
                    Self::animate_duck(&mut animation, children, &mut sprite_query, &mut transform_query);
                },
                AnimalType::Horse => {
                    Self::animate_horse(&mut animation, children, &mut sprite_query, &mut transform_query);
                },
            }
        }
    }

    fn animate_chicken(
        animation: &mut AnimalAnimation,
        children: &Children,
        sprite_query: &mut Query<&mut Sprite>,
        transform_query: &mut Query<&mut Transform>,
    ) {
        let body_color = Color::srgb(1.0, 0.9, 0.7);
        let comb_color = Color::srgb(1.0, 0.2, 0.2);
        
        for &child in children {
            if let Ok(mut sprite) = sprite_query.get_mut(child) {
                // 通过颜色值判断部位类型
                let current_srgba = sprite.color.to_srgba();
                let original_color = if current_srgba.red > 0.8 && current_srgba.green < 0.3 {
                    comb_color // 鸡冠
                } else if current_srgba.red < 0.1 && current_srgba.green < 0.1 && current_srgba.blue < 0.1 {
                    Color::srgb(0.0, 0.0, 0.0) // 眼睛
                } else {
                    body_color // 身体
                };
                
                let mut modified_color = original_color;
                
                if animation.is_eating {
                    // 啄食动画
                    if let Ok(mut transform) = transform_query.get_mut(child) {
                        let bob_amount = (animation.current_frame as f32 * 0.8).sin() * 2.0;
                        transform.translation.y = bob_amount;
                    }
                    
                    let pulse = (animation.current_frame as f32 * 0.8).sin() * 0.3;
                    let base_srgba = original_color.to_srgba();
                    modified_color = Color::srgba(
                        (base_srgba.red + pulse).clamp(0.0, 1.0),
                        (base_srgba.green + pulse).clamp(0.0, 1.0),
                        (base_srgba.blue + pulse).clamp(0.0, 1.0),
                        1.0,
                    );
                }
                
                if animation.is_happy {
                    let brightness = 1.0 + (animation.current_frame as f32 * 0.5).sin() * 0.2;
                    let base_srgba = original_color.to_srgba();
                    modified_color = Color::srgba(
                        (base_srgba.red * brightness).clamp(0.0, 1.0),
                        (base_srgba.green * brightness).clamp(0.0, 1.0),
                        (base_srgba.blue * brightness).clamp(0.0, 1.0),
                        1.0,
                    );
                }
                
                if animation.is_sleeping {
                    let darkness = 0.6 + (animation.current_frame as f32 * 0.3).sin() * 0.1;
                    let base_srgba = original_color.to_srgba();
                    modified_color = Color::srgba(
                        base_srgba.red * darkness,
                        base_srgba.green * darkness,
                        base_srgba.blue * darkness,
                        1.0,
                    );
                }
                
                sprite.color = modified_color;
            }
        }
    }

    fn animate_cow(
        animation: &mut AnimalAnimation,
        children: &Children,
        sprite_query: &mut Query<&mut Sprite>,
        transform_query: &mut Query<&mut Transform>,
    ) {
        let body_color = Color::srgb(0.8, 0.8, 0.8);
        let spot_color = Color::srgb(0.2, 0.2, 0.2);
        
        for &child in children {
            if let Ok(mut sprite) = sprite_query.get_mut(child) {
                // 通过颜色值判断部位类型
                let current_srgba = sprite.color.to_srgba();
                let original_color = if current_srgba.red < 0.3 && current_srgba.green < 0.3 && current_srgba.blue < 0.3 {
                    spot_color // 斑点
                } else {
                    body_color // 身体
                };
                
                let mut modified_color = original_color;
                
                if animation.is_eating {
                    // 咀嚼动画
                    if let Ok(mut transform) = transform_query.get_mut(child) {
                        let chew_amount = (animation.current_frame as f32 * 1.2).sin() * 1.0;
                        transform.translation.y = chew_amount.abs();
                    }
                    
                    let pulse = (animation.current_frame as f32 * 1.2).sin() * 0.15;
                    let base_srgba = original_color.to_srgba();
                    modified_color = Color::srgba(
                        (base_srgba.red + pulse).clamp(0.0, 1.0),
                        (base_srgba.green + pulse).clamp(0.0, 1.0),
                        (base_srgba.blue + pulse).clamp(0.0, 1.0),
                        1.0,
                    );
                }
                
                if animation.is_happy {
                    let brightness = 1.0 + (animation.current_frame as f32 * 0.4).sin() * 0.15;
                    let base_srgba = original_color.to_srgba();
                    modified_color = Color::srgba(
                        (base_srgba.red * brightness).clamp(0.0, 1.0),
                        (base_srgba.green * brightness).clamp(0.0, 1.0),
                        (base_srgba.blue * brightness).clamp(0.0, 1.0),
                        1.0,
                    );
                }
                
                sprite.color = modified_color;
            }
        }
    }

    fn animate_sheep(
        animation: &mut AnimalAnimation,
        children: &Children,
        sprite_query: &mut Query<&mut Sprite>,
        transform_query: &mut Query<&mut Transform>,
    ) {
        let wool_color = Color::srgb(0.95, 0.95, 0.95);
        
        for &child in children {
            if let Ok(mut sprite) = sprite_query.get_mut(child) {
                let original_color = wool_color;
                let mut modified_color = original_color;
                
                if animation.is_eating {
                    // 吃草动画
                    if let Ok(mut transform) = transform_query.get_mut(child) {
                        let graze_amount = (animation.current_frame as f32 * 0.6).sin() * 1.5;
                        transform.translation.y = graze_amount.abs() - 1.5;
                    }
                    
                    let pulse = (animation.current_frame as f32 * 0.6).sin() * 0.1;
                    let base_srgba = original_color.to_srgba();
                    modified_color = Color::srgba(
                        (base_srgba.red + pulse).clamp(0.0, 1.0),
                        (base_srgba.green + pulse).clamp(0.0, 1.0),
                        (base_srgba.blue + pulse).clamp(0.0, 1.0),
                        1.0,
                    );
                }
                
                if animation.is_happy {
                    // 跳跃动画
                    if let Ok(mut transform) = transform_query.get_mut(child) {
                        let jump_amount = (animation.current_frame as f32 * 0.8).sin() * 3.0;
                        transform.translation.y = jump_amount.max(0.0);
                    }
                    
                    let brightness = 1.0 + (animation.current_frame as f32 * 0.8).sin() * 0.2;
                    let base_srgba = original_color.to_srgba();
                    modified_color = Color::srgba(
                        (base_srgba.red * brightness).clamp(0.0, 1.0),
                        (base_srgba.green * brightness).clamp(0.0, 1.0),
                        (base_srgba.blue * brightness).clamp(0.0, 1.0),
                        1.0,
                    );
                }
                
                if animation.is_sleeping {
                    let darkness = 0.8 + (animation.current_frame as f32 * 0.4).sin() * 0.05;
                    let base_srgba = original_color.to_srgba();
                    modified_color = Color::srgba(
                        base_srgba.red * darkness,
                        base_srgba.green * darkness,
                        base_srgba.blue * darkness,
                        1.0,
                    );
                }
                
                sprite.color = modified_color;
            }
        }
    }

    fn animate_dog(
        animation: &mut AnimalAnimation,
        children: &Children,
        sprite_query: &mut Query<&mut Sprite>,
        transform_query: &mut Query<&mut Transform>,
    ) {
        let fur_color = Color::srgb(0.6, 0.4, 0.2);
        
        for &child in children {
            if let Ok(mut sprite) = sprite_query.get_mut(child) {
                let original_color = fur_color;
                let mut modified_color = original_color;
                
                if animation.is_eating {
                    // 快速吃食动画
                    if let Ok(mut transform) = transform_query.get_mut(child) {
                        let eat_amount = (animation.current_frame as f32 * 1.5).sin() * 2.0;
                        transform.translation.y = eat_amount;
                    }
                    
                    let pulse = (animation.current_frame as f32 * 1.5).sin() * 0.2;
                    let base_srgba = original_color.to_srgba();
                    modified_color = Color::srgba(
                        (base_srgba.red + pulse).clamp(0.0, 1.0),
                        (base_srgba.green + pulse).clamp(0.0, 1.0),
                        (base_srgba.blue + pulse).clamp(0.0, 1.0),
                        1.0,
                    );
                }
                
                if animation.is_happy {
                    // 摇尾巴动画
                    if let Ok(mut transform) = transform_query.get_mut(child) {
                        if transform.translation.x > 5.0 { // 尾巴
                            let wag_amount = (animation.current_frame as f32 * 2.0).sin() * 15.0;
                            transform.rotation = Quat::from_rotation_z(wag_amount.to_radians());
                        }
                    }
                    
                    let brightness = 1.0 + (animation.current_frame as f32 * 1.0).sin() * 0.25;
                    let base_srgba = original_color.to_srgba();
                    modified_color = Color::srgba(
                        (base_srgba.red * brightness).clamp(0.0, 1.0),
                        (base_srgba.green * brightness).clamp(0.0, 1.0),
                        (base_srgba.blue * brightness).clamp(0.0, 1.0),
                        1.0,
                    );
                }
                
                sprite.color = modified_color;
            }
        }
    }

    fn animate_cat(
        animation: &mut AnimalAnimation,
        children: &Children,
        sprite_query: &mut Query<&mut Sprite>,
        transform_query: &mut Query<&mut Transform>,
    ) {
        let fur_color = Color::srgb(0.7, 0.5, 0.3);
        
        for &child in children {
            if let Ok(mut sprite) = sprite_query.get_mut(child) {
                let original_color = fur_color;
                let mut modified_color = original_color;
                
                if animation.is_eating {
                    // 优雅吃食动画
                    if let Ok(mut transform) = transform_query.get_mut(child) {
                        let eat_amount = (animation.current_frame as f32 * 0.8).sin() * 1.0;
                        transform.translation.y = eat_amount;
                    }
                    
                    let pulse = (animation.current_frame as f32 * 0.8).sin() * 0.15;
                    let base_srgba = original_color.to_srgba();
                    modified_color = Color::srgba(
                        (base_srgba.red + pulse).clamp(0.0, 1.0),
                        (base_srgba.green + pulse).clamp(0.0, 1.0),
                        (base_srgba.blue + pulse).clamp(0.0, 1.0),
                        1.0,
                    );
                }
                
                if animation.is_happy {
                    // 尾巴摆动动画
                    if let Ok(mut transform) = transform_query.get_mut(child) {
                        if transform.translation.x > 4.0 { // 尾巴
                            let sway_amount = (animation.current_frame as f32 * 0.6).sin() * 10.0;
                            transform.rotation = Quat::from_rotation_z(sway_amount.to_radians());
                        }
                    }
                    
                    let brightness = 1.0 + (animation.current_frame as f32 * 0.6).sin() * 0.2;
                    let base_srgba = original_color.to_srgba();
                    modified_color = Color::srgba(
                        (base_srgba.red * brightness).clamp(0.0, 1.0),
                        (base_srgba.green * brightness).clamp(0.0, 1.0),
                        (base_srgba.blue * brightness).clamp(0.0, 1.0),
                        1.0,
                    );
                }
                
                sprite.color = modified_color;
            }
        }
    }

    fn animate_pig(
        animation: &mut AnimalAnimation,
        children: &Children,
        sprite_query: &mut Query<&mut Sprite>,
        transform_query: &mut Query<&mut Transform>,
    ) {
        let body_color = Color::srgb(1.0, 0.7, 0.7);
        
        for &child in children {
            if let Ok(mut sprite) = sprite_query.get_mut(child) {
                let original_color = body_color;
                let mut modified_color = original_color;
                
                if animation.is_eating {
                    // 用鼻子拱食动画
                    if let Ok(mut transform) = transform_query.get_mut(child) {
                        let snuffle_amount = (animation.current_frame as f32 * 1.0).sin() * 1.5;
                        transform.translation.y = snuffle_amount;
                        
                        // 鼻子部位额外动画
                        if transform.translation.x < -10.0 {
                            transform.rotation = Quat::from_rotation_z(snuffle_amount.to_radians());
                        }
                    }
                    
                    let pulse = (animation.current_frame as f32 * 1.0).sin() * 0.25;
                    let base_srgba = original_color.to_srgba();
                    modified_color = Color::srgba(
                        (base_srgba.red + pulse).clamp(0.0, 1.0),
                        (base_srgba.green + pulse).clamp(0.0, 1.0),
                        (base_srgba.blue + pulse).clamp(0.0, 1.0),
                        1.0,
                    );
                }
                
                if animation.is_happy {
                    // 快乐摇摆动画
                    if let Ok(mut transform) = transform_query.get_mut(child) {
                        let wiggle_amount = (animation.current_frame as f32 * 1.2).sin() * 2.0;
                        transform.translation.x = wiggle_amount;
                    }
                    
                    let brightness = 1.0 + (animation.current_frame as f32 * 1.2).sin() * 0.3;
                    let base_srgba = original_color.to_srgba();
                    modified_color = Color::srgba(
                        (base_srgba.red * brightness).clamp(0.0, 1.0),
                        (base_srgba.green * brightness).clamp(0.0, 1.0),
                        (base_srgba.blue * brightness).clamp(0.0, 1.0),
                        1.0,
                    );
                }
                
                sprite.color = modified_color;
            }
        }
    }

    fn animate_duck(
        animation: &mut AnimalAnimation,
        children: &Children,
        sprite_query: &mut Query<&mut Sprite>,
        transform_query: &mut Query<&mut Transform>,
    ) {
        let body_color = Color::srgb(1.0, 0.8, 0.3);
        
        for &child in children {
            if let Ok(mut sprite) = sprite_query.get_mut(child) {
                let original_color = body_color;
                let mut modified_color = original_color;
                
                if animation.is_eating {
                    // 鸭子觅食动画
                    if let Ok(mut transform) = transform_query.get_mut(child) {
                        let dabbling_amount = (animation.current_frame as f32 * 0.9).sin() * 2.0;
                        transform.translation.y = dabbling_amount;
                        
                        // 头部额外动画
                        if transform.translation.x < -5.0 {
                            transform.rotation = Quat::from_rotation_z(dabbling_amount.to_radians() * 0.5);
                        }
                    }
                    
                    let pulse = (animation.current_frame as f32 * 0.9).sin() * 0.2;
                    let base_srgba = original_color.to_srgba();
                    modified_color = Color::srgba(
                        (base_srgba.red + pulse).clamp(0.0, 1.0),
                        (base_srgba.green + pulse).clamp(0.0, 1.0),
                        (base_srgba.blue + pulse).clamp(0.0, 1.0),
                        1.0,
                    );
                }
                
                if animation.is_happy {
                    // 鸭子摇摆动画
                    if let Ok(mut transform) = transform_query.get_mut(child) {
                        let waddle_amount = (animation.current_frame as f32 * 0.7).sin() * 1.0;
                        transform.translation.x = waddle_amount;
                    }
                    
                    let brightness = 1.0 + (animation.current_frame as f32 * 0.7).sin() * 0.25;
                    let base_srgba = original_color.to_srgba();
                    modified_color = Color::srgba(
                        (base_srgba.red * brightness).clamp(0.0, 1.0),
                        (base_srgba.green * brightness).clamp(0.0, 1.0),
                        (base_srgba.blue * brightness).clamp(0.0, 1.0),
                        1.0,
                    );
                }
                
                sprite.color = modified_color;
            }
        }
    }

    fn animate_horse(
        animation: &mut AnimalAnimation,
        children: &Children,
        sprite_query: &mut Query<&mut Sprite>,
        transform_query: &mut Query<&mut Transform>,
    ) {
        let body_color = Color::srgb(0.6, 0.4, 0.2);
        let mane_color = Color::srgb(0.4, 0.3, 0.1);
        
        for &child in children {
            if let Ok(mut sprite) = sprite_query.get_mut(child) {
                // 通过颜色值判断部位类型
                let current_srgba = sprite.color.to_srgba();
                let original_color = if current_srgba.red < 0.5 && current_srgba.green < 0.35 && current_srgba.blue < 0.15 {
                    mane_color // 鬃毛
                } else {
                    body_color // 身体
                };
                
                let mut modified_color = original_color;
                
                if animation.is_eating {
                    // 马匹低头吃草动画
                    if let Ok(mut transform) = transform_query.get_mut(child) {
                        let graze_amount = (animation.current_frame as f32 * 0.5).sin() * 3.0;
                        transform.translation.y = graze_amount.abs() - 2.0;
                        
                        // 头部和脖子旋转
                        if transform.translation.x < -5.0 {
                            transform.rotation = Quat::from_rotation_z(graze_amount.to_radians() * 0.3);
                        }
                    }
                    
                    let pulse = (animation.current_frame as f32 * 0.5).sin() * 0.1;
                    let base_srgba = original_color.to_srgba();
                    modified_color = Color::srgba(
                        (base_srgba.red + pulse).clamp(0.0, 1.0),
                        (base_srgba.green + pulse).clamp(0.0, 1.0),
                        (base_srgba.blue + pulse).clamp(0.0, 1.0),
                        1.0,
                    );
                }
                
                if animation.is_happy {
                    // 马匹昂首动画
                    if let Ok(mut transform) = transform_query.get_mut(child) {
                        let proud_amount = (animation.current_frame as f32 * 0.4).sin() * 2.0;
                        transform.translation.y = proud_amount.max(0.0);
                        
                        // 鬃毛飘动
                        let current_srgba = sprite.color.to_srgba();
                        if current_srgba.red < 0.5 && current_srgba.green < 0.35 && current_srgba.blue < 0.15 {
                            transform.rotation = Quat::from_rotation_z(proud_amount.to_radians() * 0.5);
                        }
                    }
                    
                    let brightness = 1.0 + (animation.current_frame as f32 * 0.4).sin() * 0.15;
                    let base_srgba = original_color.to_srgba();
                    modified_color = Color::srgba(
                        (base_srgba.red * brightness).clamp(0.0, 1.0),
                        (base_srgba.green * brightness).clamp(0.0, 1.0),
                        (base_srgba.blue * brightness).clamp(0.0, 1.0),
                        1.0,
                    );
                }
                
                sprite.color = modified_color;
            }
        }
    }

    pub fn trigger_feeding_animation(
        commands: &mut Commands,
        transform: &Transform,
    ) {
        // Spawn feeding effect
        commands.spawn((
            Sprite {
                color: Color::srgba(1.0, 1.0, 0.0, 0.8),
                custom_size: Some(Vec2::new(8.0, 8.0)),
                ..default()
            },
            Transform::from_xyz(
                transform.translation.x,
                transform.translation.y + 15.0,
                transform.translation.z + 1.0,
            ),
            FeedingEffect {
                timer: Timer::from_seconds(1.0, TimerMode::Once),
            },
        ));
    }

    pub fn trigger_happy_animation(
        commands: &mut Commands,
        transform: &Transform,
    ) {
        // Spawn heart effects
        for i in 0..3 {
            let offset_x = (i as f32 - 1.0) * 10.0;
            commands.spawn((
                Sprite {
                    color: Color::srgba(1.0, 0.0, 1.0, 0.9),
                    custom_size: Some(Vec2::new(6.0, 6.0)),
                    ..default()
                },
                Transform::from_xyz(
                    transform.translation.x + offset_x,
                    transform.translation.y + 20.0 + (i as f32) * 5.0,
                    transform.translation.z + 1.0,
                ),
                HeartEffect {
                    timer: Timer::from_seconds(1.5, TimerMode::Once),
                    velocity: Vec3::new(offset_x * 0.5, 30.0, 0.0),
                },
            ));
        }
    }
}

#[derive(Component)]
pub struct FeedingEffect {
    pub timer: Timer,
}

#[derive(Component)]
pub struct HeartEffect {
    pub timer: Timer,
    pub velocity: Vec3,
}

pub fn update_feeding_effects(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut FeedingEffect, &mut Transform, &mut Sprite)>,
) {
    for (entity, mut effect, mut transform, mut sprite) in query.iter_mut() {
        effect.timer.tick(time.delta());
        
        // Fade out and move upward
        let alpha = 1.0 - effect.timer.fraction();
        let base_srgba = sprite.color.to_srgba();
        sprite.color = Color::srgba(base_srgba.red, base_srgba.green, base_srgba.blue, alpha);
        transform.translation.y += 20.0 * time.delta_secs();
        
        if effect.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn update_heart_effects(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut HeartEffect, &mut Transform, &mut Sprite)>,
) {
    for (entity, mut effect, mut transform, mut sprite) in query.iter_mut() {
        effect.timer.tick(time.delta());
        
        // Move upward and fade out
        transform.translation += effect.velocity * time.delta_secs();
        let alpha = 1.0 - effect.timer.fraction();
        let base_srgba = sprite.color.to_srgba();
        sprite.color = Color::srgba(base_srgba.red, base_srgba.green, base_srgba.blue, alpha);
        
        if effect.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}