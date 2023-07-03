pub use raylib::prelude::*;
mod tile;
use tile::TileMap;

const BG_COLOUR: Color = Color::BLACK;
const FG_COLOUR: Color = Color::WHITE;
const TILE_SIZE: i32 = 125;
const MOVE_SPEED: f32 = TILE_SIZE as f32 / 4f32;
const W: i32 = TILE_SIZE * 4;
const H: i32 = TILE_SIZE * 4;
const FPS: u32 = 60u32;
const TITLE_FONT_SIZE: i32 = 26;
const TILE_FONT_SIZE: i32 = 24;
const WINDOW_RECT: Rectangle = Rectangle::new(0f32, 0f32, W as f32, H as f32);

enum GameState {
    Starting,
    Playing,
    GameOver,
    Paused,
}

pub struct KeyStates {
    up: bool,
    down: bool,
    right: bool,
    left: bool,
    space: bool,
    r: bool,
}

impl KeyStates {
    fn new() -> KeyStates {
        KeyStates {
            up: false,
            down: false,
            right: false,
            left: false,
            space: false,
            r: false,
        }
    }

    pub fn any_movement(&self) -> bool {
        self.up || self.down || self.right || self.left
    }
}

fn main() {
    let (mut rl, thread) = init().size(W, H).title("2048 - Rust & Raylib").build();
    rl.set_target_fps(FPS);

    // Game Variables
    let mut current_main_text = "Press Space to Start";
    let mut subtitle_text = "";
    let mut score_text = String::from("");
    let mut game_state = GameState::Starting;

    let mut key_states = KeyStates::new();

    // Create Sprites
    let mut tile_map: TileMap = TileMap::new();

    // Game Loop
    while !rl.window_should_close() {
        // Get Input Info
        key_states.space = rl.is_key_pressed(KeyboardKey::KEY_SPACE);
        key_states.down = rl.is_key_pressed(KeyboardKey::KEY_DOWN);
        key_states.up = rl.is_key_pressed(KeyboardKey::KEY_UP);
        key_states.left = rl.is_key_pressed(KeyboardKey::KEY_LEFT);
        key_states.right = rl.is_key_pressed(KeyboardKey::KEY_RIGHT);
        key_states.r = rl.is_key_pressed(KeyboardKey::KEY_R);

        // Updating
        match game_state {
            GameState::Starting | GameState::GameOver => {
                // Start and Initialize Game if Space Key is Pressed
                if key_states.space {
                    game_state = GameState::Playing;
                    tile_map.start();
                }
            }
            GameState::Playing => {
                // Updating Sprites
                tile_map.update(&key_states);
                score_text = format!("Score: {}", tile_map.get_score());

                // Pause Game Check
                if key_states.space {
                    game_state = GameState::Paused;
                    current_main_text = "Press Space to Resume";
                    subtitle_text = "Game Paused";
                }

                // Game Over Checks
                if tile_map.is_game_over() || key_states.r {
                    game_state = GameState::GameOver;
                    current_main_text = "Press Space to Restart";
                    subtitle_text = "Game Over";
                }
            }
            GameState::Paused => {
                // Resume Game Checks
                if key_states.space {
                    game_state = GameState::Playing;
                }
            }
        }

        // Drawing
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(BG_COLOUR);

        match game_state {
            GameState::Starting | GameState::GameOver | GameState::Paused => {
                // Drawing Main, Subtitle, and Score Text
                d.draw_text(
                    current_main_text,
                    W / 2 - measure_text(current_main_text, TITLE_FONT_SIZE) / 2,
                    H / 2 - TITLE_FONT_SIZE / 2,
                    TITLE_FONT_SIZE,
                    FG_COLOUR,
                );
                d.draw_text(
                    subtitle_text,
                    W / 2 - measure_text(subtitle_text, TITLE_FONT_SIZE - 2) / 2,
                    H / 2 - TITLE_FONT_SIZE / 2 - (TITLE_FONT_SIZE - 2) - 4,
                    TITLE_FONT_SIZE,
                    FG_COLOUR,
                );
                d.draw_text(score_text.as_str(), 5, 5, TITLE_FONT_SIZE - 4, FG_COLOUR);
            }
            GameState::Playing => {
                // Draw Sprites
                tile_map.draw(&mut d);
            }
        }
    }
}
