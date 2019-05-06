// #![allow(unused_variables)]
// #![allow(dead_code)]

use stdweb;
use stdweb::js_serializable;
use stdweb::js_deserializable;
use stdweb::__js_serializable_boilerplate;
use stdweb::__js_serializable_serde_boilerplate;
use stdweb::__js_deserializable_serde_boilerplate;
use stdweb_internal_macros;
use stdweb_internal_runtime;
use stdweb_internal_macros::js_export;
use stdweb::js;
//#[allow(unused_imports)]
use stdweb::_js_impl;
use stdweb::__js_raw_asm;
use stdweb::{Reference};
use stdweb::unstable::TryInto;

use serde::{Serialize, Deserialize};
use serde_json;
use serde_json::{Value};

#[derive(Serialize, Deserialize, Debug)]
struct GameState {
    positions: Vec<(i32, i32)>,
    keys_pressed: Vec<i32>
    // velocities: Vec<(i32, i32)>,
    // height_widths: Vec<(i32, i32)>
}

impl GameState {
    fn decrease_x(&mut self) {
        self.positions[0].0 -= 1;
    }

    fn increase_x(&mut self) {
        self.positions[0].0 += 1;
    }

    fn decrease_y(&mut self) {
        self.positions[0].1 += 1;
    }

    fn increase_y(&mut self) {
        self.positions[0].1 -= 1;
    }

    fn update_player(&mut self) {
        //keys = [32, 65, 68, 87]
        if self.keys_pressed.contains(&68) {
            self.increase_x();
        } 
        if self.keys_pressed.contains(&65) {
            self.decrease_x();
        }
        if self.keys_pressed.contains(&32) || self.keys_pressed.contains(&87) {
            self.increase_y();
        }
    }

    fn update_positions(&mut self) -> &Vec<(i32, i32)> {
        // for tuple in &mut self.positions {
        //     tuple.0 += 1;
        //     tuple.1 += 1;
        // }
        &self.update_player();
        //update_npcs();
        //self.check_collisions();
        &self.positions
    }

    
}

js_serializable!( GameState );
js_deserializable!( GameState );


#[js_export]
fn update_game_state(js_game_state: Reference) -> GameState {
    let js_game_state_deserialized = js!(
        let gameState = @{js_game_state};
        console.log("GAMESTATE", gameState);
        return {
            positions: gameState.positions,
            keys_pressed: gameState.keys_pressed,
            player_jumping: gameState.player_jumping
        };
    );

    let mut game_state: GameState = js_game_state_deserialized.try_into().unwrap();
    game_state.update_positions();
    game_state
}

fn main() {
    stdweb::initialize();
    stdweb::event_loop();
}

//https://crates.io/crates/serde
//https://github.com/jakesgordon/javascript-tiny-platformer/blob/master/platformer.js
//webpack mjs
//https://kyren.github.io/2018/09/14/rustconf-talk.html