use macroquad::prelude::*;
const COLS: usize = 10.0;
const ROWS: usize = 20.0;
const BLOCK_SIZE: f32 = 30.0;

#[macroquad::main("Rust Tetris")]
async fn main() {
    let background_color = BLACK;
    // A mutable 2D array
    let mut board:[[Option<Color>; COLS ROWS] = [None; COLS]: ROWS]
    board[19][5] = Some(RED);

    loop {
        clear_background(background_color);
        //board size
        let board_width = COLS * BLOCK_SIZE;
        let board_height = ROWS * BLOCK_SIZE;



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
        next_frame().await
    }
}
