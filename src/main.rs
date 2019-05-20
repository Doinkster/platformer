// #![allow(unused_variables)]
// #![allow(dead_code)]

use stdweb;
use stdweb::js_serializable;
use stdweb::js_deserializable;
use stdweb::__js_serializable_boilerplate;
use stdweb::__js_serializable_serde_boilerplate;
use stdweb::__js_deserializable_serde_boilerplate;
use stdweb_internal_macros;
#[allow(unused_imports)] use stdweb_internal_runtime;
use stdweb_internal_macros::js_export;
use stdweb::js;
use stdweb::_js_impl;
use stdweb::__js_raw_asm;
use stdweb::{Reference};
use stdweb::unstable::TryInto;

use serde::{Serialize, Deserialize};
use serde_json;
#[allow(unused_imports)] use serde_json::{Value};

#[derive(Serialize, Deserialize, Debug)]
struct GameState {
    //positions: Vec<(i32, i32)>,
    player: Vec<Vec<f32>>,
    npcs: Vec<Vec<f32>>,
    level: Vec<Vec<f32>>,
    keys_pressed: Vec<i32>
    // velocities: Vec<(i32, i32)>,
    // height_widths: Vec<(i32, i32)>
}

impl<'a> GameState {
    fn increase_entity_x(&mut self, entity: &'a mut Vec<f32>) -> &'a mut Vec<f32> {
        //self.player[0][0] += 1.0;
        //const new_position = self.increase_int(entity[0][0]);
        entity[0] = entity[0] + 1.0;
        entity
    }

    fn decrease_entity_x(&mut self, entity: &'a mut Vec<f32>) -> &'a mut Vec<f32> {
        //self.player[0][0] += 1.0;
        //const new_position = self.increase_int(entity[0][0]);
        entity[0] = entity[0] - 1.0;
        entity
    }

    // fn increase_entity_y(&self, entity: Vec<f32>) -> Vec<f32> {
    //     //self.player[0][0] += 1.0;
    //     //const new_position = self.increase_int(entity[0][0]);
    //     entity[1] = entity[1] + 1.0;
    //     entity
    // }

    // fn decrease_entity_y(&self, entity: Vec<f32>) -> Vec<f32> {
    //     //self.player[0][0] += 1.0;
    //     //const new_position = self.increase_int(entity[0][0]);
    //     entity[1] = entity[1] - 1.0;
    //     entity
    // }

    // fn decrease_y(&mut self) {
    //     self.player[1] += 1.0;
    // }

    // fn increase_y(&mut self) {
    //     self.player[1] -= 1.0;
    // }

    fn update_players(&mut self) {
        //keys = [32, 65, 68, 87]
        if self.keys_pressed.contains(&68) {
            //&mut self.increase_entity_x(&mut self.player[0]);
            self.player[0] = self.increase_entity_x(&mut self.player[0]).to_vec();
        } 
        if self.keys_pressed.contains(&65) {
            self.player[0] = self.decrease_entity_x(&mut self.player[0]).to_vec();
        }
        // if self.keys_pressed.contains(&32) || self.keys_pressed.contains(&87) {
        //     &mut self.increase_entity_y(self.player[1]);
        // }
    }

    fn update_npcs() {
        
    }

    fn update_positions(&mut self) -> &Vec<f32> {
        &self.update_players();
        //update_npcs();
        //self.check_collisions();
        &self.player[0]
    }

    fn update(&mut self) {

    }
}

js_serializable!( GameState );
js_deserializable!( GameState );

// player: [5, 5, 5, 5, 0, 0, 0.3, 0.8, 0],
// npcs: [
//   [5  , 40 , 5 , 30, 0, 0, 0.3, 0.8, 0],
//   [200, 195, 15, 15, 0, 0, 0.3, 0.8, 0],
//   [350, 100, 30, 5 , 0, 0, 0.3, 0.8, 0]
// ],
// level: [[450, 150, 0, 0, 400, 30, 0, 0, 0]],

// phyiscal_components: [
//     [5, 5, 5, 5, 0, 0, 0.3, 0.8, 0], 
//     [5  , 40 , 5 , 30, 0, 0, 0.3, 0.8, 0],
//     [200, 195, 15, 15, 0, 0, 0.3, 0.8, 0],
//     [350, 100, 30, 5 , 0, 0, 0.3, 0.8, 0],
//     [450, 150, 0, 0, 400, 30, 0, 0, 0]
// ]

// entity_index: [
//     {type: player, index: 0},
//     {type: npc, index: 1}
// ]

#[js_export]
fn update_game_state(js_game_state: Reference) -> GameState {
    let js_game_state_deserialized = js!(
        let gameState = @{js_game_state};
        console.log("GAMESTATE", gameState.keys_pressed);
        return {
            player: gameState.player,
            npcs: gameState.npcs,
            level: gameState.level,
            keys_pressed: gameState.keys_pressed,
        };
    );

    let mut game_state: GameState = js_game_state_deserialized.try_into().unwrap();
    game_state.update();
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