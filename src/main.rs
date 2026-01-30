// mod App;

use bevy::prelude::*;
use bevy::image::{ImageSampler, ImageSamplerDescriptor, ImageAddressMode, ImageLoaderSettings};

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    Playing,
    GameOver,
}

#[derive(Resource)]
struct PlayerChoice {
    character_path: String,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Health {
    current: f32,
    max: f32,
}

#[derive(Component)]
struct AttackTimer(Timer);

#[derive(Component)]
struct TargetPosition(Vec3);

#[derive(Component)]
struct HealthBar;

#[derive(Component)]
struct Projectile {
    velocity: Vec3,
    damage: f32,
    is_player: bool,
}

#[derive(Component)]
struct Prop;

#[derive(Component)]
struct ClickIndicator;

#[derive(Component)]
struct MenuUI;

#[derive(Component)]
enum MenuButton {
    SelectCharA,
    SelectCharB,
    StartGame,
    Restart,
}

#[derive(Resource)]
struct EnemySpawnTimer(Timer);

#[derive(Resource)]
struct Progress {
    min_x: f32,
    wall_x: f32,
}

#[derive(Resource, Default)]
struct HoverPosition(Vec3);

impl Default for Progress {
    fn default() -> Self {
        Self {
            min_x: 0.0,
            wall_x: 20.0,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "League WASM Game".into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .insert_resource(PlayerChoice { character_path: "Models/GLB format/character-a.glb".to_string() })
        .insert_resource(EnemySpawnTimer(Timer::from_seconds(5.0, TimerMode::Repeating)))
        .init_resource::<Progress>()
        .init_resource::<HoverPosition>()
        .add_systems(OnEnter(GameState::Menu), setup_menu)
        .add_systems(Update, menu_interaction.run_if(in_state(GameState::Menu).or(in_state(GameState::GameOver))))
        .add_systems(OnExit(GameState::Menu), cleanup_menu)
        .add_systems(OnEnter(GameState::Playing), setup_game)
        .add_systems(Update, (
            handle_input,
            move_player,
            move_camera,
            spawn_enemies,
            move_enemies,
            combat_system,
            update_projectiles,
            update_health_bars,
            handle_death,
            loop_environment,
        ).run_if(in_state(GameState::Playing)))
        .add_systems(OnExit(GameState::Playing), cleanup_playing)
        .add_systems(OnEnter(GameState::GameOver), setup_game_over)
        .add_systems(OnExit(GameState::GameOver), cleanup_menu)
        .run();
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d::default(),
        MenuUI,
    ));

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        MenuUI,
    )).with_children(|parent| {
        parent.spawn((
            Text::new("League WASM Game"),
            TextFont::from_font_size(60.0),
            TextColor(Color::WHITE),
        ));
        
        parent.spawn(Node {
            flex_direction: FlexDirection::Row,
            ..default()
        }).with_children(|row| {
            // Character A
            row.spawn((
                Button,
                Node {
                    width: Val::Px(200.0),
                    height: Val::Px(250.0),
                    margin: UiRect::all(Val::Px(10.0)),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15).into()),
                MenuButton::SelectCharA,
            )).with_children(|p| {
                p.spawn((
                    ImageNode::new(asset_server.load("Previews/character-a.png")),
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(150.0),
                        ..default()
                    },
                ));
                p.spawn((Text::new("Character A"), TextFont::from_font_size(20.0)));
            });

            // Character B
            row.spawn((
                Button,
                Node {
                    width: Val::Px(200.0),
                    height: Val::Px(250.0),
                    margin: UiRect::all(Val::Px(10.0)),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15).into()),
                MenuButton::SelectCharB,
            )).with_children(|p| {
                p.spawn((
                    ImageNode::new(asset_server.load("Previews/character-b.png")),
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(150.0),
                        ..default()
                    },
                ));
                p.spawn((Text::new("Character B"), TextFont::from_font_size(20.0)));
            });
        });

        // Start Game
        parent.spawn((
            Button,
            Node {
                width: Val::Px(250.0),
                height: Val::Px(60.0),
                margin: UiRect::all(Val::Px(20.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.45, 0.15).into()),
            MenuButton::StartGame,
        )).with_child((Text::new("START GAME"), TextFont::from_font_size(30.0)));
    });
}

