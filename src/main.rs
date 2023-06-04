use breakout::game::Game;

const WIDTH:u32=1280;
const HEIGHT:u32=720;

fn main() {
    let mut game = Game::new(WIDTH,HEIGHT);
    game.init();
    game.run();
}