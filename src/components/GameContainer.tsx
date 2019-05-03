import React, { useState, useEffect, useReducer, useRef } from "react";
import useOneFrame from "./useOneFrame";

export const GameContainer = props => {
  const [keys, setKeys] = useState([]);
  const canvasRef = useRef(null);
  
  const addToKeys = e => {
    const newKeys = keys;
    const keysToUse = [65, 68, 32];
    if (newKeys.indexOf(e.keyCode) === -1) {
      if (keysToUse.indexOf(e.keyCode) !== -1) {
        newKeys.push(e.keyCode);
      }
    }
    setKeys(newKeys);
  };

  const removeFromKeys = e => {
    const newKeys = keys;
    const index = newKeys.indexOf(e.keyCode);
    if (index !== -1) {
      newKeys.splice(index, 1);
    }
    setKeys(newKeys);
  };

  useEffect(() => {
    window.addEventListener("keydown", addToKeys);
    window.addEventListener("keyup", removeFromKeys);
    return () => {
      window.removeEventListener("keydown", addToKeys);
      window.removeEventListener("keyup", removeFromKeys);
    };
  }, []);

  const updatePositions = positions => {
    const objPos = { positions: positions };
    const newGameState = props.rustFuncs.update_game_state(objPos);
    return newGameState;
  };

  const positionsReducer = (state, action) => {
    switch (action.type) {
      case "update":
        const positions = updatePositions(state.positions);
        return positions;
      default:
        throw new Error();
    }
  };

  const [positions, dispatchPositions] = useReducer(positionsReducer, {
    positions: [[5, 5], [200, 195], [350, 100]]
  });

  const drawCanvas = () => {
    const ctx = canvasRef.current.getContext("2d");
    ctx.clearRect(0, 0, 500, 200);
    positions.positions.forEach(entityPosition => {
      ctx.fillRect(entityPosition[0], entityPosition[1], 5, 5);
    });
  };

  useEffect(drawCanvas);
  useOneFrame(() => dispatchPositions({ type: "update" }));

  return (
    <canvas ref={canvasRef} width="500" height="200" className="gameCanvas" />
  );
};

// http://www.somethinghitme.com/2013/01/09/creating-a-canvas-platformer-tutorial-part-one/
// https://medium.com/@lavrton/using-react-with-html5-canvas-871d07d8d753
// https://codesandbox.io/s/ojxl32jm4z

// const [velocities, setVelocities] = React.useState([[]]);
// const [heights_widths, setHeights_widths] = React.useState([[]]);
