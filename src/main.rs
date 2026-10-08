use macroquad::prelude::*;
const COLS: usize = 10;
const ROWS: usize = 20;
const BLOCK_SIZE: f32 = 30.0;

//FALLING PIECESSS, SO I ( and the game) knows what the hell I'm doing
struct Piece {
    x: i32,
    y: i32,
    color: Color,
}

#[macroquad::main("Rust Tetris")]
async fn main() {
    let background_color = BLACK;
    // A mutable 2D array
    let mut board: [[Option<Color>; COLS]; ROWS] = [[None; COLS]; ROWS];
    board[19][5] = Some(RED);

    //Spawn point?
    let mut current_piece = Piece {
        x: 4,
        y: 5,
        color: YELLOW,
    };

    let mut last_update = get_time();
    loop {
        clear_background(background_color);

        if get_time() - last_update > 0.5 {
            // move the piece down one row
            current_piece.y += 1;

            //Reset the timer
            last_update = get_time();
        }

        //board size
        let board_width = COLS as f32 * BLOCK_SIZE;
        let board_height = ROWS as f32 * BLOCK_SIZE;

        //the board posititon
        let offset_x = (screen_width() - board_width) / 2.0;
        let offset_y = (screen_height() - board_height) / 2.0;

        // rendering the board
        draw_rectangle(offset_x, offset_y, board_width, board_height, WHITE);

        //drawing the blocks
        for i in 0..=10 {
            //x or horizontal axis(starting from the offset space,x is the first line afterwards. same for y)
            let x = offset_x + (i as f32 * BLOCK_SIZE);
            draw_line(x, offset_y, x, offset_y + board_height, 1.0, LIGHTGRAY);
        }
        for i in 0..=20 {
            // y or vertical axis
            let y = offset_y + (i as f32 * BLOCK_SIZE);
            draw_line(offset_x, y, offset_x + board_width, y, 1.0, LIGHTGRAY);
        }
        // Use a nested loop to look at every column and row to draw the blocks
        for row in 0..ROWS {
            for col in 0..COLS {
                match board[row][col] {
                    Some(color) => {
                        let x = offset_x + (col as f32 * BLOCK_SIZE);
                        let y = offset_y + (row as f32 * BLOCK_SIZE);

                        //Draw the colored block
                        draw_rectangle(x, y, BLOCK_SIZE, BLOCK_SIZE, color);

                        // a darker outline so it doesnt blend
                        draw_rectangle_lines(x, y, BLOCK_SIZE, BLOCK_SIZE, 2.0, color);
                    }
                    None => { //Nothing to yet sha
                    }
                }
            }
        }
        //calculating for the falling pieces
         let px = offset_x + (current_piece.x as f32 * BLOCK_SIZE);
         let py = offset_y + (current_piece.y as f32 * BLOCK_SIZE);
         draw_rectangle(px, py, BLOCK_SIZE, BLOCK_SIZE, current_piece.color);
         draw_rectangle_lines(px, py, BLOCK_SIZE, BLOCK_SIZE, 1.0, color_u8!(0, 0, 0, 100));

        for i in 0..=COLS {
            let x = offset_x + (i as f32 * BLOCK_SIZE);
            draw_line(
                x,
                offset_y,
                x,
                offset_y + board_height,
                1.0,
                color_u8!(50, 50, 50, 255),
            );
        }
        for i in 0..=ROWS {
            let y = offset_y + (i as f32 * BLOCK_SIZE);
            draw_line(
                offset_x,
                y,
                offset_x + board_width,
                y,
                1.0,
                color_u8!(50, 50, 50, 255),
            );
        }
        next_frame().await
    }
}
