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

#[allow(unused_imports)] use stdweb::console;
#[allow(unused_imports)] use stdweb::__internal_console_unsafe;

use serde::{Serialize, Deserialize};
use serde_json;
#[allow(unused_imports)] use serde_json::{Value};

#[derive(Serialize, Deserialize, Debug)]
struct PhysicsComponent {
    position_x: f32,
    position_y: f32,
    velocity_x: f32,
    velocity_y: f32,
    max_speed: f32,
    width: i32,
    height: i32,
    jumping: i32,
    gravity: f32,
    friction: f32
}

#[derive(Serialize, Deserialize, Debug)]
struct EntityIndexes {
    entity_type: i32,
    index: usize
}

#[derive(Serialize, Deserialize, Debug)]
struct GameState {
    physical_components: Vec<PhysicsComponent>,
    entity_indexes: Vec<EntityIndexes>,
    keys_pressed: Vec<i32>
}

fn all_entities_update(component: &mut PhysicsComponent) {
    component.velocity_x *= component.friction;
    component.velocity_y += component.gravity;
    component.position_x += component.velocity_x;
    component.position_y += component.velocity_y;
}

fn update_player(player_component: &mut PhysicsComponent, keys_pressed: &Vec<i32>) {
    //keys = [32 = space, 65 = a, 68 = d, 87 = w]
    if keys_pressed.contains(&68) {
        if player_component.velocity_x < player_component.max_speed {
            player_component.velocity_x += 0.5;
        }
    } 
    if keys_pressed.contains(&65) {
        if player_component.velocity_x > -player_component.max_speed {
            player_component.velocity_x -= 0.5;
        }
    }
    if keys_pressed.contains(&32) || keys_pressed.contains(&87) {
        player_component.velocity_y -= 0.5;
    }
    all_entities_update(player_component);
}

fn update_npc() {
    
}

fn update(mut game_state: GameState) -> GameState {
    for entity in &game_state.entity_indexes {
        match entity.entity_type {
            0 => update_player(&mut game_state.physical_components[entity.index], &game_state.keys_pressed),
            1 => update_npc(),
            2 => update_npc(),
            _ => panic!()
        }
    };
    game_state
}

js_serializable!( GameState );
js_deserializable!( GameState );

#[js_export]
fn update_game_state(js_game_state: Reference) -> GameState {
    let js_game_state_deserialized = js!(
        let gameState = @{js_game_state};
        return {
            physical_components: gameState.physical_components,
            entity_indexes: gameState.entity_indexes,
            keys_pressed: gameState.keys_pressed,
        };
    );

    let game_state: GameState = js_game_state_deserialized.try_into().unwrap();
    update(game_state)
}

fn main() {
    stdweb::initialize();
    stdweb::event_loop();
}

//https://crates.io/crates/serde
//https://github.com/jakesgordon/javascript-tiny-platformer/blob/master/platformer.js
//webpack mjs
//https://kyren.github.io/2018/09/14/rustconf-talk.html