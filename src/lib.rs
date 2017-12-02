extern crate piston_window;
extern crate gilrs;
extern crate num;

use piston_window::*;
use gilrs::{Gilrs, Button, Event, EventType, Axis};


pub struct Player{
    position: (f64, f64),
    velocity: (f64, f64),
    direction: (f64, f64),
    bound: f64,
    speed: f64,
    action: Action,
}

#[derive(Clone, Debug)]
enum Action{
    Rock,
    No,
}

impl Player{
    pub fn new(x: f64, y: f64, bound: f64) -> Self{
        Player{ position: (x, y), velocity: (0., 0.), direction: (0., 0.), bound: bound, speed: 0.5, action: Action::No }
    }

    pub fn get_pos(&self) -> (f64, f64){
        self.position
    }

    pub fn update_pos(&mut self){
        self.position.0 += self.velocity.0 * self.speed;
        self.position.1 += self.velocity.1 * self.speed;

        self.position.0 = num::clamp(self.position.0, 0.0, self.bound);
        self.position.1 = num::clamp(self.position.1, 0.0, self.bound);
    }

    pub fn perform_action(&mut self) -> Option<Rock>{
        let action = self.action.clone();
        match action{
            Action::Rock => {
                self.action = Action::No;
                let x = self.position.0 + self.direction.0 * 20.;
                let y = self.position.1 + self.direction.1 * 20.;
                Some(Rock::new(x, y, Some(0.8)))
            },
            Action::No => None,
        }
    }

    pub fn abs_velocity_sq(&self) -> f64 {
        self.velocity.0 * self.velocity.0 + self.velocity.1 * self.velocity.1
    }

    pub fn update_state(&mut self, event: EventType){
        if let EventType::AxisChanged(axis, val, _) = event{
            match axis{
                Axis::LeftStickX => self.velocity.0 = val as f64,
                Axis::LeftStickY => self.velocity.1 = val as f64,
                Axis::RightStickX => self.direction.0 = val as f64,
                Axis::RightStickY => self.direction.1 = val as f64,
                _ => (),
            }
            if self.abs_velocity_sq() < 0.1 {
                self.velocity.0 = 0.0;
                self.velocity.1 = 0.0;
            }
        };
        if let EventType::ButtonPressed(button, _) = event{
            self.action = Action::Rock;
            println!("ROCK");
        };
    }
}

pub struct Rock{
    position: (f64, f64),
    life: Option<f64>,
}

impl Rock{
    pub fn new(x: f64, y: f64, life: Option<f64>) -> Self{
        Rock{ position: (x, y), life: life }
    }

    pub fn get_pos(&self) -> (f64, f64){
        self.position
    }

    pub fn get_life(&self) -> Option<f64>{
        self.life
    }

    pub fn hit(&mut self, decay: f64){
        if let Some(i) = self.life{
            self.life = Some(i * decay);
        };
    }
}




