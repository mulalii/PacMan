//use rand::Rng;
//use std::io;
use std::collections::VecDeque;
use std::collections::HashMap;
use macroquad::prelude::*;

/* 
    - use threading to slow down movement
    - thread::sleep::Duration::from_millis(100)
    - 

*/

struct Player {
    x : usize,
    y : usize,
    px: f32,
    py: f32,
    score : u32,
    move_timer: f32,
    move_delay: f32,
    facing: (f32, f32),
}

struct Ghost {
    x : usize,
    y : usize,
    px: f32,
    py: f32,
    move_timer: f32,
    move_delay: f32,
}

#[derive(PartialEq)]
#[derive(Clone, Copy)]
enum Tile {
    Wall,
    Pellet,
    Empty,
    PowerPellet,
    GhostHouse,
    //Food,
    //Player,
    //Ghost,
}

//impl Player {}

const TILE_SIZE: f32 = 20.0;

fn grid_to_pixels(x: usize, y: usize) -> (f32, f32) {
    (x as f32 * TILE_SIZE, y as f32 * TILE_SIZE)
}

fn ghost_bfs_ai(ghost : &mut Ghost, player : &Player, grid : &Vec<Vec<Tile>>) {
    //find shortest path from ghost to player using BFS.
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut queue = VecDeque::new();
    let start = (ghost.x, ghost.y);
    let goal = (player.x, player.y);

    ghost.move_timer += get_frame_time();
    
    if ghost.move_timer < ghost.move_delay {
        return;
    }

    let mut visited: HashMap<(usize, usize), Option<(usize, usize)>> = HashMap::new();
    queue.push_back(start);
    visited.insert(start, None); 

    while let Some((x, y)) = queue.pop_front() {
        if (x, y) == goal {
            println!("Goal reached!!");

            let mut path = Vec::new();
            let mut current = Some((x, y));
            while let Some(pos) = current {
                path.push(pos);
                current = visited[&pos];
            }
            path.reverse();

            if path.len() > 1 {
                let (nx, ny) = path[1];
                ghost.x = nx;
                ghost.y = ny;
                ghost.move_timer = 0.0;
            }
            return;
        }

        for (dx, dy) in directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >=0 && ny >= 0 && (nx as usize) < grid[0].len() && (ny as usize) < grid.len() {
                let nnx = nx as usize;
                let nny = ny as usize;
                if grid[nny][nnx] != Tile::Wall && !visited.contains_key(&(nnx, nny)) {
                    visited.insert((nnx, nny), Some((x,y)));
                    queue.push_back((nnx, nny));
                }
            }
        }
    }
}

fn move_player(player: &mut Player, grid: &mut Vec<Vec<Tile>>) {
    /*
    - Player will move in the direction of the key pressed.
    - player cant move in wall
    - wrap around logic
    */

    player.move_timer += get_frame_time();
    
    if player.move_timer < player.move_delay {
        return;
    }

    let mut direction: Option<(isize, isize)> = None;

    if is_key_down(KeyCode::Right) {direction = Some((1, 0)); player.facing = (1.0, 0.0);}
    if is_key_down(KeyCode::Left) {direction = Some((-1, 0)); player.facing = (-1.0, 0.0);}
    if is_key_down(KeyCode::Up) {direction = Some((0, -1)); player.facing = (0.0, -1.0);}
    if is_key_down(KeyCode::Down) {direction = Some((0, 1)); player.facing = (0.0, 1.0);}

    if let Some((dx, dy)) = direction {
        
        let new_x = (player.x as isize + dx).rem_euclid(grid[0].len() as isize) as usize;
        let new_y = (player.y as isize + dy).rem_euclid(grid.len() as isize) as usize;
        
        match grid[new_y][new_x] {
            Tile::Wall => println!("You can't move in wall"),
            Tile::Pellet => {
                player.score += 10;
                grid[new_y][new_x] = Tile::Empty;
                player.x = new_x;
                player.y = new_y;
            }
            Tile::PowerPellet => {
                player.score += 50;
                grid[new_y][new_x] = Tile::Empty;
                player.x = new_x;
                player.y = new_y;
            }
            Tile::GhostHouse => println!("You can't move in ghost house"),
            Tile::Empty => {
                player.x = new_x;
                player.y = new_y;
            }
        }
        player.move_timer = 0.0;
    }
}

fn update_position(px: &mut f32, py: &mut f32, x: usize, y: usize, move_delay: f32) {
    let (target_px, target_py) = grid_to_pixels(x, y);
    let speed = TILE_SIZE / move_delay; // pixels per second
    let dt = get_frame_time();

    if (*px - target_px).abs() > 0.1 {
        *px += (target_px - *px).signum() * speed * dt;
    }
    if (*py - target_py).abs() > 0.1 {
        *py += (target_py - *py).signum() * speed * dt;
    }
}

