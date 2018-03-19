extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

pub struct Game {
    gl: GlGraphics,
    board: Board,
    running: bool,
}

impl Game {
    fn render(&mut self, arg: &RenderArgs) {
        use graphics;

        // let white: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        let black: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(black, gl);
        });

        self.board.render(&mut self.gl, arg);
    }

    fn update(&mut self, arg: &UpdateArgs) -> bool {
        if !self.running {
            return true;
        }

        self.board.update(arg);

        true
    }

    fn input(&mut self, btn: &Button, mouse_pos: Option<&[f64; 2]>) {
        if btn == &Button::Keyboard(Key::Return) {
            self.running = !self.running;
        }

        if let Some(m) = mouse_pos {
            if btn == &Button::Mouse(MouseButton::Left) {
                let x_across = m[0] / self.board.scale;
                let y_down = m[1] / self.board.scale;

                self.board.tiles[x_across as usize][y_down as usize] =
                    match self.board.tiles[x_across as usize][y_down as usize] {
                        Tile::Alive => Tile::Dead,
                        Tile::Dead => Tile::Alive,
                    };
            }
        }
    }
}

struct Board {
    tiles: Vec<Vec<Tile>>,
    scale: f64,
    tile_width: i32,
    tile_height: i32,
}

impl Board {
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        // println!("Board Render");

        let white: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        let black: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let mut squares: Vec<Vec<graphics::types::Rectangle>> = Vec::new();

        for x in 0..self.tiles.len() {
            squares.push(Vec::new());

            for y in 0..self.tiles[x].len() {
                squares[x].push(graphics::rectangle::square(
                    x as f64 * self.scale as f64,
                    y as f64 * self.scale as f64,
                    self.scale,
                ));
            }
        }

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            for x in 0..self.tiles.len() {
                for y in 0..self.tiles[x].len() {
                    let new_tile = self.tiles[x][y];

                    let color = match new_tile {
                        Tile::Alive => white,
                        Tile::Dead => black,
                    };

                    graphics::rectangle(color, squares[x][y], transform, gl)
                }
            }
        });

        // println!("{:?}", self.tiles);
    }

    fn update(&mut self, _args: &UpdateArgs) -> bool {
        let mut new_tiles: Vec<Vec<Tile>> = Vec::new();

        for x_across in 0..self.tiles.len() {
            new_tiles.push(Vec::new());

            for y_down in 0..self.tiles[x_across].len() {
                let adjacent_tiles = self.get_adjacent_tiles(x_across as i32, y_down as i32);

                let new_tile = match adjacent_tiles {
                    x if x < 2 => Tile::Dead,
                    x if (x == 2 || x == 3) && self.tiles[x_across][y_down] != Tile::Dead => {
                        Tile::Alive
                    }
                    x if x > 3 => Tile::Dead,
                    x if x == 3 && self.tiles[x_across][y_down] == Tile::Dead => Tile::Alive,
                    _ => Tile::Dead,
                };

                new_tiles[x_across].push(new_tile);
            }
        }

        self.tiles = new_tiles;

        true
    }

    fn get_adjacent_tiles(&self, x_across: i32, y_down: i32) -> i32 {
        let mut adjacent: i32 = 0;

        if x_across > 0 {
            //Top left
            if y_down > 0 && self.tiles[x_across as usize - 1][y_down as usize - 1] == Tile::Alive {
                adjacent += 1;
            }

            //Direct Left
            if self.tiles[x_across as usize - 1][y_down as usize] == Tile::Alive {
                adjacent += 1;
            }

            //Bottom Left
            if y_down < self.tile_height - 1
                && self.tiles[x_across as usize - 1][y_down as usize + 1] == Tile::Alive
            {
                adjacent += 1;
            }
        }

        if x_across < self.tile_width - 1 {
            //Top Right
            if y_down > 0 && self.tiles[x_across as usize + 1][y_down as usize - 1] == Tile::Alive {
                adjacent += 1;
            }

            //Direct Right
            if self.tiles[x_across as usize + 1][y_down as usize] == Tile::Alive {
                adjacent += 1;
            }
            //Bottom Right
            if y_down < self.tile_height - 1
                && self.tiles[x_across as usize + 1][y_down as usize + 1] == Tile::Alive
            {
                adjacent += 1;
            }
        }

        //Direct Top
        if y_down > 0 && self.tiles[x_across as usize][y_down as usize - 1] == Tile::Alive {
            adjacent += 1;
        }

        //Direct Bottom
        if y_down < self.tile_height - 1
            && self.tiles[x_across as usize][y_down as usize + 1] == Tile::Alive
        {
            adjacent += 1;
        }

        adjacent
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Alive,
    Dead,
}

fn main() {
    let opengl = OpenGL::V3_2;

    let window_width = 1000;
    let window_height = 750;
    let scale = 10;

    let mut window: Window = WindowSettings::new("game-of-life", [window_width, window_height])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut starting_tiles = Vec::new();

    for width in 0..window_width / scale {
        starting_tiles.push(Vec::with_capacity((window_height / scale) as usize));

        for _ in 0..window_height / scale {
            starting_tiles[width as usize].push(Tile::Dead);
        }
    }

    starting_tiles[(window_width / scale / 2) as usize][(window_height / scale / 2) as usize] =
        Tile::Alive;
    starting_tiles[(window_width / scale / 2 + 1) as usize][(window_height / scale / 2) as usize] =
        Tile::Alive;
    starting_tiles[(window_width / scale / 2 + 2) as usize][(window_height / scale / 2) as usize] =
        Tile::Alive;

    starting_tiles[(window_width / scale / 2 - 1) as usize]
        [(window_height / scale / 2 + 1) as usize] = Tile::Alive;
    starting_tiles[(window_width / scale / 2) as usize][(window_height / scale / 2 + 1) as usize] =
        Tile::Alive;
    starting_tiles[(window_width / scale / 2 + 1) as usize]
        [(window_height / scale / 2 + 1) as usize] = Tile::Alive;

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        board: Board {
            tiles: starting_tiles,
            scale: scale as f64,
            tile_width: (window_width / scale) as i32,
            tile_height: (window_height / scale) as i32,
        },
        running: true,
    };

    let mut event_settings = EventSettings::new();
    event_settings.ups = 4;
    event_settings.max_fps = 250;

    let mut events = Events::new(event_settings);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(u) = e.update_args() {
            if !game.update(&u) {
                break;
            }
        }

        if let Some(b) = e.button_args() {
            if b.state == ButtonState::Press {
                if let Some(m) = e.mouse_cursor_args() {
                    game.input(&b.button, Some(&m));
                } else {
                    game.input(&b.button, None);
                }
            }
        }
    }
}
