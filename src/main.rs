use std::fmt;

use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::render::mesh::CylinderAnchor;
use bevy::{color::palettes::basic::*, input::mouse::MouseButtonInput, prelude::*, ui};
use bevy::{
    color::palettes::tailwind::*, picking::pointer::PointerInteraction, prelude::*,
    window::PrimaryWindow, winit::WinitSettings,
};
use bevy_egui::egui::{text_edit, Sense};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use neargate::{
    Cell, Equippable, EquippableSlot, Game, GameState, Item, ItemQuality, ItemRarity, JobType,
    Spell, Unit, AURAS, CONSUMABLES, EFFECTS, SPELLS,
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

/*fn main() {
    let mut warrior = Unit::new("Warrior");
    //warrior.accuracy += 50.0;
    //warrior.evasion += 50.0;
    let mut mage = Unit::new("Mage");
    let weapon = Equippable::new("Weapon");
    mage.equip(&weapon);
    let armor = Equippable::new("Armor");
    mage.equip(&armor);
    mage.equip(&armor);
    let frostfire_bolt = &SPELLS["Frostfire Bolt"];
    let immunity = Spell::new("Immunity");
    mage.learn(&frostfire_bolt);
    mage.learn(&frostfire_bolt);
    mage.learn(&immunity);
    for spell_name in mage.spellbook.iter() {
        println!("Mage has learned: {}", spell_name);
    }
    mage.set_job(JobType::Theurgist);
    for job in mage.jobs.iter() {
        println!("Mage has the job: {:?}", job.1.job_type);
    }
    let savage_gladiator = AURAS["Savage Gladiator"];
    mage.auras.push(savage_gladiator.name);
    let cripple = AURAS["Cripple"];
    mage.auras.push(cripple.name);

    warrior.prepare();
    println!("{}", warrior.current_health);
    mage.prepare();
    println!("{}", mage.constitution);
    println!("{}", mage.current_health);

    mage.consumables.insert("Potion", 1);

    let consumables_keys: Vec<_> = mage.consumables.keys().cloned().collect();
    for key in consumables_keys {
        println!("Mage has the consumable: {}", key);
        let item = &CONSUMABLES[&key];
        item.use_item(&mut mage);
    }

    while warrior.is_alive() && mage.is_alive() {
        warrior.calculate_stats();
        warrior.attack(&mut mage);
        if warrior.is_alive() && mage.is_alive() {
            warrior.process_effects();
        }

        if warrior.is_alive() && mage.is_alive() {
            mage.calculate_stats();
            mage.cast(&mut warrior, &SPELLS[mage.spellbook[0]]);
            if warrior.is_alive() && mage.is_alive() {
                mage.process_effects();
            }
        }
    }
    println!("Warrior: {} HP", warrior.current_health);
    println!("Mage: {} HP", mage.current_health);

    if let Some(job) = mage.jobs.get_key_value(&JobType::Theurgist) {
        println!(
            "Mage has {} points in the {} job",
            job.1.points, job.1.job_type
        );
    }
}*/

use bevy::prelude::*;
fn main() {
    App::new()
        .init_resource::<Game>()
        .add_plugins((DefaultPlugins, MeshPickingPlugin, EguiPlugin))
        .init_resource::<OccupiedScreenSpace>()
        .init_state::<GameState>()
        .enable_state_scoped_entities::<GameState>()
        //.add_systems(Startup, setup_cameras)
        .add_systems(Startup, setup_system)
        .add_systems(Update, ui_example_system)
        //.add_systems(Update, update_camera_transform_system)
        .add_systems(Update, camera_drag_system)
        .add_systems(OnEnter(GameState::Playing), setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut game: ResMut<Game<'static>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = if std::env::var("GITHUB_ACTIONS") == Ok("true".to_string()) {
        // We're seeding the PRNG here to make this example deterministic for testing purposes.
        // This isn't strictly required in practical use unless you need your app to be deterministic.
        ChaCha8Rng::seed_from_u64(19878367467713)
    } else {
        ChaCha8Rng::from_entropy()
    };

    commands.spawn((
        StateScoped(GameState::Playing),
        PointLight {
            intensity: 2_000_000.0,
            shadows_enabled: true,
            range: 30.0,
            ..default()
        },
        Transform::from_xyz(4.0, 10.0, 4.0),
    ));

    let white_matl = materials.add(Color::WHITE);
    let ground_matl = materials.add(Color::from(GRAY_300));
    let hover_matl = materials.add(Color::from(CYAN_300));
    let pressed_matl = materials.add(Color::from(YELLOW_300));
    // spawn the game board
    let cell_scene = Cell::load_cell_scene(asset_server);
    game.board = (0..BOARD_SIZE_J)
        .map(|j| {
            (0..BOARD_SIZE_I)
                .map(|i| {
                    let height = rng.gen_range(-0.1..0.1);
                    commands
                        .spawn((
                            Mesh3d(meshes.add(Cuboid::new(1.0, 0.20, 1.0))),
                            MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
                            Transform::from_xyz(i as f32, 0.5, j as f32),
                        ))
                        .observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
                        .observe(update_material_on::<Pointer<Out>>(white_matl.clone()))
                        .observe(update_material_on::<Pointer<Down>>(pressed_matl.clone()))
                        .observe(update_material_on::<Pointer<Up>>(hover_matl.clone()));
                    Cell { height }
                })
                .collect()
        })
        .collect();
    commands.insert_resource(Random(rng));

    // Unit
    game.unit = Unit::new("Wellington");
    game.unit.auras.push("Savage Gladiator");
    game.unit.auras.push("Cripple");
    game.unit.effects.insert("Ignite", 3);
    game.unit.effects.insert("Frost", 3);
    let helm = Equippable::new("Order of the Wombat Fez");
    let mut legs = Equippable::new("Dressy Pantaloons");
    legs.equippable_slot = EquippableSlot::Legs;
    legs.item_quality = ItemQuality::Superior;
    legs.item_rarity = ItemRarity::Rare;
    let mut neck = Equippable::new("Necklace of the Silver Monkey");
    neck.equippable_slot = EquippableSlot::Neck;
    neck.item_quality = ItemQuality::Unique;
    neck.item_rarity = ItemRarity::Legendary;
    game.unit.equip(&helm);
    game.unit.equip(&legs);
    game.unit.equip(&neck);
    game.unit.prepare();

    let cylinder_pos = Vec3::new(2.0, 1.0, 2.0);
    commands
        .spawn((
            Mesh3d(meshes.add(Cylinder {
                radius: 0.3,
                half_height: 1.0,
                ..Default::default()
            })),
            MeshMaterial3d(materials.add(Color::srgb(0.8, 0.2, 0.2))),
            Transform::from_translation(cylinder_pos),
        ))
        .observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
        .observe(update_material_on::<Pointer<Out>>(white_matl.clone()))
        .observe(update_material_on::<Pointer<Down>>(pressed_matl.clone()))
        .observe(update_material_on::<Pointer<Up>>(hover_matl.clone()));

    game.unit.x = cylinder_pos.x as usize;
    game.unit.y = cylinder_pos.y as usize;
    game.unit.z = cylinder_pos.z as usize;

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(cylinder_pos.x, cylinder_pos.y + 3.0, cylinder_pos.z + 5.0)
            .looking_at(cylinder_pos, Vec3::Y),
    ));
}

