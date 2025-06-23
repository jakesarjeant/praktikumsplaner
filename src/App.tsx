import StudentForm from "./blocks/student-form";
import UploadForm from "./blocks/upload-form";

function App() {
  return (
    <div className="flex flex-col gap-8 w-full max-w-2xl">
      <UploadForm />
      <StudentForm />
    </div>
  );
}

export default App;
