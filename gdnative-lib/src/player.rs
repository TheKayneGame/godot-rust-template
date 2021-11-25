use gdnative::api::{Sprite};
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Sprite)]
pub struct Player;

#[methods]
impl Player {
    fn new(_owner: &Sprite) -> Self {
        Player
    }

    #[export]
    fn _ready(&self, _owner: &Sprite) {        
        godot_print!("start");
        let mut test: i8 = 100;
        for n in 1..11 {
            test = n * n;
            godot_print!("{} {}", n, test);

        }
    }

    #[export]
    fn _process(&mut self, owner: &Sprite, delta: f32) {
        

        let input = Input::godot_singleton();
        let mut velocity = Vector2::new(0.0, 0.0);

        if Input::is_action_pressed(input, "ui_right") {
            velocity.x += 10.0
        }
        if Input::is_action_pressed(input, "ui_left") {
            velocity.x -= 10.0
        }
        if Input::is_action_pressed(input, "ui_down") {
            velocity.y += 10.0
        }
        if Input::is_action_pressed(input, "ui_up") {
            velocity.y -= 10.0
        }

        let change = velocity * delta;
        let position = owner.global_position() + change;

        owner.set_global_position(position);
    }
}