import { useEffect, useRef, useState, useCallback } from "react";
import MyWorker from "../worker?worker";

export function useSolverWorker(
  onSolution: (solution: (number | null)[][]) => void,
  onFinal: (solution: (number | null)[][]) => void,
): {
  worker: Worker | null;
  start: (
    plan: string,
    subjects: string[],
    excluded_teachers: string[],
  ) => void;
  working: boolean;
  abort: () => void;
} {
  const workerRef = useRef<Worker | null>(null);
  const [working, setWorking] = useState(false);

  const startWorker = useCallback(() => {
    const worker = new MyWorker();

    worker.addEventListener("message", (e: MessageEvent) => {
      // NOTE: progress updates are handled directly by the progress component to
      // avoid re-running components higher up in the hierarchy
      if (e.data.type === "solution") {
        onSolution(e.data.solution);
      }

      if (e.data.type === "final") {
        setWorking(false);
        onFinal(e.data.solution);
      }
    });

    workerRef.current = worker;

    return () => {
      workerRef.current?.terminate();
    };
  }, [setWorking, onSolution, onFinal]);

  useEffect(() => {
    startWorker();
  }, [startWorker]);

  // TODO: handle error — at the very least, the spinner should stop

  const start = useCallback(
    (plan: string, subjects: string[], excluded_teachers: string[]) => {
      setWorking(true);
      workerRef.current?.postMessage({
        type: "start",
        subjects,
        plan,
        excluded_teachers,
      });
    },
    [workerRef],
  );

  // TODO: abort
  const abort = useCallback(
    () => {
      workerRef.current?.terminate();
      setWorking(false);
      // TODO: Does this really make that much sense?
      startWorker();
    },
    [workerRef, startWorker]
  );

  return {
    worker: workerRef.current,
    start,
    working,
    abort
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
    [setProgress, setLastMsg, lastMsg],
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
