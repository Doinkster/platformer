import * as React from 'react'
import rustFuncs from "../../static/scroller.js"
import { GameContainer } from "./index"

export const Main = props => {
  const [rustFuncsObject, setRustFuncsObject] = React.useState(0);

  //console.log("rustFuncsObject", rustFuncsObject);
  //console.log("setRustFuncsObject", setRustFuncsObject);

  const setRustFunctions = async () => {
      const rustFuncsObject = await rustFuncs;
      setRustFuncsObject(() => rustFuncsObject);
  };

  React.useEffect(() => {
      setRustFunctions();
  }, []);

  return (
    <div>
      {
        (rustFuncsObject === 0) ? (null) : (<GameContainer rustFuncs={rustFuncsObject}></GameContainer>)
      }
    </div>
  );
}
