extern crate clap;
extern crate fillercore;

mod arguments;
use arguments::Arguments;

mod game_manager;
use game_manager::GameManager;

mod util;

use engine::Engine;
use fillercore::engine;

fn main() {
    let args = Arguments::new();

    let players = args.player_paths();

    if players.len() <= 2 {
        let mut builder = Engine::builder(&players[0]);
        if let Some(player2) = players.get(1) {
            builder.with_player2(player2);
        }

        if args.verbose() {
            builder.verbose();
        }

        let mut filler = builder.finish();

        filler.run();
        if let Some(json_dir) = args.json_path() {
            util::write_replay(json_dir, &filler);
        }
    } else {
        let mut gm = GameManager::new(players);
        if let Some(json_dir) = args.json_path() {
            gm.with_replays(json_dir);
        }
        gm.run();
    }
}