fn menu_interaction(
    mut next_state: ResMut<NextState<GameState>>,
    mut player_choice: ResMut<PlayerChoice>,
    interaction_query: Query<
        (&Interaction, &MenuButton),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match button {
                MenuButton::SelectCharA => {
                    player_choice.character_path = "Models/GLB format/character-a.glb".to_string();
                }
                MenuButton::SelectCharB => {
                    player_choice.character_path = "Models/GLB format/character-b.glb".to_string();
                }
                MenuButton::StartGame => {
                    next_state.set(GameState::Playing);
                }
                MenuButton::Restart => {
                    next_state.set(GameState::Menu);
                }
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuUI>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_choice: Res<PlayerChoice>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera - top down LoL style (isometric-ish)
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(20.0, 20.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Light
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4)),
    ));

    // Character
    commands.spawn((
        Player,
        Health { current: 200.0, max: 200.0 },
        AttackTimer({
            let mut t = Timer::from_seconds(0.5, TimerMode::Once);
            t.set_elapsed(std::time::Duration::from_secs_f32(0.5));
            t
        }),
        TargetPosition(Vec3::ZERO),
        SceneRoot(asset_server.load(format!("{}#Scene0", player_choice.character_path))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    )).with_children(|parent| {
        // Health bar background
        parent.spawn((
            Mesh3d(meshes.add(Plane3d::default().mesh().size(2.0, 0.2))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.2, 0.0, 0.0),
                unlit: true,
                ..default()
            })),
            Transform::from_xyz(0.0, 3.5, 0.0).with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        ));
        // Health bar foreground
        parent.spawn((
            HealthBar,
            Mesh3d(meshes.add(Plane3d::default().mesh().size(1.0, 1.0))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.0, 1.0, 0.0),
                unlit: true,
                ..default()
            })),
            Transform::from_xyz(0.0, 3.51, 0.0)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2))
                .with_scale(Vec3::new(2.0, 1.0, 0.2)),
        ));
    });

    // Ground plane
    let mut grass_mesh = Plane3d::default().mesh().size(2000.0, 2000.0).build();
    if let Some(bevy::render::mesh::VertexAttributeValues::Float32x2(ref mut uvs)) = grass_mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0) {
        for uv in uvs {
            uv[0] *= 200.0;
            uv[1] *= 200.0;
        }
    }

    commands.spawn((
        Mesh3d(meshes.add(grass_mesh)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.4, 0.2),
            base_color_texture: Some(asset_server.load_with_settings(
                "PNG/Default/background_terrain_top.png",
                |s: &mut ImageLoaderSettings| {
                    s.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                        address_mode_u: ImageAddressMode::Repeat,
                        address_mode_v: ImageAddressMode::Repeat,
                        ..default()
                    });
                }
            )),
            perceptual_roughness: 0.9,
            ..default()
        })),
    ));

    // Click Indicator (Small dot)
    commands.spawn((
        ClickIndicator,
        Mesh3d(meshes.add(Sphere::new(0.2).mesh())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.0, 1.0, 1.0), // Cyan
            emissive: LinearRgba::new(0.0, 2.0, 2.0, 1.0),
            ..default()
        })),
        Transform::from_xyz(0.0, -1.0, 0.0),
    ));

    // Spawn Buildings in a left-diagonal lane layout (x axis)
    let requested_buildings = [
        "Models/GLB format/building-i.glb",
        "Models/GLB format/building-p.glb",
        "Models/GLB format/building-j.glb",
        "Models/GLB format/building-s.glb",
    ];

    let lane_half_width = 8.0;
    let spacing = 10.0; // Increased density
    let building_scale = 2.5;

    // Tree assets (primitives)
    let trunk_mesh = meshes.add(Cylinder::new(0.3, 1.0));
    let leaves_mesh = meshes.add(Cone::new(1.2, 2.5));
    let trunk_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.4, 0.2, 0.1),
        ..default()
    });
    let leaves_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.1, 0.5, 0.1),
        ..default()
    });

    for i in -40i32..=40 {
        let t = i as f32 * spacing;
        let center = Vec3::new(t, 0.0, 0.0);
        let side_offset = Vec3::Z * lane_half_width;

        // Buildings
        commands.spawn((
            Prop,
            SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(requested_buildings[(i.abs() as usize) % 4]))),
            Transform::from_translation(center + side_offset)
                .with_scale(Vec3::splat(building_scale)),
        ));

        commands.spawn((
            Prop,
            SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(requested_buildings[((i.abs() as usize) + 2) % 4]))),
            Transform::from_translation(center - side_offset)
                .with_scale(Vec3::splat(building_scale)),
        ));

        // Trees between buildings
        if i < 40 {
            let tree_t = t + spacing / 2.0;
            let tree_center = Vec3::new(tree_t, 0.0, 0.0);
            
            spawn_tree(&mut commands, tree_center + side_offset, trunk_mesh.clone(), leaves_mesh.clone(), trunk_material.clone(), leaves_material.clone());
            spawn_tree(&mut commands, tree_center - side_offset, trunk_mesh.clone(), leaves_mesh.clone(), trunk_material.clone(), leaves_material.clone());
        }
    }
}

