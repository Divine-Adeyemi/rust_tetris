use macroquad::prelude::*;

const COLS: usize = 10;
const ROWS: usize = 20;
const BLOCK_SIZE: f32 = 30.0;

// Retro arcade theme: bright pieces on a black screen
const UI_SHELL: Color = Color::new(0.07, 0.07, 0.10, 1.0); // dark cabinet around the screen
const UI_BG: Color = Color::new(0.0, 0.0, 0.0, 1.0); // black screen
const UI_FG: Color = Color::new(0.95, 0.95, 0.95, 1.0); // white text / borders
const UI_DIM: Color = Color::new(0.50, 0.50, 0.55, 1.0); // gray secondary text
const UI_ACCENT: Color = Color::new(1.0, 0.84, 0.0, 1.0); // gold highlight

// Classic tetromino colors, arcade-bright
const C_CYAN: Color = Color::new(0.0, 0.90, 0.90, 1.0); // I
const C_BLUE: Color = Color::new(0.15, 0.40, 1.0, 1.0); // J
const C_ORANGE: Color = Color::new(1.0, 0.60, 0.0, 1.0); // L
const C_YELLOW: Color = Color::new(1.0, 0.85, 0.0, 1.0); // O
const C_GREEN: Color = Color::new(0.10, 0.80, 0.15, 1.0); // S
const C_PURPLE: Color = Color::new(0.70, 0.15, 0.90, 1.0); // T
const C_RED: Color = Color::new(0.95, 0.10, 0.10, 1.0); // Z

const SCREEN_MARGIN: f32 = 16.0;

// Fixed design resolution; scaled to fit the real window / browser canvas
const DESIGN_W: f32 = 660.0;
const DESIGN_H: f32 = 740.0;

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
        0 => (C_CYAN, [[0,0,0,0], [1,1,1,1], [0,0,0,0], [0,0,0,0]]),
        1 => (C_BLUE, [[0,0,0,0], [1,0,0,0], [1,1,1,0], [0,0,0,0]]),
        2 => (C_ORANGE, [[0,0,0,0], [0,0,1,0], [1,1,1,0], [0,0,0,0]]),
        3 => (C_YELLOW, [[0,0,0,0], [0,1,1,0], [0,1,1,0], [0,0,0,0]]),
        4 => (C_GREEN, [[0,0,0,0], [0,1,1,0], [1,1,0,0], [0,0,0,0]]),
        5 => (C_PURPLE, [[0,0,0,0], [0,1,0,0], [1,1,1,0], [0,0,0,0]]),
        _ => (C_RED, [[0,0,0,0], [1,1,0,0], [0,1,1,0], [0,0,0,0]]),
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

fn lighten(c: Color, t: f32) -> Color {
    Color::new(c.r + (1.0 - c.r) * t, c.g + (1.0 - c.g) * t, c.b + (1.0 - c.b) * t, 1.0)
}

fn darken(c: Color, t: f32) -> Color {
    Color::new(c.r * (1.0 - t), c.g * (1.0 - t), c.b * (1.0 - t), 1.0)
}

// Chunky NES-style brick: filled square, dark frame, bright core pixel
fn draw_block(x: f32, y: f32, size: f32, fill: Color) {
    draw_rectangle(x, y, size, size, fill);
    draw_rectangle_lines(x + 1.0, y + 1.0, size - 2.0, size - 2.0, 2.0, darken(fill, 0.6));
    let core = (size * 0.4).floor();
    let pad = ((size - core) / 2.0).floor();
    draw_rectangle(x + pad, y + pad, core, core, lighten(fill, 0.6));
}

// Bordered panel box in the style of arcade side boxes
fn draw_box(x: f32, y: f32, w: f32, h: f32) {
    draw_rectangle(x, y, w, h, UI_BG);
    draw_rectangle_lines(x, y, w, h, 3.0, UI_FG);
    draw_rectangle_lines(x + 5.0, y + 5.0, w - 10.0, h - 10.0, 1.0, UI_DIM);
}

fn draw_text_centered(text: &str, cx: f32, y: f32, size: u16, color: Color) {
    let dims = measure_text(text, None, size, 1.0);
    draw_text(text, cx - dims.width / 2.0, y, size as f32, color);
}

