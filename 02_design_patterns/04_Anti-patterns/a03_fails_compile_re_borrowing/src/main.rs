use std::collections::HashMap;

type PlayerID = i32;

#[derive(Debug, Default)]
struct Player {
    score: i32,
}

fn start_game(player_a: PlayerID, player_b: PlayerID, server: &mut HashMap<PlayerID, Player>) {
    server.entry(player_a).or_default();
    server.entry(player_b).or_default();

    let player_a = server.get(&player_a);
    let player_b = server.get(&player_b);

    dbg!(player_a, player_b);
}
fn main() {
    let mut server: HashMap<PlayerID, Player> = HashMap::new();

    start_game(100, 90, &mut server);
}
