import React, { useEffect, useReducer, useRef } from "react";
import useOneFrame from "./useOneFrame";
import useKeyInput from "./useKeyInput";

export const GameContainer = props => {
  const canvasRef = useRef(null);

  const update = positions => {
    const objPos = { positions: positions };
    const newGameState = props.rustFuncs.update_game_state(objPos);
    return newGameState;
  };

  const reducer = (state, action) => {
    switch (action.type) {
      case "update":
        const positions = update(state.positions);
        return positions;
      default:
        throw new Error();
    }
  };

  const drawCanvas = () => {
    const ctx = canvasRef.current.getContext("2d");
    ctx.clearRect(0, 0, 500, 200);
    positions.positions.forEach(entityPosition => {
      ctx.fillRect(entityPosition[0], entityPosition[1], 5, 5);
    });
  };

  const [positions, dispatchPositions] = useReducer(reducer, {
    positions: [[5, 5], [200, 195], [350, 100]]
  });
  useEffect(drawCanvas);
  useKeyInput();
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
