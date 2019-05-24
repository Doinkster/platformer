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
    position_x: i32,
    position_y: i32,
    velocity_x: i32,
    velocity_y: i32,
    max_speed: i32,
    width: i32,
    height: i32,
    jumping: bool,
    grounded: bool,
    gravity: i32,
    friction: i32
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

fn update_position(entity: &mut PhysicsComponent, entity_index: &EntityIndex) {
    //console!(log, "index", entity_index.index as f32);
    //console!(log, "type", entity_index.entity_type as f32);
    console!(log, "grounded", entity.grounded);
    if entity_index.entity_type == 0 && entity.grounded == true {
        entity.velocity_y = 0;
    }
    entity.position_x += entity.velocity_x;
    entity.position_y += entity.velocity_y;
}

fn update_player(player_entity: &mut PhysicsComponent, keys_pressed: &Vec<i32>) {
    //keys = [32 = space, 65 = a, 68 = d, 87 = w]player_entity.jumping);player_entity.grounded);keys_pressed);
    if keys_pressed.contains(&32) || keys_pressed.contains(&87) {
        if player_entity.jumping == false && player_entity.grounded == true {
            player_entity.jumping = true;
            player_entity.grounded = false;
            player_entity.velocity_y = -player_entity.max_speed * 2;
        }
    }
    if keys_pressed.contains(&68) {
        if player_entity.velocity_x < player_entity.max_speed {
            player_entity.velocity_x += 1;
        }
    } 
    if keys_pressed.contains(&65) {
        if player_entity.velocity_x > -player_entity.max_speed {
            player_entity.velocity_x -= 1;
        }
    }
}

// fn update_npc() {
    
// }

fn get_collision_direction(entitys: &mut Vec<PhysicsComponent>, index: usize, other_index: usize) -> i32 {
    //vector == actual distance between objects
    let vector_x = (entitys[index].position_x + (entitys[index].width / 2)) - 
        (entitys[other_index].position_x + (entitys[other_index].width / 2));
    let vector_y = (entitys[index].position_y + (entitys[index].width / 2)) - 
        (entitys[other_index].position_y + (entitys[other_index].width / 2));

    //half widths == minimum distance needed between objects before collision
    let half_widths_added = (entitys[index].width / 2) + (entitys[other_index].width / 2);
    let half_heights_added = (entitys[index].height / 2) + (entitys[other_index].height / 2);
    let mut collision_direction = -1;

    //if collision
    if vector_x.abs() < half_widths_added && vector_y.abs() < half_heights_added {
        let x_magnitude = half_widths_added - vector_x.abs();
        let y_magnitude = half_heights_added - vector_y.abs();

        //collision_direction -> 0=left, 1=top, 2=right, 3=bottom
        if x_magnitude >= y_magnitude {
            if vector_y > 0 {
                collision_direction = 1;
                entitys[index].position_y += y_magnitude;
            } else {
                collision_direction = 3;
                entitys[index].position_y -= y_magnitude;
            }
        } else {
            if vector_x > 0 {
                collision_direction = 0;
                entitys[index].position_x += x_magnitude;
            } else {
                collision_direction = 2;
                entitys[index].position_x -= x_magnitude;
            }
        }
    }
    collision_direction
}

fn compare_player_to_others_with_collision_direction(
    entitys: &mut Vec<PhysicsComponent>, 
    index: &EntityIndex, 
    other_index: &EntityIndex, 
    direction: i32) 
{
    //collision_direction -> 0=left, 1=top, 2=right, 3=bottom
    //console!(log, "HERE", index.index as f32);
    if other_index.entity_type == 2 {
        if direction == 0 || direction == 2 {
            entitys[index.index].velocity_x = 0;
            entitys[index.index].jumping = false;
        }
        if direction == 1 {
            entitys[index.index].velocity_y = -1;
        }
        if direction == 3 {
            //entitys[index.index].velocity_y = 0;
            entitys[index.index].jumping = false;
            entitys[index.index].grounded = true;
        }
    }
}

fn compare_npcs_to_others_with_collision_direction() {

}

fn compare_levels_to_others_with_collision_direction() {

}

fn apply_gravity_and_friction(entity: &mut PhysicsComponent) {
    entity.velocity_x *= entity.friction / 10;
    entity.velocity_y += entity.gravity / 10;
}

fn check_collisions(
    physical_entitys: &mut Vec<PhysicsComponent>, 
    entity_indexes: &Vec<EntityIndex>, 
    index: &usize, 
    other_index: &usize) 
{
    let collision_direction = get_collision_direction(physical_entitys, *index, *other_index);
    match entity_indexes[*index].entity_type {
        0 => compare_player_to_others_with_collision_direction(
            physical_entitys, 
            &entity_indexes[*index], 
            &entity_indexes[*other_index], 
            collision_direction),
        1 => compare_npcs_to_others_with_collision_direction(),
        2 => compare_levels_to_others_with_collision_direction(),
        _ => panic!()
    }
    
}

fn update(mut game_state: GameState) -> GameState {
    for index in 0..game_state.entity_indexes.len() {
        // match game_state.entity_indexes[index].entity_type
        if game_state.entity_indexes[index].entity_type == 0 {
            update_player(&mut game_state.physical_entitys[index], &game_state.keys_pressed);
            game_state.physical_entitys[index].grounded = false;
        }
        apply_gravity_and_friction(&mut game_state.physical_entitys[index]);
        for other_index in 0..game_state.entity_indexes.len() {
            if index != other_index { 
                check_collisions(&mut game_state.physical_entitys, &game_state.entity_indexes, &index, &other_index);
            }
        }
        update_position(&mut game_state.physical_entitys[index], &game_state.entity_indexes[index]);
    }

    game_state
}

js_serializable!( GameState );
js_deserializable!( GameState );

#[js_export]
fn update_game_state(js_game_state: Reference) -> GameState {
    let js_game_state_deserialized = js!(
        let gameState = @{js_game_state};
        //console.log("GAME STATE", gameState);
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