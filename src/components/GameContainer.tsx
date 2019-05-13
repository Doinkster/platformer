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
    //[x, y, width, height, velX, velY, gravity, friction, jumping(0 = false, 1 = true)]
    // player: {
    //   positionX: 5,
    //   positionY: 5,
    //   velocityX: 0,
    //   velocityY: 0,
    //   playerJumping: false,
    //   playerGravity: .3,
    //   playerFriction: .8
    // },

    // entities: {
    //   //order matters inside arrays
    //   positionsX: [5, 200, 350],
    //   positionsY: [40, 195, 100],
    //   velocitiesX: [0, 0, 0],
    //   velocitiesY: [0, 0, 0],
    //   widths: [5, 15, 30],
    //   heights: [30, 15, 5],
    //   entitiesJumping: [false, false, false],
    //   entitiesGravity: [.3, .3, .3],
    //   entitiesFriction: [.8, .8, .8]
    // },
    // level: {
    //   positionsX: [450],
    //   positionsY: [150],
    //   velocitiesX: [0, 0, 0],
    //   velocitiesY: [0, 0, 0],
    //   widths: [400],
    //   heights: [30],
    //   entitiesJumping: [false],
    //   entitiesGravity: [0],
    //   entitiesFriction: [0]
    // },
    player: [5, 5, 5, 5, 0, 0, 0.3, 0.8, 0],
    entities: [
      [5  , 40 , 5, 30, 0 , 0, 0.3, 0.8, 0],
      [200, 195, 15, 15, 0, 0, 0.3, 0.8, 0],
      [350, 100, 30, 5, 0, 0 , 0.3, 0.8, 0]
    ],
    level: [[450, 150, 0, 0, 400, 30, 0, 0, 0]],
    keys_pressed: []
  });

  const drawCanvas = () => {
    const ctx = canvasRef.current.getContext("2d");
    ctx.clearRect(0, 0, 500, 200);
    ctx.fillRect(
      gameState.player[0],
      gameState.player[1],
      gameState.player[2],
      gameState.player[3]
    )
    gameState.entities.forEach((entity) => {
      ctx.fillRect(
        entity[0],
        entity[1],
        entity[2],
        entity[3]
      );
    });
    gameState.level.forEach((level, index) => {
      ctx.fillRect(
        level[0],
        level[1],
        level[2],
        level[3]
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

// const [velocities, setVelocities] = React.useState([[]]);
// const [heights_widths, setHeights_widths] = React.useState([[]]);
