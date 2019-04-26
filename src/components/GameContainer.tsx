import * as React from 'react'

const useOneFrame = dispatchPositions => {
  const dispatchPositionsRef = React.useRef(dispatchPositions);
  const frameCountRef = React.useRef(0);

  const frame = () => {
    frameCountRef.current = requestAnimationFrame(
      frame
    );
    dispatchPositionsRef.current();
  }

  React.useEffect(() => {
    frameCountRef.current = requestAnimationFrame(frame);
    console.log('HERE@#!@#!@#');
    return () => {
      cancelAnimationFrame(frameCountRef.current);
    }
  }, []);
}

export const GameContainer = props => {
  const canvas = React.useRef(null)

  const update = (positions) => {
      const objPos = {positions: positions};
      const newGameState = props.rustFuncs.update_game_state(objPos);
      return newGameState;
  }

  const reducer = (state, action) => {
    switch (action.type) {
      case "update":
        const positions = update(state.positions);
        return positions;
      default:
        throw new Error();
    }
  }
  
  const drawCanvas = () => {
    const ctx = canvas.current.getContext('2d');
    ctx.clearRect(0, 0, 500, 200);
    positions.positions.forEach(entityPosition => {
        ctx.fillRect(entityPosition[0], entityPosition[1], 5, 5)
    })
  }

  const [positions, dispatchPositions] = React.useReducer(reducer, {positions: [[5, 5], [200, 195], [350, 100]]});
  React.useEffect(drawCanvas);
  useOneFrame(() => dispatchPositions({type: "update"}));

  return (
      <canvas ref={canvas} width="500" height="200" className="gameCanvas"></canvas>
  )
}

// http://www.somethinghitme.com/2013/01/09/creating-a-canvas-platformer-tutorial-part-one/
// https://medium.com/@lavrton/using-react-with-html5-canvas-871d07d8d753
// https://codesandbox.io/s/ojxl32jm4z


// const [velocities, setVelocities] = React.useState([[]]);
// const [heights_widths, setHeights_widths] = React.useState([[]]);
