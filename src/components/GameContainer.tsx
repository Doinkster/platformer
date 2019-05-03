import React, { useState, useEffect, useReducer, useRef } from "react";
import useOneFrame from "./useOneFrame";

export const GameContainer = props => {
  const [keys, setKeys] = useState([]);
  //const [positions, setPositions] = useState([[5, 5], [200, 195], [350, 100]]);
  const keyCodes = useRef([32, 65, 68]);
  const canvasRef = useRef(null);

  const addToKeys = useRef((e) => {
    const newKeys = keys;
    if (newKeys.indexOf(e.keyCode) === -1) {
      if (keyCodes.current.indexOf(e.keyCode) !== -1) {
        newKeys.push(e.keyCode);
      }
    }
    setKeys(newKeys);
  });

  const removeFromKeys = useRef((e) => {
    const newKeys = keys;
    const index = newKeys.indexOf(e.keyCode);
    if (index !== -1) {
      newKeys.splice(index, 1);
    }
    setKeys(newKeys);
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
    // gameState = { 
    //   positions: positions, 
    //   keys_pressed: keys 
    // };
    console.log("before", gameState)
    const newGameState = props.rustFuncs.update_game_state(gameState);
    console.log("after", gameState)
    return newGameState;
  };

  const gameStateReducer = (state, action) => {
    switch (action.type) {
      case "update":
        const newGameState = updateGameState(state);
        return newGameState;
      default:
        throw new Error();
    }
  };

  const [gameState, dispatchGameState] = useReducer(gameStateReducer, {
    positions: [[5, 5], [200, 195], [350, 100]],
    keys_pressed: keys
  });

  const drawCanvas = () => {
    const ctx = canvasRef.current.getContext("2d");
    ctx.clearRect(0, 0, 500, 200);
    gameState.positions.forEach(entityPosition => {
      ctx.fillRect(entityPosition[0], entityPosition[1], 5, 5);
    });
  };

  useEffect(drawCanvas);
  useOneFrame(() => dispatchGameState({ type: "update" }));

  return (
    <canvas ref={canvasRef} width="500" height="200" className="gameCanvas" />
  );
};

// http://www.somethinghitme.com/2013/01/09/creating-a-canvas-platformer-tutorial-part-one/
// https://medium.com/@lavrton/using-react-with-html5-canvas-871d07d8d753
// https://codesandbox.io/s/ojxl32jm4z

// const [velocities, setVelocities] = React.useState([[]]);
// const [heights_widths, setHeights_widths] = React.useState([[]]);