fn spawn_tree(
    commands: &mut Commands,
    pos: Vec3,
    trunk_mesh: Handle<Mesh>,
    leaves_mesh: Handle<Mesh>,
    trunk_mat: Handle<StandardMaterial>,
    leaves_mat: Handle<StandardMaterial>,
) {
    commands.spawn((
        Prop,
        Mesh3d(trunk_mesh),
        MeshMaterial3d(trunk_mat),
        Transform::from_translation(pos + Vec3::Y * 0.5),
    ));
    commands.spawn((
        Prop,
        Mesh3d(leaves_mesh),
        MeshMaterial3d(leaves_mat),
        Transform::from_translation(pos + Vec3::Y * 2.0),
    ));
}

fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut timer: ResMut<EnemySpawnTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player_query: Query<&Transform, With<Player>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        if let Ok(player_transform) = player_query.get_single() {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            
            let enemy_models = [
                "Models/GLB format/character-p.glb",
                "Models/GLB format/character-q.glb",
                "Models/GLB format/character-n.glb",
                "Models/GLB format/character-m.glb",
            ];
            let model_path = enemy_models[rng.gen_range(0..enemy_models.len())];

            // Spawn ahead of player
            let spawn_x = player_transform.translation.x - 60.0;
            let spawn_z = (time.elapsed_secs().sin() * 5.0) + rng.gen_range(-3.0..3.0);
            
            commands.spawn((
                Enemy,
                Health { current: 100.0, max: 100.0 },
                AttackTimer({
                    let mut t = Timer::from_seconds(2.0, TimerMode::Once);
                    t.set_elapsed(std::time::Duration::from_secs_f32(2.0));
                    t
                }),
                SceneRoot(asset_server.load(format!("{}#Scene0", model_path))),
                Transform::from_xyz(spawn_x, 0.0, spawn_z),
            )).with_children(|parent| {
                // Health bar background
                parent.spawn((
                    Mesh3d(meshes.add(Plane3d::default().mesh().size(2.0, 0.2))),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: Color::srgb(0.2, 0.0, 0.0),
                        unlit: true,
                        ..default()
                    })),
                    Transform::from_xyz(0.0, 3.5, 0.0).with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                ));
                // Health bar foreground
                parent.spawn((
                    HealthBar,
                    Mesh3d(meshes.add(Plane3d::default().mesh().size(1.0, 1.0))),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: Color::srgb(1.0, 0.0, 0.0), // Red for enemies
                        unlit: true,
                        ..default()
                    })),
                    Transform::from_xyz(0.0, 3.51, 0.0)
                        .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2))
                        .with_scale(Vec3::new(2.0, 1.0, 0.2)),
                ));
            });
        }
    }
}

fn move_enemies(
    time: Res<Time>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for mut enemy_transform in &mut enemy_query {
            let player_pos = player_transform.translation;
            let enemy_pos = enemy_transform.translation;
            let dir = (player_pos - enemy_pos).normalize();
            let dist = player_pos.distance(enemy_pos);
            
            if dist > 10.0 {
                enemy_transform.translation += dir * 5.0 * time.delta_secs();
                enemy_transform.look_to(dir, Vec3::Y);
            } else {
                // Just face the player if close enough to stop
                enemy_transform.look_to(dir, Vec3::Y);
            }
        }
    }
}

