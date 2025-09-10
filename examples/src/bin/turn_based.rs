//! A turn-based example with bevy_rand relations and observer event propagation, where
//! different entities contain their own RNG state instead of depending on non-deterministic
//! sources, and are seeded via relationships. Adapted from bevy examples.
#![allow(clippy::type_complexity)]

use std::time::Duration;

use bevy::{
    ecs::relationship::RelatedSpawner, log::LogPlugin, prelude::*,
    time::common_conditions::on_timer,
};
use bevy_rand::prelude::*;
use rand::{
    Rng,
    seq::{IteratorRandom, SliceRandom},
};

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins,
            LogPlugin::default(),
            EntropyPlugin::<WyRand>::with_seed(42u64.to_ne_bytes()),
            EntropyRelationsPlugin::<WyRand, WyRand>::default(),
        ))
        .add_systems(Startup, (character_setup, observer_setup).chain())
        .add_systems(
            Update,
            next_turn.run_if(on_timer(Duration::from_millis(200))),
        )
        // Add a global observer that will emit a line whenever an attack hits an entity.
        .add_observer(track_hits)
        .run();
}

#[derive(Component, PartialEq, Eq, Clone, Copy)]
enum Kind {
    Player,
    Enemy,
}

#[derive(Component)]
struct Character;

/// An entity that can take damage.
#[derive(Component)]
struct Health {
    points: u16,
}

/// For damage to reach the wearer, it must exceed the armor.
#[derive(Component)]
struct Armor {
    rating: u16,
}

// This event represents an attack we want to "bubble" up from the armor to the character.
//
// We enable propagation by adding the event attribute and specifying two important pieces of information.
//
// - **traversal:**
// Which component we want to propagate along. In this case, we want to "bubble" (meaning propagate
// from child to parent) so we use the `ChildOf` component for propagation. The component supplied
// must implement the `Traversal` trait.
//
// - **auto_propagate:**
// We can also choose whether or not this event will propagate by default when triggered. If this is
// false, it will only propagate following a call to `On::propagate(true)`.
#[derive(Clone, Component, EntityEvent)]
#[entity_event(propagate = &'static ChildOf, auto_propagate)]
struct Attack {
    #[event_target]
    target: Entity,
    damage: u16,
}

// This event kicks off whether a character attacks a target, and contains the state needed
// for a particular character to engage that attack
#[derive(Clone, Component, EntityEvent)]
struct Turn {
    entity: Entity,
    rng: WyRand,
    character: Name,
    kind: Kind,
}

// In this example, we spawn characters wearing different pieces of armor. Each piece of armor
// is represented as a child entity, with an `Armor` component. Each character is either a player
// character or enemy characters (goblins), and each take turns attack the other.
//
// We're going to model how attack damage can be partially blocked by the character's armor using
// event bubbling. Our events will target the armor, and if the armor isn't strong enough to block
// the attack it will continue up and hit the character.
fn character_setup(mut global_rng: GlobalRngEntity<WyRand>) {
    let mut global_rng = global_rng.rng_commands();

    let child_spawner = |parent: &mut RelatedSpawner<ChildOf>| {
        parent.spawn((
            Name::new("Helmet"),
            Armor { rating: 5 },
            RngSource::<WyRand, WyRand>::new(parent.target_entity()),
        ));
        parent.spawn((
            Name::new("Socks"),
            Armor { rating: 10 },
            RngSource::<WyRand, WyRand>::new(parent.target_entity()),
        ));
        parent.spawn((
            Name::new("Shirt"),
            Armor { rating: 15 },
            RngSource::<WyRand, WyRand>::new(parent.target_entity()),
        ));
    };

    global_rng
        .with_target_rngs([
            (
                Name::new("Player"),
                Health { points: 60 },
                Character,
                Kind::Player,
                RngLinks::<WyRand, WyRand>::default(),
                Children::spawn(SpawnWith(child_spawner)),
            ),
            (
                Name::new("Goblin 1"),
                Health { points: 25 },
                Character,
                Kind::Enemy,
                RngLinks::<WyRand, WyRand>::default(),
                Children::spawn(SpawnWith(child_spawner)),
            ),
            (
                Name::new("Goblin 2"),
                Health { points: 25 },
                Character,
                Kind::Enemy,
                RngLinks::<WyRand, WyRand>::default(),
                Children::spawn(SpawnWith(child_spawner)),
            ),
        ])
        .reseed_linked();
}

fn observer_setup(
    characters: Query<Entity, With<Character>>,
    armor: Query<Entity, With<Armor>>,
    mut commands: Commands,
) {
    let turn = Observer::new(attack_target);
    let damage = Observer::new(take_damage);
    let block = Observer::new(block_attack);

    commands.spawn_batch([
        damage.with_entities(characters.iter()),
        turn.with_entities(characters),
        block.with_entities(armor),
    ]);
}

