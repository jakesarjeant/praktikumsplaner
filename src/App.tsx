import { useState } from "react";
import { Button } from "@/components/ui/button";

function App() {
  const [count, setCount] = useState(0);

  return (
    <>
      <Button onClick={() => setCount((c) => c + 1)}>Count is {count}</Button>
    </>
  );
}

export default App;
