import React, { useEffect } from "react";
import rustFuncs from "../../static/scroller.js";
import { GameContainer } from "./index";

export const Main = props => {
  const [rustFuncsObject, setRustFuncsObject] = React.useState(0);

  const setRustFunctions = async () => {
    const rustFuncsObject = await rustFuncs;
    setRustFuncsObject(() => rustFuncsObject);
  };

  useEffect(() => {
    setRustFunctions();
  }, []);

  return (
    <>
      {
        (rustFuncsObject === 0) ? (null) : (<GameContainer rustFuncs={rustFuncsObject}></GameContainer>)
      }
    </>
  );
};
