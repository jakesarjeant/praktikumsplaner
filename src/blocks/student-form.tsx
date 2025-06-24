import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { useState, useMemo } from "react";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import Sortable from "@/components/sortable";
import {
  ComboInput,
  ComboEmpty,
  ComboGroup,
  ComboItem,
} from "@/components/combo-input";

import { WilliStundenplan } from "willi";
import { CSS } from "@dnd-kit/utilities";
import { GripVertical, Trash } from "lucide-react";

// TODO: Accept the whole WilliStundenplan|null instead
export default function StudentForm({ faecher }: { faecher: string[] }) {
  // TODO: Move all this stuff into a ComboInput component
  // ---

  const [selectedSubjects, setSelectedSubjects] = useState<{ id: string }[]>(
    [],
  );
  const availableSubjects = useMemo(
    () =>
      faecher
        .filter((f) => !selectedSubjects.find((s) => s.id == f))
        .map((id) => ({ id })),
    [faecher, selectedSubjects],
  );

  return (
    <Card className="w-full">
      <CardHeader>
        <CardTitle>Parameter Festlegen</CardTitle>
        <CardDescription>
          Geben Sie die Daten des Praktikanten ein, um einen Plan zu generieren.
        </CardDescription>
      </CardHeader>
      <CardContent>
        <Label className="gap-8 align-center">
          <span className="shrink-0">Vollständiger Name</span>
          <Input type="text" id="student-name" placeholder="Max Mustermann" />
        </Label>
        <Separator className="my-4" />
        <div>
          <p className="mb-4">
            Mit der folgenden Suchfunktion können Sie aus den im hochgeladenen
            Stundenplan vorhandenen Fächern auswählen, und sie daraufhin durch
            Ziehen nach priorität sortieren.
          </p>
          <Label className="gap-8 align-center">
            <span className="shrink-0">Fächer Wählen</span>
            {/* TODO: ComboInput */}
            <ComboInput placeholder="Fach Suchen (Enter zum Auswählen)">
              <ComboEmpty>Keine Ergebnisse.</ComboEmpty>
              <ComboGroup>
                {availableSubjects.map((s) => (
                  <ComboItem
                    key={s.id}
                    value={s.id}
                    onSelect={(val) => {
                      setSelectedSubjects((s) => [...s, { id: val }]);
                    }}
                  >
                    {s.id}
                  </ComboItem>
                ))}
              </ComboGroup>
            </ComboInput>
          </Label>
          <Sortable
            items={selectedSubjects}
            updateSort={setSelectedSubjects}
            render={({
              transform,
              transition,
              setNodeRef,
              isDragging,
              attributes,
              listeners,
              item,
              idx,
            }) => (
              <div
                data-dragging={isDragging}
                ref={setNodeRef}
                className="w-full p-2 flex items-center justify-between relative z-0\
                         data-[dragging=true]:z-10 data-[dragging=true]:opacity-80\
                         flex-nowrap"
                style={{
                  transform: CSS.Transform.toString(transform),
                  transition,
                }}
              >
                <div className="flex gap-2 flex-row items-center justify-start flex-initial">
                  <Button
                    {...attributes}
                    {...listeners}
                    variant="ghost"
                    size="icon"
                    className="text-muted-foreground size-7 hover:bg-transparent"
                  >
                    <GripVertical />
                  </Button>
                  <span className="bold font-mono">{1 + idx}.</span>
                  <span>{item.id}</span>
                </div>
                {/* TODO: Implement deletion */}
                <Button
                  variant="ghost"
                  size="icon"
                  className="text-muted-foreground hover:text-destructive size-7 hover:bg-transparent"
                >
                  <Trash />
                </Button>
              </div>
            )}
          />
        </div>
      </CardContent>
      <CardFooter className="flex justify-end">
        <Button>Plan Erstellen</Button>
      </CardFooter>
    </Card>
  );
}
