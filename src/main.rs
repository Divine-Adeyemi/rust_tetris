use macroquad::prelude::*;

const COLS: usize = 10;
const ROWS: usize = 20;
const BLOCK_SIZE: f32 = 30.0;

#[derive(PartialEq)]
enum GameState {
    Playing,
    Paused,
    GameOver,
}

struct Piece {
    x: i32,
    y: i32,
    color: Color,
    shape: [[u8; 4]; 4],
}

fn get_random_piece() -> Piece {
    let piece_type = rand::gen_range(0, 7);
    let (color, shape) = match piece_type {
        0 => (SKYBLUE, [[0,0,0,0], [1,1,1,1], [0,0,0,0], [0,0,0,0]]), 
        1 => (BLUE, [[0,0,0,0], [1,0,0,0], [1,1,1,0], [0,0,0,0]]), 
        2 => (ORANGE, [[0,0,0,0], [0,0,1,0], [1,1,1,0], [0,0,0,0]]), 
        3 => (YELLOW, [[0,0,0,0], [0,1,1,0], [0,1,1,0], [0,0,0,0]]), 
        4 => (GREEN, [[0,0,0,0], [0,1,1,0], [1,1,0,0], [0,0,0,0]]), 
        5 => (PURPLE, [[0,0,0,0], [0,1,0,0], [1,1,1,0], [0,0,0,0]]), 
        _ => (RED, [[0,0,0,0], [1,1,0,0], [0,1,1,0], [0,0,0,0]]), 
    };
    Piece { x: 3, y: -2, color, shape }
}

fn rotate_shape(shape: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
    let mut new_shape = [[0; 4]; 4];
    for r in 0..4 {
        for c in 0..4 {
            new_shape[c][3 - r] = shape[r][c];
        }
    }
    new_shape
}

fn is_valid_move(board: &[[Option<Color>; COLS]; ROWS], piece: &Piece, target_x: i32, target_y: i32) -> bool {
    for r in 0..4 {
        for c in 0..4 {
            if piece.shape[r][c] == 1 {
                let global_x = target_x + c as i32;
                let global_y = target_y + r as i32;
                if global_x < 0 || global_x >= COLS as i32 { return false; }
                if global_y >= ROWS as i32 { return false; }
                if global_y >= 0 {
                    if board[global_y as usize][global_x as usize].is_some() { return false; }
                }
            }
        }
    }
    true 
}

fn clear_lines(board: &mut [[Option<Color>; COLS]; ROWS]) -> u32 {
    let mut lines_cleared = 0;
    for y in 0..ROWS {
        let mut is_full = true;
        for x in 0..COLS {
            if board[y][x].is_none() {
                is_full = false;
                break;
            }
        }
        if is_full {
            lines_cleared += 1;
            for r in (1..=y).rev() {
                board[r] = board[r - 1]; 
            }
            board[0] = [None; COLS];
        }
    }
    lines_cleared
}

