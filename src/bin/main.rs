extern crate piston_window;
extern crate gilrs;
extern crate num;
extern crate gamepad_move;

use piston_window::*;
use gilrs::{Gilrs, Button, Event, EventType};
use gamepad_move::{Player, Rock};
use std::sync::{Arc, Mutex};
use std::thread;

const SIZE: u32 = 800;

fn main() {
    println!("Hello, world!");

    // Main window
    let mut window: PistonWindow = 
        WindowSettings::new("A little game", [SIZE, SIZE])
        .exit_on_esc(true).build().unwrap();

    // Model
    let player = Arc::new(Mutex::new(Player::new(40., 40., SIZE as f64)));
    let mut rocks: Vec<Rock> = Vec::new();
    (0..10).for_each(|i| {
        let pos: u32 = 35 * i + 10;
        rocks.push(Rock::new(pos as f64, pos as f64, None));
    });

    // Controller thread with own event loop
    let control_player = player.clone();
    thread::spawn(move ||{
        let mut gilrs = Gilrs::new();

        loop{
            while let Some(Event{id, event, ..}) = gilrs.next_event() {
                println!("Controller event: {:?}", event);
                control_player.lock().unwrap().update_state(event);
            }
        }
    });

    // Main event loop
    while let Some(event) = window.next() {

        // Run one update step and get position
        player.lock().unwrap().update_pos();
        let (x, y) = player.lock().unwrap().get_pos();
        let y = (SIZE as f64) - y; // Invert y-axis for intuitive experience

        // Add potential new rock
        if let Some(rock) = player.lock().unwrap().perform_action(){
            rocks.push(rock);
        }

        window.draw_2d(&event, |context, graphics| {

            // Background
            clear([0.25; 4], graphics);
            rectangle([0.1, 0.1, 0.1, 1.], [0., 0., SIZE as f64, SIZE as f64],
                      context.transform, graphics);

            // Draw players
            let transform = context.trans(x - 5., y - 5.);
            ellipse([0.6, 0.6, 0.6, 1.0], [0., 0., 10., 10.],
                      transform.transform, graphics);

            // Draw and update rocks
            rocks.iter_mut().for_each(|rock| {
                let (x, y) = rock.get_pos();
                let y = (SIZE as f64) - y;
                let mut life = 1.;
                if let Some(l) = rock.get_life(){ rock.hit(0.998); life = l; };

                let transform = context.trans(x - 8., y - 8.);
                rectangle([0.7, 0.6, 0.4, life as f32], [0., 0., 16., 16.],
                      transform.transform, graphics);
            });
        });
    }
}

