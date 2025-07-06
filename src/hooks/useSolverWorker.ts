import { useEffect, useRef, useState, useCallback } from "react";
import MyWorker from "../worker?worker";

export function useSolverWorker(
  onSolution: (solution: (number | null)[][]) => void,
): {
  worker: Worker | null;
  start: (plan: string, subjects: string[]) => void;
  working: boolean;
  // abort: () => void;
} {
  const workerRef = useRef<Worker | null>(null);
  const [working, setWorking] = useState(false);

  useEffect(() => {
    const worker = new MyWorker();

    worker.addEventListener("message", (e: MessageEvent) => {
      // NOTE: progress updates are handled directly by the progress component to
      // avoid re-running components higher up in the hierarchy
      if (e.data.type === "result") {
        setWorking(false);
        onSolution(e.data.solution);
      }
    });

    workerRef.current = worker;

    return () => {
      worker.terminate();
    };
  }, [onSolution]);

  // TODO: handle error — at the very least, the spinner should stop

  const start = useCallback(
    (plan: string, subjects: string[]) => {
      setWorking(true);
      workerRef.current?.postMessage({ type: "start", subjects, plan });
    },
    [workerRef],
  );

  // TODO: abort

  return {
    worker: workerRef.current,
    start,
    working,
  };
}

interface Progress {
  best: number;
  current_cost: number;
  current_classes: number;
  visited: number;
}

/**
 * Receives progress reports from the worker.
 *
 * @param {Worker} worker The web worker
 * @returns {Progress} Statistics about solver progress */
export function useProgress(worker: Worker | null): Progress | null {
  const [progress, setProgress] = useState<Progress | null>(null);
  const [lastMsg, setLastMsg] = useState<DOMHighResTimeStamp>(
    performance.now(),
  );

  const handle = useCallback(
    (e: MessageEvent) => {
      if (performance.now() - lastMsg < 50) return;
      setLastMsg(performance.now());
      if (e.data.type === "progress") {
        setProgress(e.data.progress);
      } else if (e.data.type === "result") {
        setProgress(null);
      }
    },
    [setProgress],
  );

  useEffect(() => {
    if (!worker) return;

    worker.addEventListener("message", handle);

    return () => {
      worker.removeEventListener("message", handle);
    };
  }, [worker, handle]);

  return progress;
}