/// Calculate the order of attacks each Character will take during this next turn.
fn next_turn(
    mut characters: Query<
        (Entity, &mut Entropy<WyRand>, &Name, &Kind),
        (With<Character>, Without<GlobalRng>),
    >,
    mut global: Single<&mut Entropy<WyRand>, (With<GlobalRng>, Without<Character>)>,
    mut commands: Commands,
    mut global_rng: GlobalRngEntity<WyRand>,
) {
    info!("Next turn!\n");

    let mut order: Vec<_> = characters.iter_mut().collect();

    order.shuffle(&mut global);

    order
        .into_iter()
        .for_each(|(entity, mut rng, character, &kind)| {
            // Each Character gets a turn event that sends their needed state to be used
            // to calculate their attack.
            commands.trigger(Turn {
                entity,
                rng: rng.fork_inner(),
                kind,
                character: character.clone(),
            })
        });

    // Do note *when* the events kick in. This event will take effect *after* the turn
    // has completed, even if you were to place it at the top of the function.
    global_rng.rng_commands().reseed_linked();
}

/// An observer system that takes a charater's turn to attack a piece of the enemy's armor. Who is attacked
/// and which armor piece is attacked is decided by the character's RNG state.
fn attack_target(
    mut trigger: On<Turn>,
    characters: Query<(&Kind, &Name, &Children), With<Character>>,
    armor_pieces: Query<Entity, With<Armor>>,
    mut commands: Commands,
) {
    let target_kind = trigger.kind;

    // Check if character has been killed already.
    if !characters.contains(trigger.entity) {
        return;
    }

    // This probably could be better, but it works and doesn't cause issues
    if let Some(chosen) = characters
        .iter()
        .filter(|enemy| *enemy.0 != target_kind)
        .choose(&mut trigger.rng)
        && let Some(target) = chosen
            .2
            .iter()
            .choose(&mut trigger.rng)
            .and_then(|piece| armor_pieces.get(piece).ok())
    {
        let damage = trigger.rng.random_range(1..20);
        commands.trigger(Attack { target, damage });
        info!(
            "⚔️  {} Attacked {} for {} damage",
            trigger.character, chosen.1, damage
        );
    }
}

fn track_hits(trigger: On<Attack>, name: Query<&Name>) {
    if let Ok(name) = name.get(trigger.target) {
        info!("Attack hit {}", name);
    }
}

/// A callback placed on [`Armor`], checking if the blow glanced off or if it absorbed all the [`Attack`] damage.
/// Here, the Armor has its own RNG state to calculate whether a blow glances off it, not relying on the parent RNG state.
fn block_attack(mut trigger: On<Attack>, mut armor: Query<(&mut Entropy<WyRand>, &Armor, &Name)>) {
    if let Ok((mut rng, armor, name)) = armor.get_mut(trigger.target) {
        let attack = trigger.event_mut();
        let glance = rng.random_bool(0.1);

        // The attack has a chance to glance off the armor, dealing no damage to the target
        if glance {
            info!("🛡️  Attack glanced off {}, no damage done\n", name);
            trigger.propagate(false);
            return;
        }

        let damage = attack.damage.saturating_sub(armor.rating);
        if damage > 0 {
            info!("🩸 {} damage passed through {}", damage, name);
            // The attack isn't stopped by the armor. We reduce the damage of the attack, and allow
            // it to continue on to the goblin.
            attack.damage = damage;
        } else {
            info!("🛡️  {} damage blocked by {}", attack.damage, name);
            // Armor stopped the attack, the event stops here.
            trigger.propagate(false);
            info!("(propagation halted early)\n");
        }
    }
}

/// A callback on the armor wearer, triggered when a piece of armor is not able to block an attack,
/// or the wearer is attacked directly. The simulation ends when any one of the characters die.
fn take_damage(
    trigger: On<Attack>,
    mut hp: Query<(&mut Health, &Name)>,
    mut commands: Commands,
    mut app_exit: EventWriter<AppExit>,
) {
    let attack = trigger.event();
    let (mut health, name) = hp.get_mut(trigger.target).unwrap();
    health.points = health.points.saturating_sub(attack.damage);

    if health.points > 0 {
        info!("{} has {:.1} HP", name, health.points);
    } else {
        warn!("💀 {} has died a gruesome death", name);
        commands.entity(trigger.target).despawn();
        app_exit.write(AppExit::Success);
    }

    info!("(propagation reached root)\n");
}
