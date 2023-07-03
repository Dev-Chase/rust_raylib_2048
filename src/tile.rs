use crate::{KeyStates, H, MOVE_SPEED, TILE_FONT_SIZE, TILE_SIZE, W, WINDOW_RECT};

use rand::Rng;
pub use raylib::prelude::*;

// Function for Getting Tile Colour
fn tile_colours(val: i32) -> [Color; 2] {
    match val {
        2 => [Color::LIGHTGRAY, Color::BLACK],
        4 => [Color::BEIGE, Color::BLACK],
        8 | 16 => [Color::ORANGE, Color::WHITE],
        32 | 64 => [Color::RED, Color::WHITE],
        _ => [Color::new(255, 209, 32, 255), Color::WHITE],
    }
}

// Function for Getting Vector Center
pub struct TileMap {
    list: Vec<GameTile>,
    dir: Vector2,
    done_moving: bool,
}

impl TileMap {
    // Function for Creating a New Instance of TileMap
    pub fn new() -> TileMap {
        TileMap {
            list: Vec::new(),
            dir: Vector2::zero(),
            done_moving: true,
        }
    }

    // Function for Resetting TileMap
    pub fn start(&mut self) {
        self.dir = Vector2::zero();
        self.done_moving = true;
        self.list.clear();
        self.gen_new_tile();
        self.gen_new_tile();
    }

    // Function for Generating and Placing a New Tile
    fn gen_new_tile(&mut self) {
        // Create New Tile
        let mut new_tile = GameTile::new(
            match rand::thread_rng().gen_ratio(1, 10) {
                true => 4,
                false => 2,
            },
            self.dir,
        );

        // Move the Tile to a Random Spot on the Map
        new_tile.random_spot();

        // Moving the New Tile if and while it's Colliding with Another Tile
        while self
            .list
            .iter()
            .any(|tile| new_tile.rect.x == tile.rect.x && new_tile.rect.y == tile.rect.y)
        {
            new_tile.random_spot();
        }

        // Insert Tile into List
        self.list.push(new_tile);
    }

    // Updating Functions (Run Every Frame)
    pub fn update(&mut self, key_states: &KeyStates) {
        // Updating the Done Moving Variable so that Tiles don't Continually Spawn but keeping Beginning Position in account
        self.done_moving =
            self.list.iter().all(|tile| tile.at_target) || self.dir == Vector2::zero();

        // Input Handling
        if self.done_moving && key_states.any_movement() {
            // Changing the Direction that the tiles are Facing based on User Input
            if key_states.up {
                self.dir.x = 0.0;
                self.dir.y = -1.0;
            } else if key_states.down {
                self.dir.x = 0.0;
                self.dir.y = 1.0;
            } else if key_states.right {
                self.dir.x = 1.0;
                self.dir.y = 0.0;
            } else if key_states.left {
                self.dir.x = -1.0;
                self.dir.y = 0.0;
            }
            for tile in self.list.iter_mut() {
                tile.target = self.dir;
                tile.at_target = false;
            }

            // Check if the Tiles can Move before setting them in motion
            self.done_moving = (0..self.list.len()).all(|i| self.tile_at_target(i));
        }

        // Updating Tiles
        if !self.done_moving {
            // Move every Tile that isn't done moving/at target
            for i in 0..self.list.len() {
                if !self.tile_at_target(i) {
                    self.list[i].rect.x += self.list[i].target.x * MOVE_SPEED;
                    self.list[i].rect.y += self.list[i].target.y * MOVE_SPEED;
                }
            }

            // Updating done_moving Attribute and at_target Attributes
            self.done_moving = (0..self.list.len()).all(|i| self.tile_at_target(i));

            // If done_moving: Merge Colliding Tiles & Generate a New One
            if self.done_moving {
                // While Looping through list of Tiles in Order to Remove in Place
                let mut i = 0;
                while i < self.list.len() {
                    // If there is a collision between two Tiles that aren't the same Tile
                    if let Some(ind) = self.list.iter().enumerate().position(|(tile_i, tile)| {
                        tile_i != i && tile.rect.check_collision_recs(&self.list[i].rect)
                    }) {
                        // Double the Value of the Second Tile
                        self.list[ind].val *= 2;

                        // Delete the First Tile
                        self.list.remove(i);
                    } else {
                        // Only changing the Index if not removing tiles/altering length of List
                        i += 1;
                    }
                }

                // Generate a New Tile
                self.gen_new_tile();

                // Updating done_moving Attribute and at_target Attributes
                self.done_moving = (0..self.list.len()).all(|i| self.tile_at_target(i));
            }
        }
    }

