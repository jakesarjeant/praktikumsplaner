import StudentForm from "./blocks/student-form";
import UploadForm from "./blocks/upload-form";

import { useState } from "react";
import { WilliStundenplan } from "willi";

function App() {
  const [plan, setPlan] = useState<WilliStundenplan | null>(null);

  return (
    <div className="flex flex-col gap-8 w-full max-w-2xl">
      <UploadForm setPlan={setPlan} />
      <StudentForm faecher={["Chemie", "Physik", "Mathe", "Musik"]} />
    </div>
  );
}

export default App;
