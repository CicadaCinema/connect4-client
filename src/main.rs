extern crate piston_window;

use piston_window::*;

fn process_colour(input_colour:i32) -> [f32; 4]{
    match input_colour {
        0 => [0.5, 0.5, 0.0, 1.0],
        1 => [0.0, 1.0, 0.0, 1.0],
        2 => [1.0, 0.0, 0.0, 1.0],
        _ => [1.0, 1.0, 1.0, 1.0],
    }
}

fn main() {
    let mut state = [[0; 7]; 6];

    let mut window: PistonWindow =
        WindowSettings::new("Connect 4", [640, 480])
            .exit_on_esc(true).build().unwrap();
    while let Some(event) = window.next() {

        if let Some(Button::Mouse(button)) = event.press_args() {
            if button == MouseButton::Left {
                println!("lmb pressed");
            }
        }

        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);

            for row_index in 0..(state.len()) {
                for column_index in 0..(state[0].len()) {
                    let x_origin = 5.0 + (column_index as f64) * 55.0;
                    let y_origin = 5.0 + (row_index as f64) * 55.0;

                    rectangle(process_colour(state[row_index][column_index]),
                              [x_origin, y_origin, 50.0, 50.0],
                              context.transform,
                              graphics);
                }
            }
        });
    }
}