import { useEffect, useRef } from "react";

const useOneFrame = dispatchPositions => {
  //frameCountRef used to track frame ID to use in cancelAnimationFrame
  const frameCountRef = useRef(0);

  const frame = () => {
    frameCountRef.current = requestAnimationFrame(frame);
    dispatchPositions();
  };

  useEffect(() => {
    frameCountRef.current = requestAnimationFrame(frame);
    return () => {
      cancelAnimationFrame(frameCountRef.current);
    };
  }, []);
};

export default useOneFrame;
