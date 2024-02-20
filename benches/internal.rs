use bevy_rand::BENCH;

fn main() {
    // MacOS workaround T___T
    let _ = BENCH;

    divan::main();
}
