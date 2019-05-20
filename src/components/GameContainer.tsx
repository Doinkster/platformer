import React, { useState, useEffect, useReducer, useRef } from "react";
import useOneFrame from "./useOneFrame";

export const GameContainer = props => {
  //const [keys, setKeys] = useState([]);
  //const [positions, setPositions] = useState([[5, 5], [200, 195], [350, 100]]);
  const keyCodes = useRef([32, 65, 68, 87]);
  const canvasRef = useRef(null);

  const addToKeys = useRef(e => {
    const newKeys = gameState.keys_pressed;
    if (newKeys.indexOf(e.keyCode) === -1) {
      if (keyCodes.current.indexOf(e.keyCode) !== -1) {
        newKeys.push(e.keyCode);
      }
    }
    dispatchGameState({ type: "updateKeys", keys_pressed: newKeys });
  });

  const removeFromKeys = useRef(e => {
    const newKeys = gameState.keys_pressed;
    const index = newKeys.indexOf(e.keyCode);
    if (index !== -1) {
      newKeys.splice(index, 1);
    }
    dispatchGameState({ type: "updateKeys", keys_pressed: newKeys });
  });

  useEffect(() => {
    window.addEventListener("keydown", addToKeys.current);
    window.addEventListener("keyup", removeFromKeys.current);
    return () => {
      window.removeEventListener("keydown", addToKeys.current);
      window.removeEventListener("keyup", removeFromKeys.current);
    };
  }, []);

  const updateGameState = gameState => {
    const newGameState = props.rustFuncs.update_game_state(gameState);
    return newGameState;
  };

  const gameStateReducer = (state, action) => {
    switch (action.type) {
      case "updateGameState":
        const newGameState = updateGameState(state);
        return newGameState;
      case "updateKeys":
        return { ...state, keys_pressed: action.keys_pressed };
      default:
        throw new Error();
    }
  };

  const [gameState, dispatchGameState] = useReducer(gameStateReducer, {
    physical_components: [
      {
        position_x: 5.0,
        position_y: 5.0,
        velocity_x: 0.0,
        velocity_y: 0.0,
        max_speed: 5,
        width: 5,
        height: 5,
        jumping: 0,
        gravity: 0.3,
        friction: 0.8
      },
      {
        position_x: 200.0,
        position_y: 195.0,
        velocity_x: 0.0,
        velocity_y: 0.0,
        max_speed: 5,
        width: 5,
        height: 30,
        jumping: 0,
        gravity: 0.3,
        friction: 0.8
      },
      {
        position_x: 350.0,
        position_y: 105.0,
        velocity_x: 0.0,
        velocity_y: 0.0,
        max_speed: 5,
        width: 15,
        height: 15,
        jumping: 0,
        gravity: 0.3,
        friction: 0.8
      },
      {
        position_x: 5.0,
        position_y: 5.0,
        velocity_x: 0.0,
        velocity_y: 0.0,
        max_speed: 5,
        width: 30,
        height: 5,
        jumping: 0,
        gravity: 0.3,
        friction: 0.8
      },
      {
        position_x: 450.0,
        position_y: 150.0,
        velocity_x: 0.0,
        velocity_y: 0.0,
        max_speed: 5,
        width: 400,
        height: 30,
        jumping: 0,
        gravity: 0.3,
        friction: 0.8
      }
    ],
    entity_indexes: [
      //entity_type: 0 = player, 1 = npc, 2 = level
      { entity_type: 0, index: 0 },
      { entity_type: 1, index: 1 },
      { entity_type: 1, index: 2 },
      { entity_type: 1, index: 3 },
      { entity_type: 2, index: 4 }
    ],
    keys_pressed: [],
    //canvas: {height: .....}
  });

  const drawCanvas = () => {
    const ctx = canvasRef.current.getContext("2d");
    ctx.clearRect(0, 0, 500, 200);
    gameState.physical_components.forEach(entity => {
      ctx.fillRect(
        entity.position_x,
        entity.position_y,
        entity.width,
        entity.height
      );
    });
  };

  useEffect(drawCanvas);
  useOneFrame(() => dispatchGameState({ type: "updateGameState" }));

  return (
    <canvas ref={canvasRef} width="500" height="200" className="gameCanvas" />
  );
};

// http://www.somethinghitme.com/2013/01/09/creating-a-canvas-platformer-tutorial-part-one/
// https://medium.com/@lavrton/using-react-with-html5-canvas-871d07d8d753
// https://codesandbox.io/s/ojxl32jm4z
