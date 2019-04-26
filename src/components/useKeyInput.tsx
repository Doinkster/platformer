import { useEffect, useState } from "react";

const useKeyInput = () => {
  const [keyPressedDown, setKeyPressedDown] = useState(false);

  function downHandler({ key }) {
    setKeyPressedDown(true);
  }

  const upHandler = ({ key }) => {
    setKeyPressedDown(false);
  };



  useEffect(() => {
    window.addEventListener("keydown", downHandler);
    window.addEventListener("keyup", upHandler);
    return () => {
      window.removeEventListener("keydown", downHandler);
      window.removeEventListener("keyup", upHandler);
    };
  });
};

export default useKeyInput;

//https://usehooks.com/useKeyPress/
//https://codesandbox.io/s/5v71vl72kk
