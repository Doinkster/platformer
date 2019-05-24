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
    physical_entitys: Vec<PhysicsComponent>,
    entity_indexes: Vec<EntityIndexes>,
    keys_pressed: Vec<i32>
}

fn update_position(entity: &mut PhysicsComponent) {
    entity.velocity_x *= entity.friction;
    entity.velocity_y += entity.gravity;
    entity.position_x += entity.velocity_x;
    entity.position_y += entity.velocity_y;
    
    if entity.position_x >= 500.0 {
        entity.position_x = 500.0 - entity.width as f32;
    } else if entity.position_x <= 0.0 {
        entity.position_x = 0.0;
    }

    if entity.position_y >= 150.0 - entity.height as f32 {
        entity.position_y = 150.0 - entity.height as f32;
        entity.jumping = 0;
    }
}

fn update_player(game_state: &mut GameState, index: usize) {
    check_collisions(&mut game_state.physical_entitys, &game_state.entity_indexes);
    //keys = [32 = space, 65 = a, 68 = d, 87 = w]
    let player_entity = game_state.physical_entitys[index];
    let keys_pressed = game_state.keys_pressed;
    if keys_pressed.contains(&32) || keys_pressed.contains(&87) {
        if player_entity.jumping == 0 {
            player_entity.jumping = 1;
            player_entity.velocity_y = -player_entity.max_speed * 2.0;
        }
    }
    if keys_pressed.contains(&68) {
        if player_entity.velocity_x < player_entity.max_speed {
            player_entity.velocity_x += 0.5;
        }
    } 
    if keys_pressed.contains(&65) {
        if player_entity.velocity_x > -player_entity.max_speed {
            player_entity.velocity_x -= 0.5;
        }
    }
    update_position(player_entity);
}

fn update_npc() {
    
}

fn check_collisions(entitys: &mut Vec<PhysicsComponent>, indexes: &Vec <EntityIndexes>) {
    for entity in indexes {
        if entity.entity_type == 0 {
            
        }
    }
}

fn update(mut game_state: GameState) -> GameState {
    //check_collisions(&mut game_state.physical_entitys, &game_state.entity_indexes);
    //works without mut?
    for entity in &mut game_state.entity_indexes {
        match entity.entity_type {
            //0 => update_player(&mut game_state.physical_entitys[entity.index], &game_state.keys_pressed),
            0 => update_player(&mut game_state, entity.index),
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
            physical_entitys: gameState.physical_entitys,
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
//http://www.somethinghitme.com/2013/01/09/creating-a-canvas-platformer-tutorial-part-one/
//http://www.somethinghitme.com/2013/04/16/creating-a-canvas-platformer-tutorial-part-tw/
//webpack mjs
//https://kyren.github.io/2018/09/14/rustconf-talk.html