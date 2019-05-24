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
    width: f32,
    height: f32,
    jumping: i32,
    gravity: f32,
    friction: f32
}

#[derive(Serialize, Deserialize, Debug)]
struct EntityIndex {
    entity_type: i32,
    index: usize
}

#[derive(Serialize, Deserialize, Debug)]
struct GameState {
    physical_entitys: Vec<PhysicsComponent>,
    entity_indexes: Vec<EntityIndex>,
    keys_pressed: Vec<i32>
}

fn update_position(entity: &mut PhysicsComponent) {
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

fn update_player(player_entity: &mut PhysicsComponent, keys_pressed: &Vec<i32>) {
    //keys = [32 = space, 65 = a, 68 = d, 87 = w]
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
}

fn update_npc() {
    
}

fn get_collision_direction() {
    let vector_x = (entity_a.position_x + (entity_a.width / 2.0)) - (entity_b.position_x + (entity_b.width / 2.0));
    let vector_y = (entity_a.position_y + (entity_a.width / 2.0)) - (entity_b.position_y + (entity_b.width / 2.0));
    let half_widths_added = (entity_a.width / 2.0) + (entity_b.width / 2.0);
    let half_heights_added = (entity_a.height / 2.0) + (entity_b.height / 2.0);
    let collision_direction = -1;
}

fn compare_player_to_others(entity_a: &mut PhysicsComponent, entity_b: &mut PhysicsComponent, entity_b_type: i32) {
    if entity_b_type == 1 || entity_b_type == 2 {
        

    }
}

fn compare_npcs_to_others() {

}

fn compare_levels_to_others() {

}

fn apply_gravity_and_friction(entity: &mut PhysicsComponent) {
    entity.velocity_x *= entity.friction;
    entity.velocity_y += entity.gravity;
}

fn check_collisions(entity: &mut PhysicsComponent, entity_index: &mut EntityIndex, entitys: &mut Vec<PhysicsComponent>, indexes: &Vec <EntityIndex>) {
    //direction -> 0 = left, 1 = top, 2 = right, 3 = bottom
    for other_entity_index in indexes {
        if entity_index.index != other_entity_index.index {
            let other_entity = &mut entitys[other_entity_index.index];
            let collision_direction = get_collision_direction(entity, other_entity);
            match entity_index.entity_type {
                0 => compare_player_to_others(entity, other_entity, other_entity_index.entity_type),
                1 => compare_npcs_to_others(),
                2 => compare_levels_to_others(),
                _ => panic!()
            }
        }
    };
}

fn update(mut game_state: GameState) -> GameState {
    //works without mut? V
    for entity_index in &mut game_state.entity_indexes {
        let entity = &mut game_state.physical_entitys[entity_index.index];
        apply_gravity_and_friction(entity);
        check_collisions(entity, &mut entity_index, &mut game_state.physical_entitys, &game_state.entity_indexes);
        match entity_index.entity_type {
            0 => update_player(entity, &game_state.keys_pressed),
            1 => update_npc(),
            2 => update_npc(),
            _ => panic!()
        }
        update_position(entity);
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