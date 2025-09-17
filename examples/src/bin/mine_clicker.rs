//! Demonstrates how to use RNG relations with observer driven code. Adapted from bevy examples.

use bevy::{ecs::entity::EntityHashSet, platform::collections::HashMap, prelude::*};
use bevy_rand::prelude::*;
use rand::Rng;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EntropyPlugin::<WyRand>::new(),
            EntropyRelationsPlugin::<WyRand, WyRand>::default(),
        ))
        .init_resource::<SpatialIndex>()
        .add_systems(Startup, (initial_setup, observer_setup).chain())
        .add_systems(Update, (draw_shapes, handle_click))
        // Observers are systems that run when an event is "triggered". This observer runs whenever
        // `ExplodeMines` is triggered.
        .add_observer(explode_nearby_mines)
        // This observer runs whenever the `Mine` component is added to an entity, and places it in a simple spatial index.
        .add_observer(on_insert_mine_pos)
        // This observer runs whenever the `Mine` component is removed from an entity (including despawning it)
        // and removes it from the spatial index.
        .add_observer(on_replace_mine_pos)
        .add_observer(explode_mine)
        .run();
}

#[derive(Component)]
struct Mine;

#[derive(Component)]
struct Explosive {
    size: f32,
}

#[derive(Component)]
struct MinePos {
    pos: Vec2,
}

#[derive(Event)]
struct ExplodeMines {
    pos: Vec2,
    radius: f32,
}

#[derive(EntityEvent)]
struct Explode {
    entity: Entity,
}

fn initial_setup(mut commands: Commands, mut global_rng: GlobalRngEntity<WyRand>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Text::new(
            "Click on a \"Mine\" to trigger it. \
            Right clicking will re-randomise the remaining mines\n\
            When it explodes it will trigger all overlapping mines.",
        ),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.),
            left: Val::Px(12.),
            ..default()
        },
    ));

    global_rng
        .rng_commands()
        .with_target_rngs((1..=1000).map(|num| (Name::new(format!("Mine {num}")), Mine)));
}

fn observer_setup(
    query: Query<Entity, With<Mine>>,
    mut commands: Commands,
    mut global_rng: GlobalRngEntity<WyRand>,
) {
    let observer = Observer::new(on_init_mine);

    commands.spawn(observer.with_entities(query));

    global_rng.rng_commands().reseed_linked();
}

// Each mine has its own RNG state, which allows them not to rely on a global RNG source
// for any update
fn on_init_mine(
    trigger: On<Insert, WyRand>,
    mut query: Query<&mut WyRand, With<Mine>>,
    mut commands: Commands,
) {
    let target = trigger.entity;

    let mut rng = query.get_mut(target).unwrap();

    commands.entity(target).insert((
        MinePos {
            pos: Vec2::new(
                rng.random_range(-600.0..=600.0),
                rng.random_range(-325.0..=275.0),
            ),
        },
        Explosive {
            size: rng.random_range(4.0..=20.0),
        },
    ));
}

fn on_insert_mine_pos(
    trigger: On<Insert, MinePos>,
    query: Query<&MinePos>,
    mut index: ResMut<SpatialIndex>,
) {
    let mine = query.get(trigger.entity).unwrap();
    let tile = (
        (mine.pos.x / CELL_SIZE).floor() as i32,
        (mine.pos.y / CELL_SIZE).floor() as i32,
    );
    index.map.entry(tile).or_default().insert(trigger.entity);
}

// Clean up old mine data from our index before it is updated or if the mine is despawned
fn on_replace_mine_pos(
    trigger: On<Replace, MinePos>,
    query: Query<&MinePos>,
    mut index: ResMut<SpatialIndex>,
) {
    let mine = query.get(trigger.entity).unwrap();
    let tile = (
        (mine.pos.x / CELL_SIZE).floor() as i32,
        (mine.pos.y / CELL_SIZE).floor() as i32,
    );
    index.map.entry(tile).and_modify(|set| {
        set.remove(&trigger.entity);
    });
}

fn explode_nearby_mines(
    trigger: On<ExplodeMines>,
    mines: Query<(&MinePos, &Explosive)>,
    index: Res<SpatialIndex>,
    mut commands: Commands,
) {
    // You can access the trigger data via the `Observer`
    let event = trigger.event();
    // Access resources
    for entity in index.get_nearby(event.pos) {
        // Run queries
        let (mine, explosive) = mines.get(entity).unwrap();
        if mine.pos.distance(event.pos) < explosive.size + event.radius {
            // And queue commands, including triggering additional events
            // Here we trigger the `Explode` event for entity `e`
            commands.trigger(Explode { entity });
        }
    }
}

fn explode_mine(
    trigger: On<Explode>,
    query: Query<(&MinePos, &Explosive, &Name)>,
    mut commands: Commands,
) {
    // If a triggered event is targeting a specific entity you can access it with `.entity()`
    let id = trigger.entity;
    let Ok(mut entity) = commands.get_entity(id) else {
        return;
    };
    entity.despawn();
    let (mine, explosive, name) = query.get(id).unwrap();
    info!("Boom! {} exploded.", name);
    // Trigger another explosion cascade.
    commands.trigger(ExplodeMines {
        pos: mine.pos,
        radius: explosive.size,
    });
}

// Draw a circle for each mine using `Gizmos`
fn draw_shapes(mut gizmos: Gizmos, mines: Query<(&MinePos, &Explosive)>) {
    for (mine, explosive) in &mines {
        gizmos.circle_2d(
            mine.pos,
            explosive.size,
            Color::hsl((explosive.size - 4.0) / 16.0 * 360.0, 1.0, 0.8),
        );
    }
}

// Trigger `ExplodeMines` at the position of a given click, or re-randomise all positions
fn handle_click(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut commands: Commands,
    mut global_rng: GlobalRngEntity<WyRand>,
) {
    let Ok(windows) = windows.single() else {
        return;
    };

    if mouse_button_input.just_pressed(MouseButton::Right) {
        // One line to automatically push updates
        global_rng.rng_commands().reseed_linked();
        info!("Reseeding remaining mines with new RNG states");
        return;
    }

    let (camera, camera_transform) = *camera;
    if let Some(pos) = windows
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
        && mouse_button_input.just_pressed(MouseButton::Left)
    {
        commands.trigger(ExplodeMines { pos, radius: 1.0 });
    }
}

#[derive(Resource, Default)]
struct SpatialIndex {
    map: HashMap<(i32, i32), EntityHashSet>,
}

/// Cell size has to be bigger than any `TriggerMine::radius`
const CELL_SIZE: f32 = 64.0;

impl SpatialIndex {
    // Lookup all entities within adjacent cells of our spatial index.
    fn get_nearby(&self, pos: Vec2) -> Vec<Entity> {
        let tile = (
            (pos.x / CELL_SIZE).floor() as i32,
            (pos.y / CELL_SIZE).floor() as i32,
        );

        let mut nearby = Vec::new();
        for x in -1..2 {
            for y in -1..2 {
                if let Some(mines) = self.map.get(&(tile.0 + x, tile.1 + y)) {
                    nearby.extend(mines.iter());
                }
            }
        }
        nearby
    }
}
