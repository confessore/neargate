use bevy::{color::palettes::basic::*, prelude::*, ui};
use neargate::{
    Cell, Equippable, Game, GameState, Item, JobType, Spell, Unit, AURAS, CONSUMABLES, SPELLS,
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy::{prelude::*, color::palettes::tailwind::*, picking::pointer::PointerInteraction, window::PrimaryWindow, winit::WinitSettings};

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
        .add_systems(Update, update_camera_transform_system)
        .add_systems(OnEnter(GameState::Playing), setup)
        .run();
}

fn setup_cameras(mut commands: Commands, mut game: ResMut<Game>) {
    game.camera_should_focus = Vec3::from(RESET_FOCUS);
    game.camera_is_focus = game.camera_should_focus;
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(
            -(BOARD_SIZE_I as f32 / 2.0),
            2.0 * BOARD_SIZE_J as f32 / 3.0,
            BOARD_SIZE_J as f32 / 2.0 - 0.5,
        )
        .looking_at(game.camera_is_focus, Vec3::Y),
    ));
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>,asset_server: Res<AssetServer>, mut game: ResMut<Game>, mut materials: ResMut<Assets<StandardMaterial>>) {
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
                    commands.spawn((
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
}

const BOARD_SIZE_I: usize = 14;
const BOARD_SIZE_J: usize = 21;

const RESET_FOCUS: [f32; 3] = [
    BOARD_SIZE_I as f32 / 2.0,
    0.0,
    BOARD_SIZE_J as f32 / 2.0 - 0.5,
];

#[derive(Resource, Deref, DerefMut)]
struct Random(ChaCha8Rng);

#[derive(Default, Resource)]
struct OccupiedScreenSpace {
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
}

const CAMERA_TARGET: Vec3 = Vec3::ZERO;

#[derive(Resource, Deref, DerefMut)]
struct OriginalCameraTransform(Transform);


fn update_camera_transform_system(
    occupied_screen_space: Res<OccupiedScreenSpace>,
    original_camera_transform: Res<OriginalCameraTransform>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<(&Projection, &mut Transform)>,
) {
    let (camera_projection, mut transform) = match camera_query.get_single_mut() {
        Ok((Projection::Perspective(projection), transform)) => (projection, transform),
        _ => unreachable!(),
    };

    let distance_to_target = (CAMERA_TARGET - original_camera_transform.translation).length();
    let frustum_height = 2.0 * distance_to_target * (camera_projection.fov * 0.5).tan();
    let frustum_width = frustum_height * camera_projection.aspect_ratio;

    let window = windows.single();

    let left_taken = occupied_screen_space.left / window.width();
    let right_taken = occupied_screen_space.right / window.width();
    let top_taken = occupied_screen_space.top / window.height();
    let bottom_taken = occupied_screen_space.bottom / window.height();
    transform.translation = original_camera_transform.translation
        + transform.rotation.mul_vec3(Vec3::new(
            (right_taken - left_taken) * frustum_width * 0.5,
            (top_taken - bottom_taken) * frustum_height * 0.5,
            0.0,
        ));
}

fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));
    let white_matl = materials.add(Color::WHITE);
    let ground_matl = materials.add(Color::from(GRAY_300));
    let hover_matl = materials.add(Color::from(CYAN_300));
    let pressed_matl = materials.add(Color::from(YELLOW_300));
    commands.spawn((
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
    ));

    let camera_pos = Vec3::new(-2.0, 2.5, 5.0);
    let camera_transform =
        Transform::from_translation(camera_pos).looking_at(CAMERA_TARGET, Vec3::Y);
    commands.insert_resource(OriginalCameraTransform(camera_transform));

    commands.spawn((Camera3d::default(), camera_transform));
}

fn ui_example_system(
    mut is_last_selected: Local<bool>,
    mut contexts: EguiContexts,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
    mut game: ResMut<Game>
) {
    let ctx = contexts.ctx_mut();

    occupied_screen_space.left = egui::SidePanel::left("left_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Left resizeable panel");
            if ui
                .add(egui::widgets::Button::new(game.board.len().to_string()).selected(!*is_last_selected))
                .clicked()
            {
                *is_last_selected = false;
            }
            if ui
                .add(egui::widgets::Button::new("Another button").selected(*is_last_selected))
                .clicked()
            {
                *is_last_selected = true;
            }
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
    occupied_screen_space.right = egui::SidePanel::right("right_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Right resizeable panel");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
    occupied_screen_space.top = egui::TopBottomPanel::top("top_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Top resizeable panel");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .height();
    occupied_screen_space.bottom = egui::TopBottomPanel::bottom("bottom_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Bottom resizeable panel");
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

/// A marker component for our shapes so we can query them separately from the ground plane.
#[derive(Component)]
struct Shape;

const SHAPES_X_EXTENT: f32 = 14.0;
const EXTRUSION_X_EXTENT: f32 = 16.0;
const Z_EXTENT: f32 = 5.0;