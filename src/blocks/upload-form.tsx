import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from "@/components/ui/card";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { Separator } from "@/components/ui/separator";

import { useState, useEffect, type ChangeEvent } from "react";
import { WilliStundenplan, WilliParseError, parse_plan } from "willi";

export default function UploadForm({
  setPlan,
}: {
  setPlan: React.Dispatch<React.SetStateAction<WilliStundenplan | null>>;
}) {
  const [file, setFile] = useState<File | null>(null);

  const [errors, setErrors] = useState<WilliParseError[]>([]);

  const handleFile = (e: ChangeEvent<HTMLInputElement>) => {
    if (e.target.files) setFile(e.target.files[0]);
  };

  // Parse/Validate file
  useEffect(() => {
    if (!file) return setPlan(null);

    const reader = new FileReader();
    reader.onload = () => {
      let { plan, errors } = parse_plan(reader.result as string);
      setErrors(errors);
      setPlan(plan);

      if (errors.length) console.error(errors);
    };
    reader.onerror = () => {
      // TODO: Error
    };

    reader.readAsText(file, "windows-1252");
  }, [file]);

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
            Bitte Laden Sie einen WILLI-Stundenplan (Dateiendung .BAL) hoch,
            der für die Planung verwendet werden soll.
          </p>
          <Separator className="my-4" />
          <Label className="gap-8">
            <span className="shrink-0">WILLI2-Datei</span>
            <Input
              type="file"
              aria-invalid={!!errors.length}
              className="flex-[1 0 0] flex"
              onChange={handleFile}
            />
          </Label>
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
    </>
  );
}
