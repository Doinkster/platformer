import React, { useState, useEffect, useReducer, useRef } from "react";
import useOneFrame from "./useOneFrame";

export const GameContainer = props => {
  //const [keys, setKeys] = useState([]);
  //const [positions, setPositions] = useState([[5, 5], [200, 195], [350, 100]]);
  const keyCodes = useRef([32, 65, 68, 87]);
  const canvasRef = useRef(null);

  const addToKeys = useRef((e) => {
    const newKeys = gameState.keys_pressed;
    if (newKeys.indexOf(e.keyCode) === -1) {
      if (keyCodes.current.indexOf(e.keyCode) !== -1) {
        newKeys.push(e.keyCode);
      }
    }
    dispatchGameState({type: "updateKeys", keys_pressed: newKeys});
  });

  const removeFromKeys = useRef((e) => {
    const newKeys = gameState.keys_pressed;
    const index = newKeys.indexOf(e.keyCode);
    if (index !== -1) {
      newKeys.splice(index, 1);
    }
    dispatchGameState({type: "updateKeys", keys_pressed: newKeys});
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
      case "updatePositions":
        const newGameState = updateGameState(state);
        return newGameState;
      case "updateKeys":
        return {...state, keys_pressed: action.keys_pressed}
      default:
        throw new Error();
    }
  };

  const [gameState, dispatchGameState] = useReducer(gameStateReducer, {
    positions: [[5, 5], [200, 195], [350, 100]],
    keys_pressed: []
  });

  const drawCanvas = () => {
    const ctx = canvasRef.current.getContext("2d");
    ctx.clearRect(0, 0, 500, 200);
    gameState.positions.forEach(entityPosition => {
      ctx.fillRect(entityPosition[0], entityPosition[1], 5, 5);
    });
  };

  useEffect(drawCanvas);
  useOneFrame(() => dispatchGameState({type: "updatePositions"}));

  return (
    <canvas ref={canvasRef} width="500" height="200" className="gameCanvas" />
  );
};

// http://www.somethinghitme.com/2013/01/09/creating-a-canvas-platformer-tutorial-part-one/
// https://medium.com/@lavrton/using-react-with-html5-canvas-871d07d8d753
// https://codesandbox.io/s/ojxl32jm4z

// const [velocities, setVelocities] = React.useState([[]]);
// const [heights_widths, setHeights_widths] = React.useState([[]]);
