use super::engine::Engine;

pub struct GameManager {
	players: Vec<String>,
}

// Panics with less than 3 players
impl GameManager {
	pub fn new(players: Vec<String>) -> Self {
		GameManager { players }
	}

	pub fn run(&self) {
		let players_iter = self.players.iter();

		for (i, player1) in players_iter.enumerate() {
			let opponent = self.players.iter().skip(i + 1);
			for player2 in opponent {
				let mut game_builder = Engine::builder(player1);
				game_builder.with_player2(player2);
				let mut game = game_builder.finish();
				game.run();
			}
		}
	}
}
