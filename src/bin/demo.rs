use setgame::Game;

fn main() {
    let game = Game::new();
    game.print_board();

    for set in game.sets() {
        println!("{}{}{}", set[0], set[1], set[2]);
    }
}
