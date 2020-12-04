extern crate piston_window;

use piston_window::*;
use std::thread;
use std::sync::mpsc;
use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

// returns an rgba array from integer input using a predefined colour scheme
fn process_colour(input_colour:i32) -> [f32; 4] {
    match input_colour {
        0 => [0.0, 0.0, 0.25, 1.0],
        1 => [0.0, 1.0, 0.0, 1.0],
        2 => [1.0, 0.0, 0.0, 1.0],
        _ => [0.0, 0.0, 0.0, 1.0],
    }
}

fn process_mouse_click(state: &mut [[i32; 7]; 6], mouse_coords: [f64; 2]) -> (bool, u8) {
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

    //println!("lmb pressed at {:?}", mouse_coords);
    //println!("indexes are {:?}", click_indexes);
    //println!("valid? {}", valid_click);

    if valid_click {
        (true, click_indexes[0] as u8)
    } else {
        (false, 0)
    }
}

fn main() {
    // the state of the game board
    let mut state = [[0; 7]; 6];
    // the latest mouse co-ordinates
    let mut mouse_coords = [0.0; 2];
    // user's player id
    let mut self_player_id = 0;
    // info text below game board
    let mut info_text = (0, "Waiting to start...");
    let mut game_over = false;

    let (tx_server_client, rx_server_client) = mpsc::channel();
    let (tx_client_canvas, rx_client_canvas) = mpsc::channel();

    // launch networking thread
    thread::spawn(move || {
        // stores data received from stream
        let mut data = [0 as u8; 3];

        // server ip: 165.232.32.238
        // local testing: localhost
        match TcpStream::connect("localhost:32032") {
            Ok(mut network_stream) => {
                println!("Successfully connected to server in port 32032");

                // find out whether this is player 1 or 2
                let mut self_id = [0 as u8; 1];
                match network_stream.read_exact(&mut self_id) {
                    Ok(_) => {
                        let text = from_utf8(&self_id).unwrap();
                        // send this over to the main thread so it can update the turn indicator
                        match text {
                            "1" => {
                                tx_client_canvas.send([1 as u8; 3]);
                            }
                            "2" => {
                                tx_client_canvas.send([2 as u8; 3]);
                            }
                            _ => {}
                        }
                    },
                    Err(e) => {
                        println!("Failed to receive data from network (0): {}", e);
                    }
                }

                if self_id[0] == 49 {
                    // clear the stream!
                    loop {
                        match rx_server_client.try_recv() {
                            Ok(_) => {},
                            Err(_) => {break},
                        }
                    }

                    let msg: [u8; 1] = [rx_server_client.recv().unwrap()];
                    //println!("Got: {:?}", msg);
                    network_stream.write(&msg).unwrap();

                    data = [0 as u8; 3];
                    match network_stream.read_exact(&mut data) {
                        Ok(_) => {
                            tx_client_canvas.send(data);
                        },
                        Err(e) => {
                            println!("Failed to receive data from network (1): {}", e);
                        }
                    }
                }

                loop {
                    data = [0 as u8; 3];
                    match network_stream.read_exact(&mut data) {
                        Ok(_) => {
                            tx_client_canvas.send(data);
                        },
                        Err(e) => {
                            println!("Failed to receive data from network (2): {}", e);
                        }
                    }

                    // clear the stream!
                    loop {
                        match rx_server_client.try_recv() {
                            Ok(_) => {},
                            Err(_) => {break},
                        }
                    }

                    let msg: [u8; 1] = [rx_server_client.recv().unwrap()];
                    //println!("Got: {:?}", msg);
                    network_stream.write(&msg).unwrap();

                    data = [0 as u8; 3];
                    match network_stream.read_exact(&mut data) {
                        Ok(_) => {
                            tx_client_canvas.send(data);
                        },
                        Err(e) => {
                            println!("Failed to receive data from network (3): {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                println!("Failed to connect: {}", e);
            }
        }
        println!("Terminated.");
    });

    // set up Piston Window
    let mut window: PistonWindow =
        WindowSettings::new("Connect 4", [5 + 55*7, 480])
            .exit_on_esc(true).build().unwrap();

    // load font
    let mut glyphs = window.load_font("RobotoSlab-Regular.ttf").unwrap();

    // update/event loop - this seems to run regardless of user input
    while let Some(event) = window.next() {
        // handle updating of mouse co-ordinates
        match event.mouse_cursor_args() {
            Some(coords) => mouse_coords = coords,
            None => {},
        }

        // handle mouse click
        if let Some(Button::Mouse(button)) = event.press_args() {
            // if game is over, don't even try to process this input
            if button == MouseButton::Left && !game_over {
                // process coords to get these two important facts
                let (valid_click, click_column) = process_mouse_click(&mut state, mouse_coords);
                // if all is well, and the clicked column isn't full yet, go ahead and send the column id to the server
                if valid_click && state[0][click_column as usize] == 0 {
                    // add one to click_column to enable server to know when a client is dead - the server will subtract 1 from the column to get the true column id
                    tx_server_client.send(click_column + 1).unwrap();
                }
            }
        }

        // handle incoming state change
        match rx_client_canvas.try_recv() {
            // if there is a value waiting in the stream, act on it (modify state)
            Ok(t) => {
                // first command is always an acknowledgement of self player id
                if self_player_id == 0 {
                    // set self player id based on data received from server
                    self_player_id = t[0];
                    // set turn indicator colour to match player colour
                    info_text.0 = t[0] as i32;
                    // set turn indicator for the first time
                    info_text.1 = match self_player_id {
                        1 => {"Your turn"}
                        2 => {"Opponent's turn"}
                        _ => {"Error_P0"}
                    }
                } else {
                    // apply new instruction to playing field (state)
                    state[t[0] as usize][t[1] as usize] = t[2] as i32 % 3;
                    println!("Got new instruction {:?}", t);

                    if t[2]>2 {
                        // someone won! - check who won
                        info_text.1 = match (t[2] - 3) == self_player_id {
                            true => {"You won!"}
                            false => {"You lost!"}
                        };
                        // game is now over
                        game_over = true;
                    } else {
                        // nobody won yet - update turn indicator (the reverse of current value)
                        info_text.1 = match info_text.1 {
                            "Opponent's turn" => "Your turn",
                            "Your turn" => "Opponent's turn",
                            _ => "Error_P1",
                        }
                    }
                }
            },
            // if not, do nothing
            Err(_) => {},
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

            // display turn indicator with the correct colour and text
            text(
                process_colour(info_text.0), 32,
                info_text.1,
                &mut glyphs,
                context.transform.trans(10.0, 400.0),
                graphics
            ).unwrap();

            // update glyphs before rendering.
            glyphs.factory.encoder.flush(_device);
        });
    }
}