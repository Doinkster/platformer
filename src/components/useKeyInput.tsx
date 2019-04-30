import { useEffect } from "react";

const useKeyInput = (keys, setKeys) => {
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
};

export default useKeyInput;

//https://usehooks.com/useKeyPress/
//https://codesandbox.io/s/5v71vl72kk
//https://www.reddit.com/r/reactjs/comments/9zupzn/why_would_i_use_react_hooks_where_the_seteffect/