fn window_conf() -> Conf {
    Conf {
        window_title: "RUST TETRIS".to_owned(),
        window_width: DESIGN_W as i32,
        window_height: DESIGN_H as i32,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut board: [[Option<Color>; COLS]; ROWS] = [[None; COLS]; ROWS];
    let mut current_piece = get_random_piece();
    let mut next_piece = get_random_piece();

    let mut last_update = get_time();
    let mut move_timer = get_time();

    let mut state = GameState::Playing;
    let mut score: u32 = 0;
    let mut highscore: u32 = 0;
    let mut total_lines: u32 = 0;
    let mut level: u32 = 1;

    loop {
        let board_width = COLS as f32 * BLOCK_SIZE;
        let board_height = ROWS as f32 * BLOCK_SIZE;
        let offset_x = SCREEN_MARGIN + 14.0;
        let offset_y = SCREEN_MARGIN + 20.0;

        // Scale the fixed design to the actual window / browser canvas (letterboxed)
        let scale = (screen_width() / DESIGN_W).min(screen_height() / DESIGN_H);
        let view_w = screen_width() / scale;
        let view_h = screen_height() / scale;
        let cam = Camera2D {
            target: vec2(DESIGN_W / 2.0, DESIGN_H / 2.0),
            zoom: vec2(2.0 / view_w, 2.0 / view_h),
            ..Default::default()
        };

        // One row of touch buttons along the bottom of the screen
        let btn_w = 88.0;
        let btn_h = 56.0;
        let btn_gap = 10.0;
        let btn_row_w = 6.0 * btn_w + 5.0 * btn_gap;
        let btn_x0 = (DESIGN_W - btn_row_w) / 2.0;
        let btn_y = DESIGN_H - SCREEN_MARGIN - btn_h - 18.0;
        let btn_at = |i: f32| Rect::new(btn_x0 + i * (btn_w + btn_gap), btn_y, btn_w, btn_h);

        let btn_pause = btn_at(0.0);
        let btn_left = btn_at(1.0);
        let btn_right = btn_at(2.0);
        let btn_rot = btn_at(3.0);
        let btn_soft = btn_at(4.0);
        let btn_hard = btn_at(5.0);

        let mut intent_pause = is_key_pressed(KeyCode::P);
        let mut intent_restart = is_key_pressed(KeyCode::Enter);

        let mut intent_left_press = is_key_pressed(KeyCode::Left);
        let mut intent_left_hold = is_key_down(KeyCode::Left);

        let mut intent_right_press = is_key_pressed(KeyCode::Right);
        let mut intent_right_hold = is_key_down(KeyCode::Right);

        let mut intent_rotate = is_key_pressed(KeyCode::Up);
        let mut intent_hard = is_key_pressed(KeyCode::Space);
        let mut intent_soft = is_key_down(KeyCode::Down);

        // Touch and mouse both drive the on-screen buttons, in design coordinates
        let mut pointers: Vec<(Vec2, bool, bool)> = Vec::new();
        for touch in touches() {
            let tapped = touch.phase == TouchPhase::Started;
            let held = touch.phase != TouchPhase::Ended && touch.phase != TouchPhase::Cancelled;
            pointers.push((cam.screen_to_world(touch.position), tapped, held));
        }
        {
            let (mx, my) = mouse_position();
            let tapped = is_mouse_button_pressed(MouseButton::Left);
            let held = is_mouse_button_down(MouseButton::Left);
            if tapped || held {
                pointers.push((cam.screen_to_world(vec2(mx, my)), tapped, held));
            }
        }

        for (pos, tapped, held) in pointers {
            if state == GameState::GameOver && tapped { intent_restart = true; }
            if btn_pause.contains(pos) && tapped { intent_pause = true; }

            if btn_left.contains(pos) {
                if tapped { intent_left_press = true; }
                if held { intent_left_hold = true; }
            }
            if btn_right.contains(pos) {
                if tapped { intent_right_press = true; }
                if held { intent_right_hold = true; }
            }
            if btn_rot.contains(pos) && tapped { intent_rotate = true; }
            if btn_hard.contains(pos) && tapped { intent_hard = true; }
            if btn_soft.contains(pos) && held { intent_soft = true; }
        }

        if intent_pause {
            if state == GameState::Playing { state = GameState::Paused; }
            else if state == GameState::Paused { state = GameState::Playing; }
        }

        if state == GameState::GameOver && intent_restart {
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

            let current_time = get_time();
            let mut next_x = current_piece.x;

            if intent_left_press {
                next_x -= 1;
                move_timer = current_time + 0.15;
            } else if intent_left_hold && current_time > move_timer {
                next_x -= 1;
                move_timer = current_time + 0.05;
            }

            if intent_right_press {
                next_x += 1;
                move_timer = current_time + 0.15;
            } else if intent_right_hold && current_time > move_timer {
                next_x += 1;
                move_timer = current_time + 0.05;
            }

            if next_x != current_piece.x {
                if is_valid_move(&board, &current_piece, next_x, current_piece.y) {
                    current_piece.x = next_x;
                }
            }

            if intent_rotate {
                let spun_shape = rotate_shape(current_piece.shape);
                let test_piece = Piece { x: current_piece.x, y: current_piece.y, color: current_piece.color, shape: spun_shape };
                if is_valid_move(&board, &test_piece, test_piece.x, test_piece.y) {
                    current_piece.shape = spun_shape;
                }
            }

            if intent_hard {
                while is_valid_move(&board, &current_piece, current_piece.x, current_piece.y + 1) {
                    current_piece.y += 1;
                }
                last_update = 0.0;
            }

            let mut current_drop_speed = (0.5 - ((level as f64 - 1.0) * 0.05)).max(0.1);

            if intent_soft {
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
                            1 => 100, 2 => 300, 3 => 500, 4 => 800, _ => 0,
                        };
                        score += points * level;
                        total_lines += lines_cleared;
                        level = (total_lines / 10) + 1;
                    }

                    current_piece = next_piece;
                    next_piece = get_random_piece();

                    if !is_valid_move(&board, &current_piece, current_piece.x, current_piece.y) {
                        state = GameState::GameOver;
                        if score > highscore { highscore = score; }
                    }
                }
                last_update = get_time();
            }
        }

        // ---- Drawing: dark cabinet around a black arcade screen ----
        clear_background(UI_SHELL);
        set_camera(&cam);
        draw_rectangle(
            SCREEN_MARGIN, SCREEN_MARGIN,
            DESIGN_W - 2.0 * SCREEN_MARGIN,
            DESIGN_H - 2.0 * SCREEN_MARGIN,
            UI_BG,
        );

        // Board well with double border
        draw_rectangle_lines(offset_x - 7.0, offset_y - 7.0, board_width + 14.0, board_height + 14.0, 3.0, UI_FG);
        draw_rectangle_lines(offset_x - 2.0, offset_y - 2.0, board_width + 4.0, board_height + 4.0, 2.0, UI_DIM);

        for row in 0..ROWS {
            for col in 0..COLS {
                if let Some(color) = board[row][col] {
                    let draw_color = if state == GameState::GameOver { UI_DIM } else { color };
                    let x = offset_x + (col as f32 * BLOCK_SIZE);
                    let y = offset_y + (row as f32 * BLOCK_SIZE);
                    draw_block(x, y, BLOCK_SIZE, draw_color);
                }
            }
        }

        if state != GameState::GameOver {
            for r in 0..4 {
                for c in 0..4 {
                    if current_piece.shape[r][c] == 1 {
                        let global_x = current_piece.x + c as i32;
                        let global_y = current_piece.y + r as i32;
                        if global_y >= 0 {
                            let px = offset_x + (global_x as f32 * BLOCK_SIZE);
                            let py = offset_y + (global_y as f32 * BLOCK_SIZE);
                            draw_block(px, py, BLOCK_SIZE, current_piece.color);
                        }
                    }
                }
            }
        }

        // ---- Side panel: boxed stats like GB Tetris ----
        let panel_x = offset_x + board_width + 22.0;
        let panel_w = DESIGN_W - panel_x - SCREEN_MARGIN - 12.0;
        let panel_cx = panel_x + panel_w / 2.0;

        draw_box(panel_x, offset_y, panel_w, 66.0);
        draw_text_centered("SCORE", panel_cx, offset_y + 26.0, 22, UI_DIM);
        draw_text_centered(&format!("{:06}", score), panel_cx, offset_y + 54.0, 30, UI_FG);

        draw_box(panel_x, offset_y + 78.0, panel_w, 66.0);
        draw_text_centered("TOP", panel_cx, offset_y + 104.0, 22, UI_DIM);
        draw_text_centered(&format!("{:06}", highscore), panel_cx, offset_y + 132.0, 30, UI_ACCENT);

        draw_box(panel_x, offset_y + 156.0, panel_w, 90.0);
        draw_text_centered(&format!("LEVEL  {:02}", level), panel_cx, offset_y + 192.0, 26, C_CYAN);
        draw_text_centered(&format!("LINES  {:03}", total_lines), panel_cx, offset_y + 226.0, 26, C_GREEN);

        let next_box_y = offset_y + 258.0;
        draw_box(panel_x, next_box_y, panel_w, 160.0);
        draw_text_centered("NEXT", panel_cx, next_box_y + 28.0, 22, UI_DIM);
        let preview_block = 24.0;
        let preview_x = panel_cx - 2.0 * preview_block;
        let preview_y = next_box_y + 44.0;
        for r in 0..4 {
            for c in 0..4 {
                if next_piece.shape[r][c] == 1 {
                    let px = preview_x + (c as f32 * preview_block);
                    let py = preview_y + (r as f32 * preview_block);
                    draw_block(px, py, preview_block, next_piece.color);
                }
            }
        }

        let controls_y = next_box_y + 190.0;
        draw_text("CONTROLS", panel_x + 6.0, controls_y, 22.0, UI_FG);
        let control_lines = [
            "< > ....... MOVE",
            "UP ....... ROTATE",
            "DOWN ....... SOFT",
            "SPACE ....... HARD",
            "P ....... PAUSE",
            "[ ] ....... LEVEL",
        ];
        for (i, line) in control_lines.iter().enumerate() {
            draw_text(line, panel_x + 6.0, controls_y + 26.0 + (i as f32 * 24.0), 18.0, UI_DIM);
        }

        // ---- Bottom touch button row ----
        let draw_btn = |rect: Rect, label: &str| {
            draw_rectangle(rect.x, rect.y, rect.w, rect.h, UI_SHELL);
            draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 3.0, UI_FG);
            draw_rectangle_lines(rect.x + 5.0, rect.y + 5.0, rect.w - 10.0, rect.h - 10.0, 1.0, UI_DIM);
            let dims = measure_text(label, None, 24, 1.0);
            draw_text(label, rect.x + (rect.w - dims.width) / 2.0, rect.y + (rect.h + dims.height) / 2.0, 24.0, UI_FG);
        };

        draw_btn(btn_pause, "||");
        draw_btn(btn_left, "<");
        draw_btn(btn_right, ">");
        draw_btn(btn_rot, "ROT");
        draw_btn(btn_soft, "SFT");
        draw_btn(btn_hard, "HRD");

        // ---- Retro dialog boxes over the board ----
        let draw_dialog = |lines: &[(&str, u16, Color)]| {
            let dw = 240.0;
            let dh = 40.0 + lines.len() as f32 * 40.0;
            let dx = offset_x + (board_width - dw) / 2.0;
            let dy = offset_y + (board_height - dh) / 2.0;
            draw_rectangle(dx, dy, dw, dh, UI_BG);
            draw_rectangle_lines(dx, dy, dw, dh, 4.0, UI_FG);
            draw_rectangle_lines(dx + 7.0, dy + 7.0, dw - 14.0, dh - 14.0, 2.0, UI_DIM);
            for (i, (text, size, color)) in lines.iter().enumerate() {
                draw_text_centered(text, dx + dw / 2.0, dy + 48.0 + (i as f32 * 40.0), *size, *color);
            }
        };

        if state == GameState::Paused {
            draw_dialog(&[("PAUSED", 34, UI_ACCENT)]);
        } else if state == GameState::GameOver {
            draw_dialog(&[("GAME  OVER", 32, C_RED), ("PRESS ENTER", 22, UI_FG), ("OR TAP", 22, UI_FG)]);
        }

        set_default_camera();
        next_frame().await;
    }
}
