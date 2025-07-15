import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from "@/components/ui/card";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from "@/components/ui/alert-dialog";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { Separator } from "@/components/ui/separator";

import { useState, useEffect, useRef, type ChangeEvent } from "react";
import { WilliStundenplan, WilliParseError, parse_plan } from "willi";

export default function UploadForm({
  setPlan,
  setRawPlan,
}: {
  setPlan: React.Dispatch<React.SetStateAction<WilliStundenplan | null>>;
  setRawPlan: React.Dispatch<React.SetStateAction<string | null>>;
}) {
  const [file, setFile] = useState<File | null>(null);
  const [errors, setErrors] = useState<WilliParseError[]>([]);
  // Initially, we consider the first upload already "confirmed"
  const [confirm, setConfirm] = useState(true);
  const [showConfirm, setShowConfirm] = useState(false);

  const handleFile = (e: ChangeEvent<HTMLInputElement>) => {
    if (e.target.files) setFile(e.target.files[0]);
    setConfirm(false);
  };

  // Parse/Validate file
  useEffect(() => {
    if (!file) return setPlan(null);

    const reader = new FileReader();
    reader.onload = () => {
      const { plan, errors } = parse_plan(reader.result as string);
      setErrors(errors);
      console.log("parsed:", plan);
      setPlan(plan);
      setRawPlan(reader.result as string);

      if (errors.length) console.error(errors);
    };
    reader.onerror = () => {
      // TODO: Error
    };

    reader.readAsText(file, "windows-1252");
  }, [file, setPlan, confirm]);

  const inputRef = useRef<HTMLInputElement>(null);

  return (
    <>
      <Card className="w-full">
        <CardHeader>
          <CardTitle>Stundenplan Auswählen</CardTitle>
          <CardDescription>
            Öffnen Sie eine WILLI2-Datei (.BAL), um loszulegen.
          </CardDescription>
        </CardHeader>
        <CardContent>
          <p>
            Bitte Laden Sie einen WILLI-Stundenplan (Dateiendung .BAL) hoch, der
            für die Planung verwendet werden soll.
          </p>
          <Separator className="my-4" />
          <button
            className="gap-8 p-0 m-0 w-full border-0 flex items-center focus-visible:outline-none cursor-pointer group"
            onClickCapture={(e) => {
              e.stopPropagation();
              if (!confirm) setShowConfirm(true);
              else inputRef.current?.click();
            }}
          >
            <Label htmlFor="file" className="shrink-0">
              WILLI2-Datei
            </Label>
            <div className="border-0 p-0 m-0 w-full">
              <Input
                type="file"
                id="file"
                ref={inputRef}
                aria-invalid={!!errors.length}
                tabIndex={-1}
                className="flex-[1 0 0] flex pointer-events-none group-focus-visible:border-ring group-focus-visible:ring-ring/50 group-focus-visible:ring-[3px]"
                onChange={handleFile}
                accept=".bal,.bak"
              />
            </div>
          </button>
          {!!errors.length && (
            <details className="mt-3">
              <summary className="text-destructive">
                Ungültige WILLI-Datei. Bitte öffnen Sie eine andere Datei.
              </summary>
              <ul className="list-disc pl-10">
                {errors.map((err, key) => (
                  <li key={key}>
                    Eintrag {err.record}: {err.error}
                  </li>
                ))}
              </ul>
            </details>
          )}
        </CardContent>
      </Card>
      <AlertDialog open={showConfirm}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Wirklich ändern?</AlertDialogTitle>
            <AlertDialogDescription>
              Wenn sie einen anderen Stundenplan öffnen, werden Ihre Auswahlen
              zurückgesetzt.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel
              onClick={() => {
                setShowConfirm(false);
              }}
            >
              Abbrechen
            </AlertDialogCancel>
            <AlertDialogAction
              onClick={() => {
                setTimeout(() => setShowConfirm(false), 0);
                inputRef.current?.click();
              }}
            >
              Weiter
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </>
  );
}