#[macroquad::main("Rust Tetris")]
async fn main() {
    let mut board: [[Option<Color>; COLS]; ROWS] = [[None; COLS]; ROWS];
    let mut current_piece = get_random_piece();
    let mut next_piece = get_random_piece();
    let mut last_update = get_time();

    let mut state = GameState::Playing;
    let mut score: u32 = 0;
    let mut highscore: u32 = 0;
    let mut total_lines: u32 = 0;
    let mut level: u32 = 1;

    loop {
        clear_background(DARKGRAY);

        if is_key_pressed(KeyCode::P) {
            if state == GameState::Playing {
                state = GameState::Paused;
            } else if state == GameState::Paused {
                state = GameState::Playing;
            }
        }

        if state == GameState::GameOver && is_key_pressed(KeyCode::Enter) {
            board = [[None; COLS]; ROWS];
            score = 0;
            total_lines = 0;
            level = 1;
            current_piece = get_random_piece();
            next_piece = get_random_piece(); 
            state = GameState::Playing;
        }

        if state == GameState::Playing {
            if is_key_pressed(KeyCode::RightBracket) {
                level += 1;
                total_lines = (level - 1) * 10; 
            }
            if is_key_pressed(KeyCode::LeftBracket) && level > 1 {
                level -= 1;
                total_lines = (level - 1) * 10;
            }

            let mut next_x = current_piece.x;
            if is_key_pressed(KeyCode::Left) { next_x -= 1; }
            else if is_key_pressed(KeyCode::Right) { next_x += 1; }
            
            if next_x != current_piece.x {
                if is_valid_move(&board, &current_piece, next_x, current_piece.y) {
                    current_piece.x = next_x;
                }
            }

            if is_key_pressed(KeyCode::Up) {
                let spun_shape = rotate_shape(current_piece.shape);
                let test_piece = Piece { x: current_piece.x, y: current_piece.y, color: current_piece.color, shape: spun_shape };
                if is_valid_move(&board, &test_piece, test_piece.x, test_piece.y) {
                    current_piece.shape = spun_shape;
                }
            }

            if is_key_pressed(KeyCode::Space) {
                while is_valid_move(&board, &current_piece, current_piece.x, current_piece.y + 1) {
                    current_piece.y += 1;
                }
                last_update = 0.0; 
            }

            let mut current_drop_speed = (0.5 - ((level as f64 - 1.0) * 0.05)).max(0.1);

            if is_key_down(KeyCode::Down) {
                current_drop_speed = 0.05; 
            }

            if get_time() - last_update > current_drop_speed {
                let next_y = current_piece.y + 1;

                if is_valid_move(&board, &current_piece, current_piece.x, next_y) {
                    current_piece.y = next_y;
                } else {
                    for r in 0..4 {
                        for c in 0..4 {
                            if current_piece.shape[r][c] == 1 {
                                let global_x = current_piece.x + c as i32;
                                let global_y = current_piece.y + r as i32;
                                if global_y >= 0 {
                                    board[global_y as usize][global_x as usize] = Some(current_piece.color);
                                }
                            }
                        }
                    }
                    
                    let lines_cleared = clear_lines(&mut board);
                    if lines_cleared > 0 {
                        let points = match lines_cleared {
                            1 => 100,
                            2 => 300,
                            3 => 500,
                            4 => 800,
                            _ => 0,
                        };
                        score += points * level;
                        total_lines += lines_cleared;
                        level = (total_lines / 10) + 1; 
                    }

                    current_piece = next_piece;
                    next_piece = get_random_piece();

                    if !is_valid_move(&board, &current_piece, current_piece.x, current_piece.y) {
                        state = GameState::GameOver;
                        if score > highscore {
                            highscore = score;
                        }
                    }
                }
                last_update = get_time();
            }
        } 

        let board_width = COLS as f32 * BLOCK_SIZE;
        let board_height = ROWS as f32 * BLOCK_SIZE;
        let offset_x = (screen_width() - board_width) / 2.0;
        let offset_y = (screen_height() - board_height) / 2.0;

        draw_rectangle(offset_x, offset_y, board_width, board_height, BLACK);

        for row in 0..ROWS {
            for col in 0..COLS {
                if let Some(color) = board[row][col] {
                    let x = offset_x + (col as f32 * BLOCK_SIZE);
                    let y = offset_y + (row as f32 * BLOCK_SIZE);
                    draw_rectangle(x, y, BLOCK_SIZE, BLOCK_SIZE, color);
                    draw_rectangle_lines(x, y, BLOCK_SIZE, BLOCK_SIZE, 1.0, color_u8!(0, 0, 0, 100));
                }
            }
        }

        if state != GameState::GameOver {
            for r in 0..4 {
                for c in 0..4 {
                    if current_piece.shape[r][c] == 1 {
                        let global_x = current_piece.x + c as i32;
                        let global_y = current_piece.y + r as i32;
                        let px = offset_x + (global_x as f32 * BLOCK_SIZE);
                        let py = offset_y + (global_y as f32 * BLOCK_SIZE);
                        draw_rectangle(px, py, BLOCK_SIZE, BLOCK_SIZE, current_piece.color);
                        draw_rectangle_lines(px, py, BLOCK_SIZE, BLOCK_SIZE, 1.0, color_u8!(0, 0, 0, 100));
                    }
                }
            }
        }

        for i in 0..=COLS {
            let x = offset_x + (i as f32 * BLOCK_SIZE);
            draw_line(x, offset_y, x, offset_y + board_height, 1.0, color_u8!(50, 50, 50, 255));
        }
        for i in 0..=ROWS {
            let y = offset_y + (i as f32 * BLOCK_SIZE);
            draw_line(offset_x, y, offset_x + board_width, y, 1.0, color_u8!(50, 50, 50, 255));
        }

        let text_x = offset_x + board_width + 20.0;
        draw_text(&format!("SCORE: {}", score), text_x, offset_y + 30.0, 30.0, WHITE);
        draw_text(&format!("HIGH: {}", highscore), text_x, offset_y + 70.0, 30.0, GOLD);
        draw_text(&format!("LEVEL: {}", level), text_x, offset_y + 110.0, 30.0, WHITE);
        draw_text(&format!("LINES: {}", total_lines), text_x, offset_y + 150.0, 30.0, WHITE);

        draw_text("NEXT:", text_x, offset_y + 210.0, 30.0, WHITE);
        for r in 0..4 {
            for c in 0..4 {
                if next_piece.shape[r][c] == 1 {
                    let px = text_x + (c as f32 * BLOCK_SIZE);
                    let py = offset_y + 230.0 + (r as f32 * BLOCK_SIZE);
                    draw_rectangle(px, py, BLOCK_SIZE, BLOCK_SIZE, next_piece.color);
                    draw_rectangle_lines(px, py, BLOCK_SIZE, BLOCK_SIZE, 1.0, color_u8!(0, 0, 0, 100));
                }
            }
        }

        if state == GameState::Paused {
            draw_text("PAUSED", offset_x + 60.0, screen_height() / 2.0, 50.0, YELLOW);
        } else if state == GameState::GameOver {
            draw_text("GAME OVER", offset_x + 25.0, screen_height() / 2.0, 50.0, RED);
            draw_text("Press ENTER", offset_x + 35.0, screen_height() / 2.0 + 40.0, 30.0, WHITE);
        }

        next_frame().await;
    }
}
