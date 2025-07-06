import { wasm_generate } from "planner-core";

self.addEventListener("message", (e) => {
  if (e.data.type === "start") {
    const { plan, subjects, excluded_teachers } = e.data;

    console.log("starting!", e.data);

    const solution = wasm_generate(
      plan,
      subjects,
      new Float64Array(),
      excluded_teachers,
    );

    console.log("done!", solution);

    self.postMessage({
      type: "result",
      solution: solution,
    });
  }
});

self.addEventListener("error", (e) => {
  console.error("Worker error:", e);
});