fn combat_system(
    mut commands: Commands,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Transform, &mut AttackTimer), With<Player>>,
    mut enemy_query: Query<(&Transform, &mut AttackTimer), (With<Enemy>, Without<Player>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    hover_pos: Res<HoverPosition>,
) {
    let projectile_mesh = meshes.add(Cuboid::new(0.1, 0.1, 1.5).mesh());
    let player_projectile_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 1.0, 1.0),
        emissive: LinearRgba::new(0.0, 10.0, 10.0, 1.0),
        ..default()
    });
    let enemy_projectile_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.0, 0.0),
        emissive: LinearRgba::new(10.0, 0.0, 0.0, 1.0),
        ..default()
    });

    if let Ok((player_transform, mut player_timer)) = player_query.get_single_mut() {
        player_timer.0.tick(time.delta());
        
        if keys.pressed(KeyCode::Space) && player_timer.0.finished() {
            let mut shoot_dir = hover_pos.0 - player_transform.translation;
            shoot_dir.y = 0.0;
            let dir = shoot_dir.normalize_or_zero();
            
            let dir = if dir == Vec3::ZERO {
                // Default shoot forward (same as player orientation)
                Vec3::new(-1.0, 0.0, -1.0).normalize()
            } else {
                dir
            };

            commands.spawn((
                Projectile {
                    velocity: dir * 25.0,
                    damage: 25.0,
                    is_player: true,
                },
                Mesh3d(projectile_mesh.clone()),
                MeshMaterial3d(player_projectile_mat.clone()),
                Transform::from_translation(player_transform.translation + Vec3::Y * 1.5)
                    .looking_to(dir, Vec3::Y),
            ));
            
            player_timer.0.reset();
        }
    }

    for (enemy_transform, mut enemy_timer) in &mut enemy_query {
        enemy_timer.0.tick(time.delta());
        if enemy_timer.0.finished() {
            if let Ok((player_transform, _)) = player_query.get_single() {
                let dist = enemy_transform.translation.distance(player_transform.translation);
                if dist < 35.0 {
                    let dir = (player_transform.translation - enemy_transform.translation).normalize();
                    commands.spawn((
                        Projectile {
                            velocity: dir * 15.0,
                            damage: 10.0,
                            is_player: false,
                        },
                        Mesh3d(projectile_mesh.clone()),
                        MeshMaterial3d(enemy_projectile_mat.clone()),
                        Transform::from_translation(enemy_transform.translation + Vec3::Y * 1.5)
                            .looking_to(dir, Vec3::Y),
                    ));
                    enemy_timer.0.reset();
                }
            }
        }
    }
}

fn move_camera(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
    progress: Res<Progress>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            let offset = Vec3::new(20.0, 20.0, 20.0);
            // Camera X follows progress.min_x (forward progress)
            // Camera Z and Y follow current player position to stay centered
            camera_transform.translation.x = progress.min_x + offset.x;
            camera_transform.translation.y = player_transform.translation.y + offset.y;
            camera_transform.translation.z = player_transform.translation.z + offset.z;
            
            camera_transform.look_at(Vec3::new(progress.min_x, player_transform.translation.y, player_transform.translation.z), Vec3::Y);
        }
    }
}

fn update_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    mut projectile_query: Query<(Entity, &mut Transform, &Projectile), (Without<Player>, Without<Enemy>)>,
    mut player_query: Query<(&Transform, &mut Health), With<Player>>,
    mut enemy_query: Query<(&Transform, &mut Health), (With<Enemy>, Without<Player>)>,
) {
    for (projectile_entity, mut projectile_transform, projectile) in &mut projectile_query {
        projectile_transform.translation += projectile.velocity * time.delta_secs();

        if projectile_transform.translation.length() > 500.0 {
            commands.entity(projectile_entity).despawn();
            continue;
        }

        if projectile.is_player {
            for (enemy_transform, mut health) in &mut enemy_query {
                if projectile_transform.translation.distance(enemy_transform.translation + Vec3::Y * 1.5) < 2.0 {
                    health.current -= projectile.damage;
                    commands.entity(projectile_entity).despawn();
                    break;
                }
            }
        } else {
            if let Ok((player_transform, mut health)) = player_query.get_single_mut() {
                if projectile_transform.translation.distance(player_transform.translation + Vec3::Y * 1.5) < 2.0 {
                    health.current -= projectile.damage;
                    commands.entity(projectile_entity).despawn();
                }
            }
        }
    }
}

fn update_health_bars(
    mut health_bar_query: Query<(&mut Transform, &Parent), With<HealthBar>>,
    health_query: Query<&Health>,
) {
    for (mut transform, parent) in &mut health_bar_query {
        if let Ok(health) = health_query.get(parent.get()) {
            let ratio = (health.current / health.max).max(0.0);
            transform.scale.x = ratio * 2.0;
        }
    }
}