const BOARD_SIZE_I: usize = 14;
const BOARD_SIZE_J: usize = 21;

#[derive(Resource, Deref, DerefMut)]
struct Random(ChaCha8Rng);

#[derive(Default, Resource)]
struct OccupiedScreenSpace {
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
}

fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    /*commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));*/
    let white_matl = materials.add(Color::WHITE);
    let ground_matl = materials.add(Color::from(GRAY_300));
    let hover_matl = materials.add(Color::from(CYAN_300));
    let pressed_matl = materials.add(Color::from(YELLOW_300));
    /*commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
            Transform::from_xyz(0.0, 0.5, 0.0),
        ))
        .observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
        .observe(update_material_on::<Pointer<Out>>(white_matl.clone()))
        .observe(update_material_on::<Pointer<Down>>(pressed_matl.clone()))
        .observe(update_material_on::<Pointer<Up>>(hover_matl.clone()))
        .observe(on_drag_rotate);
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));*/
}

fn ui_example_system(
    mut is_last_selected: Local<bool>,
    mut contexts: EguiContexts,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
    mut game: ResMut<Game<'static>>,
) {
    let ctx = contexts.ctx_mut();

    occupied_screen_space.left = egui::SidePanel::left("left_panel")
        .resizable(true)
        .show(ctx, |ui| {
            /*if ui
                .add(
                    egui::widgets::Button::new(game.board.len().to_string())
                        .selected(!*is_last_selected),
                )
                .clicked()
            {
                *is_last_selected = false;
            }
            if ui
                .add(egui::widgets::Button::new("Another button").selected(*is_last_selected))
                .clicked()
            {
                *is_last_selected = true;
            }*/
            ui.label("Effects");
            for effect in game.unit.effects.iter() {
                ui.label(format!("| {} | -> {}", effect.0, effect.1));
                ui.label(format!("{}", EFFECTS[effect.0].description));
            }
            ui.label("Auras");
            for aura in game.unit.auras.iter() {
                ui.label(format!("| {} |", *aura));
                ui.label(format!("{}", AURAS[*aura].description));
            }
            for effect in game.unit.effects.iter() {
                let fetched_effect = &EFFECTS[effect.0];
                for aura in fetched_effect.auras.iter() {
                    ui.label(format!("| {} |", *aura));
                    ui.label(format!("{}", AURAS[*aura].description));
                }
            }
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
    occupied_screen_space.right = egui::SidePanel::right("right_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Equipment");
            for equippable in game.unit.equipment.iter() {
                ui.label(format!("| {} |", equippable.0));
                ui.label(format!("{}", equippable.1.name));
                ui.label(format!(
                    "{} {}",
                    equippable.1.item_quality, equippable.1.item_rarity
                ));
            }
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();

    occupied_screen_space.top = egui::TopBottomPanel::top("top_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label(format!(
                "Name: {} | Health: {}/{} | Magic: {}/{}",
                game.unit.name,
                game.unit.current_health,
                game.unit.max_health,
                game.unit.current_magic,
                game.unit.max_magic
            ));
            ui.label(format!(
                "Constitution: {} | Strength: {} | Agility: {} | Intelligence: {}",
                game.unit.constitution,
                game.unit.strength,
                game.unit.agility,
                game.unit.intelligence
            ));
            ui.label(format!(
                "Initiative: {} | Movement: {} | Jump: {} | Accuracy: {} | Evasion: {} | Critical: {} | Critical Resist: {}",
                game.unit.initiative,
                game.unit.movement,
                game.unit.jump,
                game.unit.accuracy,
                game.unit.evasion,
                game.unit.critical,
                game.unit.critical_resist
            ));
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .height();
    occupied_screen_space.bottom = egui::TopBottomPanel::bottom("bottom_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Chat");
            ui.label("Player1: your mom plays candy crush");
            ui.label("GIGACHAD: i'll have you know that im a navy seal and i have over 300 confirmed kills");
            ui.label("Player1: oh my bad");
            ui.label("Player1: your mom plays farmville");
            ui.label("GIGACHAD: THAT'S IT");
            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut game.unit.name);
            });
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .height();
}

