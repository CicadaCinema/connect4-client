extern crate piston_window;

use piston_window::*;

// returns an rgba array from integer input using a predefined colour scheme
fn process_colour(input_colour:i32) -> [f32; 4] {
    match input_colour {
        0 => [0.0, 0.0, 0.25, 1.0],
        1 => [0.0, 1.0, 0.0, 1.0],
        2 => [1.0, 0.0, 0.0, 1.0],
        _ => [0.0, 0.0, 0.0, 1.0],
    }
}

fn process_mouse_click(mut state: [[i32; 7]; 6], mouse_coords: [f64; 2]) -> [[i32; 7]; 6]{
    let mut click_indexes: [i32; 2] = [-1, -1];
    let mut valid_click = true;

    // determine indexes of mouse click
    for coord_index in 0..mouse_coords.len() {
        // remove offset (subtract 5)
        let coord: i32 = (mouse_coords[coord_index] as i32) - 5;

        if coord % 55 > 50 {
            // if modulo 55 is greater than 50, no click
            valid_click = false;
        } else {
            // otherwise, get index of click by integer dividing by 55
            click_indexes[coord_index] = coord / 55;

            // check whether click is within the playable grid (7 by 6)
            if !(0..[7, 6][coord_index]).contains(&click_indexes[coord_index]){
                valid_click = false;
            }
        }
    }

    println!("lmb pressed at {:?}", mouse_coords);
    println!("that'll be {:?}", click_indexes);
    println!("valid? {}", valid_click);

    // toggle the colour of this cell
    if valid_click {
        state[click_indexes[1] as usize][click_indexes[0] as usize] = 1 - state[click_indexes[1] as usize][click_indexes[0] as usize];
    }

    state
}

fn main() {
    // holds the state of the game board
    let mut state = [[0; 7]; 6];
    // holds the latest mouse co-ordinates
    let mut mouse_coords = [0.0; 2];

    // set up Piston Window
    let mut window: PistonWindow =
        WindowSettings::new("Connect 4", [640, 480])
            .exit_on_esc(true).build().unwrap();

    // update/event loop - this seems to run regardless of user input
    while let Some(event) = window.next() {
        // handle updating of mouse co-ordinates
        match event.mouse_cursor_args() {
            Some(coords) => mouse_coords = coords,
            None => {},
        }

        // handle mouse click
        if let Some(Button::Mouse(button)) = event.press_args() {
            if button == MouseButton::Left {
                state = process_mouse_click(state, mouse_coords);
            }
        }

        // draw window and objects/sprites
        window.draw_2d(&event, |context, graphics, _device| {
            // fill window with background colour
            clear([1.0; 4], graphics);

            // cycle through elements of state array
            for row_index in 0..(state.len()) {
                for column_index in 0..(state[0].len()) {
                    // calculate origins of this element
                    let x_origin = 5.0 + (column_index as f64) * 55.0;
                    let y_origin = 5.0 + (row_index as f64) * 55.0;

                    // draw this element in the correct colour, with a fixed size
                    rectangle(process_colour(state[row_index][column_index]),
                              [x_origin, y_origin, 50.0, 50.0],
                              context.transform,
                              graphics);
                }
            }
        });
    }
}