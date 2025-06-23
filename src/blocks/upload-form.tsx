import { useState, type ChangeEvent } from "react";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardFooter,
  CardContent,
} from "@/components/ui/card";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { Separator } from "@/components/ui/separator";

export default function UploadForm() {
  const [file, setFile] = useState<File | null>(null);

  const handleFile = (e: ChangeEvent<HTMLInputElement>) => {
    if (e.target.files) setFile(e.target.files[0]);
  };

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
            Bitte Laden Sie einen WILLI-Stundenplan (Dateieendung .BAL) hoch,
            der für die Planung verwendet werden soll.
          </p>
          <Separator className="my-4" />
          <Label className="gap-8">
            <span className="shrink-0">WILLI2-Datei</span>
            <Input
              type="file"
              className="flex-[1 0 0] flex"
              onChange={handleFile}
            />
          </Label>
        </CardContent>
        <CardFooter className="flex justify-end">
          <Button disabled={file == null}>Weiter</Button>
        </CardFooter>
      </Card>
    </>
  );
}