    // Function for Determining and Asigning At_Target States to Tiles
    fn tile_at_target(&mut self, i: usize) -> bool {
        // Get the Indecies of all Tiles that the Requested Tile will Collide with
        let collided_indices: Vec<usize> = (0..self.list.len())
            .filter(|tile_i| {
                *tile_i != i
                    && self.list[*tile_i]
                        .rect
                        .check_collision_point_rec(self.list[i].future_point())
                    && self.tile_at_target(*tile_i)
            })
            .collect::<Vec<usize>>();

        // If Next Position is Out of Bounds or In Collision with Multiple Tiles or One Tile that has a different Value
        if !WINDOW_RECT.check_collision_point_rec(self.list[i].future_point())
            || collided_indices.len() > 1
            || collided_indices
                .iter()
                .any(|tile_i| self.list[*tile_i].val != self.list[i].val)
        {
            self.list[i].at_target = true;
            return true;
        }

        // Default to Returning False
        false
    }

    // Function for Getting Current Score
    pub fn get_score(&self) -> i32 {
        self.list.iter().map(|tile| tile.val).max().unwrap()
    }

    // Game Over Check
    pub fn is_game_over(&self) -> bool {
        (self.list.len() as i32) >= W / TILE_SIZE * H / TILE_SIZE
            && self.list.iter().all(|tile| tile.at_target)
    }

    // Drawing Function
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        for tile in &self.list {
            d.draw_rectangle_rec(tile.rect, tile_colours(tile.val)[0]);
            d.draw_text(
                &format!("{}", tile.val),
                (tile.rect.x + tile.rect.width / 2f32) as i32
                    - measure_text(format!("{}", tile.val).as_str(), TILE_FONT_SIZE) / 2,
                (tile.rect.y + tile.rect.height / 2f32) as i32 - TILE_FONT_SIZE / 2,
                TILE_FONT_SIZE,
                tile_colours(tile.val)[1],
            );
        }
    }
}

// Struct to Hold Data for Every Game Tile
struct GameTile {
    rect: Rectangle,
    val: i32,
    target: Vector2,
    at_target: bool,
}

impl GameTile {
    // Function for Creating a New Tile Instance
    fn new(val: i32, target: Vector2) -> GameTile {
        GameTile {
            rect: rrect(0, 0, TILE_SIZE, TILE_SIZE),
            val,
            target,
            at_target: true,
        }
    }

    // Function for Getting where the Tile Will be Next Step
    fn future_point(&self) -> Vector2 {
        Vector2::new(
            (self.rect.x + self.rect.width / 2f32)
                + ((self.rect.width / 2f32 + 1f32) * self.target.x),
            (self.rect.y + self.rect.height / 2f32)
                + ((self.rect.height / 2f32 + 1f32) * self.target.y),
        )
    }

    // Function for Getting a Random Tile Location on the Map
    fn random_spot(&mut self) {
        self.rect.x = (rand::thread_rng().gen_range(0..W / TILE_SIZE) * TILE_SIZE) as f32;
        self.rect.y = (rand::thread_rng().gen_range(0..H / TILE_SIZE) * TILE_SIZE) as f32;
    }
}
