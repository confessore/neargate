use neargate::{Equippable, Item, JobType, Spell, Unit, AURAS, CONSUMABLES, SPELLS};
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
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .enable_state_scoped_entities::<GameState>()
        .add_systems(Startup, setup_cameras)
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut game: ResMut<Game>) {
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

    // spawn the game board
    let cell_scene =
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("tile.glb"));
    game.board = (0..BOARD_SIZE_J)
        .map(|j| {
            (0..BOARD_SIZE_I)
                .map(|i| {
                    let height = rng.gen_range(-0.1..0.1);
                    commands.spawn((
                        StateScoped(GameState::Playing),
                        Transform::from_xyz(i as f32, height - 0.2, j as f32),
                        SceneRoot(cell_scene.clone()),
                    ));
                    Cell { height }
                })
                .collect()
        })
        .collect();
    commands.insert_resource(Random(rng));
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum GameState {
    #[default]
    Playing,
    GameOver,
}

const BOARD_SIZE_I: usize = 14;
const BOARD_SIZE_J: usize = 21;

const RESET_FOCUS: [f32; 3] = [
    BOARD_SIZE_I as f32 / 2.0,
    0.0,
    BOARD_SIZE_J as f32 / 2.0 - 0.5,
];

struct Cell {
    height: f32,
}

#[derive(Resource, Default)]
struct Game {
    board: Vec<Vec<Cell>>,
    camera_should_focus: Vec3,
    camera_is_focus: Vec3,
}

#[derive(Resource, Deref, DerefMut)]
struct Random(ChaCha8Rng);