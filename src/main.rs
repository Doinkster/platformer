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

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PhysicsComponent {
    positionX: f32,
    positionY: f32,
    velocityX: f32,
    velocityY: f32,
    width: f32,
    height: f32,
    jumping: bool,
    gravity: f32,
    friction: f32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct EntityIndexes {
    entity_type: Option<u32>,
    index: usize
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GameState {
    phyiscal_components: Vec<PhysicsComponent>,
    entity_indexes: Vec<EntityIndexes>,
    keys_pressed: Vec<i32>
}

impl<'a> GameState {
    fn update_player(&mut self, player_component: &mut PhysicsComponent) {
        //keys = [32, 65, 68, 87]
        if self.keys_pressed.contains(&68) {
            player_component.positionX += 1.0;
        } 
        if self.keys_pressed.contains(&65) {
            player_component.positionX -= 1.0;
        }
        if self.keys_pressed.contains(&32) || self.keys_pressed.contains(&87) {
            player_component.positionY += 1.0;
        }
    }

    fn update_npc(&mut self) {
        
    }


    fn update(mut self, mut phyiscal_components: Vec<PhysicsComponent>, entity_indexes: Vec<EntityIndexes>) -> GameState {
        for entity in entity_indexes {
            // match entity.entity_type {
            //     Some(0) => &self.update_player(phyiscal_components[entity.index]),
            //     Some(1) => &self.update_npc(),
            //     Some(_) => panic!(),
            //     None => panic!()
            // }
            if entity.entity_type == Some(0) {
                self.update_player(&mut phyiscal_components[entity.index])
            }
        };
        self
    }
}

js_serializable!( GameState );
js_deserializable!( GameState );

#[js_export]
fn update_game_state(js_game_state: Reference) -> GameState {
    let js_game_state_deserialized = js!(
        let gameState = @{js_game_state};
        console.log("GAMESTATE", gameState.keys_pressed);
        return {
            phyiscal_components: gameState.phyiscal_components,
            entity_indexes: gameState.entity_indexes,
            keys_pressed: gameState.keys_pressed,
        };
    );

    let mut game_state: GameState = js_game_state_deserialized.try_into().unwrap();
    let mut phyiscal_components = game_state.phyiscal_components.clone();
    let entity_indexes = game_state.entity_indexes.clone();
    game_state.update(phyiscal_components, entity_indexes)
    //game_state
}

fn main() {
    stdweb::initialize();
    stdweb::event_loop();
}

//https://crates.io/crates/serde
//https://github.com/jakesgordon/javascript-tiny-platformer/blob/master/platformer.js
//webpack mjs
//https://kyren.github.io/2018/09/14/rustconf-talk.html