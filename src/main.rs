extern crate piston_window;

use piston_window::*;

// returns an rgba array from integer input using a predefined colour scheme
fn process_colour(input_colour:i32) -> [f32; 4] {
    match input_colour {
        0 => [0.5, 0.5, 0.0, 1.0],
        1 => [0.0, 1.0, 0.0, 1.0],
        2 => [1.0, 0.0, 0.0, 1.0],
        _ => [0.0, 0.0, 0.0, 1.0],
    }
}

fn process_input(mut state: [[i32; 7]; 6]) -> [[i32; 7]; 6]{
    println!("lmb pressed");
    state[0][0] = 1 - state[0][0];
    state
}

fn main() {
    // holds the state of the game board
    let mut state = [[0; 7]; 6];

    // set up Piston Window
    let mut window: PistonWindow =
        WindowSettings::new("Connect 4", [640, 480])
            .exit_on_esc(true).build().unwrap();

    // update/event loop - this seems to run regardless of user input
    while let Some(event) = window.next() {
        // handle mouse click
        if let Some(Button::Mouse(button)) = event.press_args() {
            if button == MouseButton::Left {
                state = process_input(state);
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