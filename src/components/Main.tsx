import React, { useEffect } from "react";
import rustFuncs from "../../target/wasm32-unknown-unknown/debug/scroller.js";
import { GameContainer } from "./index";

export const Main = () => {
  const [rustFuncsObject, setRustFuncsObject] = React.useState(0);

  const setRustFunctions = async () => {
    const funcs = await rustFuncs;
    setRustFuncsObject(() => funcs);
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
