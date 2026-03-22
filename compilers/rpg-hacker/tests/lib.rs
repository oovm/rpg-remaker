use rpg_hacker::*;

#[tokio::test]
async fn test_identify_game_version() {
    let service = MemoryRpgHackerService::new();
    
    // 测试版本识别
    let version = service.identify_game_version("test.rvproj2").await.unwrap();
    assert_eq!(version, RpgMakerVersion::RpgVxAce);
    
    let version = service.identify_game_version("test.rvproj").await.unwrap();
    assert_eq!(version, RpgMakerVersion::RpgVx);
}

#[tokio::test]
async fn test_resource_management() {
    let service = MemoryRpgHackerService::new();
    let manager = ResourceManager::new(Box::new(service));
    
    // 创建一个测试游戏文件
    let mut game_file = GameFile {
        path: "test".to_string(),
        version: RpgMakerVersion::RpgVxAce,
        title: "Test Game".to_string(),
        data: GameData {
            actors: Vec::new(),
            items: Vec::new(),
            skills: Vec::new(),
            enemies: Vec::new(),
            maps: Vec::new(),
            events: Vec::new(),
            system: SystemData {
                game_title: "Test Game".to_string(),
                game_version: "1.0".to_string(),
                system_settings: SystemSettings {
                    window_skin: "Window.png".to_string(),
                    battle_background: "Battle.png".to_string(),
                    battle_bgm: "Battle.mid".to_string(),
                    battle_end_bgm: "Victory.mid".to_string(),
                    game_over_bgm: "GameOver.mid".to_string(),
                    menu_bgm: "Menu.mid".to_string(),
                    system_sounds: SystemSounds {
                        ok: "OK.wav".to_string(),
                        cancel: "Cancel.wav".to_string(),
                        buzzer: "Buzzer.wav".to_string(),
                        equip: "Equip.wav".to_string(),
                        battle_start: "BattleStart.wav".to_string(),
                        escape: "Escape.wav".to_string(),
                        enemy_attack: "EnemyAttack.wav".to_string(),
                        enemy_damage: "EnemyDamage.wav".to_string(),
                        actor_attack: "ActorAttack.wav".to_string(),
                        actor_damage: "ActorDamage.wav".to_string(),
                        recover: "Recover.wav".to_string(),
                        skill: "Skill.wav".to_string(),
                        item: "Item.wav".to_string(),
                    },
                },
                database_settings: DatabaseSettings {
                    max_actors: 100,
                    max_items: 500,
                    max_skills: 300,
                    max_enemies: 200,
                    max_maps: 999,
                    max_events: 1000,
                },
            },
        },
        resources: Vec::new(),
    };
    
    // 添加资源
    let resource = Resource {
        path: "test.png".to_string(),
        resource_type: ResourceType::Image,
        size: 1024,
        content: Some(vec![0; 1024]),
    };
    
    manager.add_resource(&mut game_file, resource).await.unwrap();
    assert_eq!(game_file.resources.len(), 1);
    
    // 获取资源
    let retrieved_resource = manager.get_resource(&game_file, "test.png").await.unwrap();
    assert_eq!(retrieved_resource.path, "test.png");
    
    // 删除资源
    manager.remove_resource(&mut game_file, "test.png").await.unwrap();
    assert_eq!(game_file.resources.len(), 0);
}