fn make_grid() -> Vec<Vec<Tile>> {
    let raw_map = [
        "############################",
        "#............##............#",
        "#.####.#####.##.#####.####.#",
        "#o####.#####.##.#####.####o#",
        "#.####.#####.##.#####.####.#",
        "#..........................#",
        "#.####.##.########.##.####.#",
        "#.####.##.########.##.####.#",
        "#......##....##....##......#",
        "######.##### ## #####.######",
        "######.##### ## #####.######",
        "######.##          ##.######",
        "######.## ###GH### ##.######",
        "######.## ######## ##.######",
        "######.## ######## ##.######",
        "#............##............#",
        "#.####.#####.##.#####.####.#",
        "#o..##................##..o#",
        "###.##.##.########.##.##.###",
        "#......##....##....##......#",
        "#.##########.##.##########.#",
        "#..........................#",
        "############################",
    ];

    raw_map.iter().map(|row| {
        row.chars().map(|c| match c {
            '#' => Tile::Wall,
            '.' => Tile::Pellet,
            'o' => Tile::PowerPellet,
            'G' => Tile::GhostHouse,
            ' ' => Tile::Empty,
            _ => Tile::Empty,
        }).collect()
    }).collect()
}

#[macroquad::main("Pac-Man")]
async fn main() {
    let mut grid = make_grid();
    let mut player = Player {x : 13, y : 17,px: 13.0 * TILE_SIZE, py : 17.0 * TILE_SIZE, score : 0, move_timer : 0.0, move_delay : 0.1, facing: (1.0, 0.0)};
    let mut ghost = Ghost {x : 14, y : 12,px: 14.0 * TILE_SIZE, py : 12.0 * TILE_SIZE, move_timer: 0.0, move_delay: 0.2};
    //Make sure to copy full path if it doesnt work 
    let pacman_texture = load_texture("C:\\Users\\munda\\Desktop\\main\\rustlang\\project_pacman\\src\\pacman_spritesheet.png").await.unwrap();
    let frame_width = 32.0;
    let frame_height = 32.0;
    let mut timer = 0.0;
    let mut frame = 0;


    loop {
        clear_background(BLACK);

        timer += get_frame_time();
        if timer > 0.1 {
            frame = (frame + 1) % 3;
            timer = 0.0;
        }

        //let (px, py) = grid_to_pixels(player.x, player.y);
        draw_texture_ex(
            &pacman_texture,
            player.px,
            player.py,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(frame as f32 * frame_width, 0.0, frame_width, frame_height)),
                dest_size: Some(Vec2{x: TILE_SIZE, y: TILE_SIZE}),
                rotation: match player.facing {
                    (1.0, 0.0) => 0.0,
                    (-1.0, 0.0) => std::f32::consts::PI,
                    (0.0, -1.0) => -std::f32::consts::FRAC_PI_2,
                    (0.0, 1.0) => std::f32::consts::FRAC_PI_2,
                    _ => 0.0,
                },
                ..Default::default()
            },
        );

        for (y, row) in grid.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let (px, py) = grid_to_pixels(x, y);
                match tile {
                    Tile::Wall => draw_rectangle(px, py, TILE_SIZE, TILE_SIZE, BLUE),
                    Tile::Pellet => draw_circle(px + TILE_SIZE/2.0, py + TILE_SIZE/2.0, 5.0, WHITE),
                    Tile::PowerPellet=> draw_circle(px + TILE_SIZE/2.0, py + TILE_SIZE/2.0, 8.0, WHITE),
                    Tile::GhostHouse => draw_rectangle(px, py, TILE_SIZE, TILE_SIZE, GRAY),
                    Tile::Empty => {}
                }
            }
        }

        //let (gx, gy) = grid_to_pixels(ghost.x, ghost.y);
        draw_circle(ghost.px + TILE_SIZE/2.0, ghost.py + TILE_SIZE/2.0, TILE_SIZE/2.5, RED);

        draw_text(
            &format!("Score: {}", player.score),
            10.0,
            10.0,
            30.0,
            WHITE,
        );

        move_player(&mut player, &mut grid);

        ghost_bfs_ai(&mut ghost, &player, &grid);

        update_position(&mut player.px, &mut player.py, player.x, player.y, player.move_delay);
        update_position(&mut ghost.px, &mut ghost.py, ghost.x, ghost.y, ghost.move_delay);

        if ghost.x == player.x && ghost.y == player.y {
            println!("Game Over! final score: {}", player.score);
            break;
        }

        // win condition
        if !grid.iter().any(|row| row.iter().any(|tile| *tile == Tile::Pellet)) {
            println!("You win! Final Score: {}", player.score);
            break;
        }

        next_frame().await;
    }
}