fn on_drag_rotate(drag: Trigger<Pointer<Drag>>, mut transforms: Query<&mut Transform>) {
    if let Ok(mut transform) = transforms.get_mut(drag.target) {
        transform.rotate_y(drag.delta.x * 0.02);
        transform.rotate_x(drag.delta.y * 0.02);
    }
}

fn update_material_on<E>(
    new_material: Handle<StandardMaterial>,
) -> impl Fn(Trigger<E>, Query<&mut MeshMaterial3d<StandardMaterial>>) {
    // An observer closure that captures `new_material`. We do this to avoid needing to write four
    // versions of this observer, each triggered by a different event and with a different hardcoded
    // material. Instead, the event type is a generic, and the material is passed in.
    move |trigger, mut query| {
        if let Ok(mut material) = query.get_mut(trigger.entity()) {
            material.0 = new_material.clone();
        }
    }
}

/// A system that draws hit indicators for every pointer.
fn draw_mesh_intersections(pointers: Query<&PointerInteraction>, mut gizmos: Gizmos) {
    for (point, normal) in pointers
        .iter()
        .filter_map(|interaction| interaction.get_nearest_hit())
        .filter_map(|(_entity, hit)| hit.position.zip(hit.normal))
    {
        gizmos.sphere(point, 0.05, RED_500);
        gizmos.arrow(point, point + normal.normalize() * 0.5, PINK_100);
    }
}

fn camera_drag_system(
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut mouse_wheel: EventReader<MouseWheel>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut last_pos: Local<Option<Vec2>>,

    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<PointerInteraction>)>,
    game: Res<Game<'static>>,
) {
    let location = Vec3::from([game.unit.x as f32, game.unit.y as f32, game.unit.z as f32]);
    if let Some(window) = windows.iter().next() {
        let cursor_pos = window.cursor_position();
        if mouse_button.pressed(MouseButton::Left) {
            if let (Some(mut transform), Some(pos), Some(new_pos)) =
                (camera_query.iter_mut().next(), *last_pos, cursor_pos)
            {
                let delta = new_pos - pos;
                transform.translation.x -= delta.x * 0.01;
                transform.translation.z += delta.y * 0.01;
            }
        }
        if mouse_button.pressed(MouseButton::Right) {
            if let (Some(mut transform), Some(pos), Some(new_pos)) =
                (camera_query.iter_mut().next(), *last_pos, cursor_pos)
            {
                // rotate the camera around the location
                let delta = new_pos - pos;
                transform.translation = Quat::from_rotation_y(-delta.x * 0.01)
                    * (transform.translation - location)
                    + location;
                transform.translation = Quat::from_rotation_x(-delta.y * 0.01)
                    * (transform.translation - location)
                    + location;
                // keep the translation from looking at the bottom of the board
                transform.translation.y =
                    (transform.translation - location).y.max(1.0) + location.y;

                transform.look_at(location, Vec3::Y);
            }
        }

        for event in mouse_wheel.read() {
            match event.unit {
                MouseScrollUnit::Line => {
                    println!(
                        "Scroll (line units): vertical: {}, horizontal: {}",
                        event.y, event.x
                    );
                    let mut transform = camera_query.iter_mut().next().unwrap();
                    // this should be how close to location the transform is
                    let distance = (transform.translation - location).length();
                    let transform_translation = transform.translation;
                    transform.translation +=
                        (transform_translation - location).normalize() * event.y * 0.1;
                    // prevent the camera from getting too close
                    if (transform.translation - location).length() < 1.0 {
                        transform.translation = transform_translation;
                    }
                }
                MouseScrollUnit::Pixel => {
                    println!(
                        "Scroll (pixel units): vertical: {}, horizontal: {}",
                        event.y, event.x
                    );
                }
            }
        }
        *last_pos = cursor_pos;
    }
}