#[tokio::test]
async fn test_game_data_modification() {
    let service = MemoryRpgHackerService::new();
    let modifier = GameDataModifier::new(Box::new(service));
    
    // 创建一个测试游戏文件
    let mut game_file = GameFile {
        path: "test".to_string(),
        version: RpgMakerVersion::RpgVxAce,
        title: "Test Game".to_string(),
        data: GameData {
            actors: vec![
                Actor {
                    id: 1,
                    name: "Actor 1".to_string(),
                    level: 1,
                    attributes: ActorAttributes {
                        max_hp: 100,
                        max_mp: 50,
                        attack: 10,
                        defense: 5,
                        magic_attack: 8,
                        magic_defense: 3,
                        agility: 7,
                        luck: 5,
                    },
                    skills: vec![1, 2],
                },
            ],
            items: Vec::new(),
            skills: Vec::new(),
            enemies: Vec::new(),
            maps: Vec::new(),
            events: Vec::new(),
            system: SystemData {
                game_title: "Test Game".to_string(),
                game_version: "1.0".to_string(),
                system_settings: SystemSettings {
                    window_skin: "Window.png".to_string(),
                    battle_background: "Battle.png".to_string(),
                    battle_bgm: "Battle.mid".to_string(),
                    battle_end_bgm: "Victory.mid".to_string(),
                    game_over_bgm: "GameOver.mid".to_string(),
                    menu_bgm: "Menu.mid".to_string(),
                    system_sounds: SystemSounds {
                        ok: "OK.wav".to_string(),
                        cancel: "Cancel.wav".to_string(),
                        buzzer: "Buzzer.wav".to_string(),
                        equip: "Equip.wav".to_string(),
                        battle_start: "BattleStart.wav".to_string(),
                        escape: "Escape.wav".to_string(),
                        enemy_attack: "EnemyAttack.wav".to_string(),
                        enemy_damage: "EnemyDamage.wav".to_string(),
                        actor_attack: "ActorAttack.wav".to_string(),
                        actor_damage: "ActorDamage.wav".to_string(),
                        recover: "Recover.wav".to_string(),
                        skill: "Skill.wav".to_string(),
                        item: "Item.wav".to_string(),
                    },
                },
                database_settings: DatabaseSettings {
                    max_actors: 100,
                    max_items: 500,
                    max_skills: 300,
                    max_enemies: 200,
                    max_maps: 999,
                    max_events: 1000,
                },
            },
        },
        resources: Vec::new(),
    };
    
    // 修改角色数据
    let actor_mod = ActorModification {
        id: 1,
        name: Some("Modified Actor".to_string()),
        level: Some(10),
        attributes: Some(ActorAttributes {
            max_hp: 1000,
            max_mp: 500,
            attack: 100,
            defense: 50,
            magic_attack: 80,
            magic_defense: 30,
            agility: 70,
            luck: 50,
        }),
        skills: Some(vec![1, 2, 3]),
    };
    
    modifier.modify_actor(&mut game_file, actor_mod).await.unwrap();
    assert_eq!(game_file.data.actors[0].name, "Modified Actor");
    assert_eq!(game_file.data.actors[0].level, 10);
    assert_eq!(game_file.data.actors[0].attributes.max_hp, 1000);
    assert_eq!(game_file.data.actors[0].skills, vec![1, 2, 3]);
}

#[tokio::test]
async fn test_version_compatibility() {
    let service = MemoryRpgHackerService::new();
    let checker = VersionCompatibilityChecker::new(Box::new(service));
    
    // 创建一个测试游戏文件
    let game_file = GameFile {
        path: "test".to_string(),
        version: RpgMakerVersion::RpgVx,
        title: "Test Game".to_string(),
        data: GameData {
            actors: Vec::new(),
            items: Vec::new(),
            skills: Vec::new(),
            enemies: Vec::new(),
            maps: Vec::new(),
            events: Vec::new(),
            system: SystemData {
                game_title: "Test Game".to_string(),
                game_version: "1.0".to_string(),
                system_settings: SystemSettings {
                    window_skin: "Window.png".to_string(),
                    battle_background: "Battle.png".to_string(),
                    battle_bgm: "Battle.mid".to_string(),
                    battle_end_bgm: "Victory.mid".to_string(),
                    game_over_bgm: "GameOver.mid".to_string(),
                    menu_bgm: "Menu.mid".to_string(),
                    system_sounds: SystemSounds {
                        ok: "OK.wav".to_string(),
                        cancel: "Cancel.wav".to_string(),
                        buzzer: "Buzzer.wav".to_string(),
                        equip: "Equip.wav".to_string(),
                        battle_start: "BattleStart.wav".to_string(),
                        escape: "Escape.wav".to_string(),
                        enemy_attack: "EnemyAttack.wav".to_string(),
                        enemy_damage: "EnemyDamage.wav".to_string(),
                        actor_attack: "ActorAttack.wav".to_string(),
                        actor_damage: "ActorDamage.wav".to_string(),
                        recover: "Recover.wav".to_string(),
                        skill: "Skill.wav".to_string(),
                        item: "Item.wav".to_string(),
                    },
                },
                database_settings: DatabaseSettings {
                    max_actors: 100,
                    max_items: 500,
                    max_skills: 300,
                    max_enemies: 200,
                    max_maps: 999,
                    max_events: 1000,
                },
            },
        },
        resources: Vec::new(),
    };
    
    // 检查兼容性
    let is_compatible = checker.check_rpgvx_ace_compatibility(&game_file).await.unwrap();
    assert!(is_compatible);
    
    // 获取兼容性报告
    let report = checker.get_compatibility_report(&game_file, RpgMakerVersion::RpgVxAce).await.unwrap();
    assert_eq!(report.source_version, RpgMakerVersion::RpgVx);
    assert_eq!(report.target_version, RpgMakerVersion::RpgVxAce);
    assert!(report.is_compatible);
}