fn handle_death(
    mut commands: Commands,
    query: Query<(Entity, &Health, Option<&Player>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (entity, health, is_player) in &query {
        if health.current <= 0.0 {
            commands.entity(entity).despawn_recursive();
            if is_player.is_some() {
                next_state.set(GameState::GameOver);
            }
        }
    }
}

fn setup_game_over(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        MenuUI,
    ));

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        MenuUI,
    )).with_children(|parent| {
        parent.spawn((
            Text::new("GAME OVER"),
            TextFont::from_font_size(80.0),
            TextColor(Color::srgb(1.0, 0.0, 0.0)),
        ));
        
        parent.spawn((
            Button,
            Node {
                width: Val::Px(250.0),
                height: Val::Px(60.0),
                margin: UiRect::all(Val::Px(20.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15).into()),
            MenuButton::Restart,
        )).with_child((Text::new("RESTART"), TextFont::from_font_size(30.0)));
    });
}

fn cleanup_playing(
    mut commands: Commands,
    query: Query<Entity, Or<(With<Player>, With<Enemy>, With<Projectile>, With<Prop>, With<ClickIndicator>, With<Camera3d>, With<DirectionalLight>)>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn loop_environment(
    mut commands: Commands,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut target_query: Query<&mut TargetPosition, With<Player>>,
    mut props_query: Query<&mut Transform, (With<Prop>, Without<Player>)>,
    enemy_query: Query<Entity, With<Enemy>>,
    mut progress: ResMut<Progress>,
) {
    let mut teleport_offset = 0.0;
    for mut transform in &mut player_query {
        if transform.translation.x < -350.0 {
            teleport_offset = 700.0;
            transform.translation.x += teleport_offset;
            if let Ok(mut target) = target_query.get_single_mut() {
                target.0.x += teleport_offset;
            }
            progress.min_x = transform.translation.x;
            progress.wall_x = progress.min_x + 30.0;
        }
    }

    if teleport_offset != 0.0 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for mut prop_transform in &mut props_query {
            // Reposition props to new "random" positions in the lane
            let side = if prop_transform.translation.z > 0.0 { 1.0 } else { -1.0 };
            prop_transform.translation.z = side * (8.0 + rng.gen_range(-2.0..4.0));
        }

        // Clear enemies to keep it fresh
        for entity in &enemy_query {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn handle_input(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut player_query: Query<&mut TargetPosition, With<Player>>,
    mut indicator_query: Query<&mut Transform, (With<ClickIndicator>, Without<Player>)>,
    progress: Res<Progress>,
    mut hover_pos: ResMut<HoverPosition>,
) {
    let window = window_query.single();
    if let Ok((camera, camera_transform)) = camera_query.get_single() {
        if let Some(cursor_position) = window.cursor_position() {
            if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) {
                let t = -ray.origin.y / ray.direction.y;
                if t > 0.0 {
                    let mut ground_pos = ray.origin + ray.direction * t;
                    
                    // Constrain X to be within the wall
                    if ground_pos.x > progress.wall_x {
                        ground_pos.x = progress.wall_x;
                    }

                    // Update hover position resource
                    hover_pos.0 = ground_pos;

                    // Always update indicator to hover position
                    for mut indicator_transform in indicator_query.iter_mut() {
                        indicator_transform.translation = ground_pos + Vec3::Y * 0.1;
                    }

                    // Only update target on click
                    if mouse_button_input.just_pressed(MouseButton::Right) {
                        for mut target_pos in player_query.iter_mut() {
                            target_pos.0 = ground_pos;
                        }
                    }
                }
            }
        }
    }
}

fn move_player(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &TargetPosition), With<Player>>,
    mut progress: ResMut<Progress>,
) {
    for (mut transform, target) in query.iter_mut() {
        let direction = target.0 - transform.translation;
        let distance = direction.length();
        
        if distance > 0.1 {
            let move_speed = 7.0;
            let move_delta = direction.normalize() * move_speed * time.delta_secs();
            
            if move_delta.length() > distance {
                transform.translation = target.0;
            } else {
                transform.translation += move_delta;
            }
        }

        // Block from moving downwards (backwards)
        if transform.translation.x > progress.wall_x {
            transform.translation.x = progress.wall_x;
        }

        // Update progress and move wall forward
        if transform.translation.x < progress.min_x {
            progress.min_x = transform.translation.x;
            progress.wall_x = progress.min_x + 15.0; // The "old platform" is left behind
        }

        let back_to_camera_dir = Vec3::new(-1.0, 0.0, -1.0).normalize();
        let target_rotation = transform.looking_to(back_to_camera_dir, Vec3::Y).rotation;
        transform.rotation = target_rotation;
    }
}
