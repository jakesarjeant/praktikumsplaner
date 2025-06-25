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
import { useState, useMemo, useEffect } from "react";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import { Muted } from "@/components/typography";
import Sortable from "@/components/sortable";
import {
  ComboInput,
  ComboEmpty,
  ComboGroup,
  ComboItem,
} from "@/components/combo-input";

import { WilliStundenplan, FachZeile } from "willi";
import { CSS } from "@dnd-kit/utilities";
import { GripVertical, Trash } from "lucide-react";

// TODO: Accept the whole WilliStundenplan|null instead
export default function StudentForm({
  plan,
}: {
  plan: WilliStundenplan | null;
}) {
  // TODO: Move all this stuff into a ComboInput component
  // ---

  const [selectedSubjects, setSelectedSubjects] = useState<
    (FachZeile & { id: string })[]
  >([]);
  // Getter is quite expensive because we need to copy the entire table into a hashmap
  const faecher = useMemo(() => plan?.faecher, [plan]);
  const availableSubjects = useMemo(
    () =>
      faecher &&
      Object.entries(faecher)
        .filter(([id]) => !selectedSubjects.find((s) => s.id == id))
        .map(([id, f]) => {
          Object.defineProperty(f, "id", { value: id });
          return f;
        }),
    [faecher, selectedSubjects],
  ) as (FachZeile & { id: string })[];

  const [studentName, setStudentName] = useState("");

  useEffect(() => {
    setSelectedSubjects([]);
  }, [plan]);

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
          <Input
            type="text"
            id="student-name"
            placeholder="Max Mustermann"
            value={studentName}
            onChange={(e) => setStudentName(e.target.value)}
          />
        </Label>
        <Separator className="my-4" />
        <div
          data-disabled={!plan}
          className="data-[disabled=true]:opacity-50 data-[disabled=true]:pointer-events-none \
                     data-[disabled=true]:cursor-not-allowed data-[disabled=true]:select-none"
        >
          <p className="mb-4">
            Mit der folgenden Suchfunktion können Sie aus den im hochgeladenen
            Stundenplan vorhandenen Fächern auswählen, und sie daraufhin durch
            Ziehen nach priorität sortieren.
          </p>
          <Label className="gap-8 align-center">
            <span className="shrink-0">Fächer Wählen</span>
            {/* TODO: ComboInput */}
            <ComboInput
              placeholder="Fach Suchen (Enter zum Auswählen)"
              disabled={!plan}
            >
              <ComboEmpty>Keine Ergebnisse.</ComboEmpty>
              <ComboGroup>
                {availableSubjects &&
                  availableSubjects.map((s) => (
                    <ComboItem
                      key={s.id}
                      keywords={[s.name || "", s.kuerzel]}
                      value={s.id}
                      onSelect={(val) => {
                        setSelectedSubjects((s) => [
                          ...s,
                          availableSubjects.find((f) => f.id == val) as {
                            id: string;
                          } & FachZeile,
                        ]);
                      }}
                    >
                      <strong>{s.kuerzel}</strong>
                      {s.name}
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
                  <strong>{item.kuerzel}</strong>
                  <span>{item.name}</span>
                </div>
                {/* TODO: Implement deletion */}
                <Button
                  variant="ghost"
                  size="icon"
                  className="text-muted-foreground hover:text-destructive size-7 hover:bg-transparent"
                  onClick={() =>
                    setSelectedSubjects((subs) =>
                      subs.filter((_, i) => i != idx),
                    )
                  }
                >
                  <Trash />
                </Button>
              </div>
            )}
          />
        </div>
      </CardContent>
      <CardFooter className="flex justify-end gap-4">
        <Muted>
          {!selectedSubjects.length && "Mindestens ein Fach auswählen"}
          {!selectedSubjects.length && !studentName && " und "}
          {!studentName && "Namen eingeben"}
          {(!studentName || !selectedSubjects.length) && "."}
        </Muted>
        <Button disabled={!selectedSubjects.length || !studentName}>
          Plan Erstellen
        </Button>
      </CardFooter>
    </Card>
  );
}
