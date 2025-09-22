import * as React from "react";

// https://www.joshwcomeau.com/snippets/react-hooks/use-interval/
export function useInterval(callback: () => void, delay: number | null) {
  const intervalRef = React.useRef<number>(null);
  const savedCallback = React.useRef(callback);

  React.useEffect(() => {
    savedCallback.current = callback;
  }, [callback]);

  React.useEffect(() => {
    const tick = () => savedCallback.current();

    if (typeof delay === 'number') {
      intervalRef.current = window.setInterval(tick, delay);

      return () => window.clearInterval(intervalRef.current!);
    }
  }, [delay]);

  return intervalRef;
}
