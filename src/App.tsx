import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Card, CardHeader, CardTitle, CardDescription } from "@/components/ui/card";

function App() {
  const [count, setCount] = useState(0);

  return (
    <>
      <Card className="w-full max-w-lg">
        <CardHeader>
          <CardTitle>Stundenplan Auswählen</CardTitle>
          <CardDescription>
            Öffnen sie eine WILLI2-Datei (.BAL), um loszulegen.
          </CardDescription>
        </CardHeader>
      </Card>
    </>
  );
}

export default App;
