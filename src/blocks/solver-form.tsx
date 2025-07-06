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
import { Badge } from "@/components/ui/badge";
import { Separator } from "@/components/ui/separator";
import { Muted } from "@/components/typography";
import Sortable from "@/components/sortable";
import {
  ComboInput,
  ComboEmpty,
  ComboGroup,
  ComboItem,
} from "@/components/combo-input";
import {
  Accordion,
  AccordionItem,
  AccordionTrigger,
  AccordionContent,
} from "@/components/ui/accordion";
import { Switch } from "@/components/ui/switch";

import { useSolverWorker, useProgress } from "../hooks/useSolverWorker";

import { WilliStundenplan, FachZeile, LehrkraftZeile } from "willi";
import { CSS } from "@dnd-kit/utilities";
import { GripVertical, X, Loader } from "lucide-react";
import { useCallback } from "react";

export default function SolverForm({
  plan,
  planString,
  setSolution,
}: {
  plan: WilliStundenplan | null;
  planString: string | null;
  setSolution: (solution: Solution | null) => void;
}) {
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

  const [accordionState, setAccordionState] = useState<string[]>([]);
  const [excludedTeachers, setExcludedTeachers] = useState<
    (LehrkraftZeile & { id: string })[]
  >([]);

  useEffect(() => {
    setSelectedSubjects([]);
  }, [plan]);

  const onSolution = useCallback(
    (assignments: (number | null)[][]) => {
      const solution = {
        name: studentName,
        assignments,
      };

      setSolution(solution);
      console.log("got solution:", solution);
    },
    [setSolution, studentName],
  );

  const { worker, start, working } = useSolverWorker(onSolution);

  return (
    <Card className="w-full pb-0 overflow-hidden">
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
            Ziehen nach Priorität sortieren.
          </p>
          <Label className="gap-8 align-center">
            <span className="shrink-0">Fächer wählen</span>
            {/* TODO: ComboInput */}
            <ComboInput
              placeholder="Fach suchen (Enter zum Auswählen)"
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
                  <strong className="font-[monospace] text-sm">
                    {1 + idx}.
                  </strong>
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
                  <X />
                </Button>
              </div>
            )}
          />
          <Accordion
            type="multiple"
            className="w-full mt-4"
            value={accordionState}
            onValueChange={setAccordionState}
          >
            <TeacherExcludeItem
              accordionState={accordionState}
              value={excludedTeachers}
              onValueChange={setExcludedTeachers}
              plan={plan}
            />
            {/*TODO: Anytime*/}
          </Accordion>
        </div>
      </CardContent>
      <SolveActions
        selectedSubjects={selectedSubjects}
        studentName={studentName}
        plan={planString}
        solveCallback={start}
        worker={worker}
        working={working}
        excludedTeachers={excludedTeachers.map((t) => t.kuerzel)}
      />
    </Card>
  );
}

function TeacherExcludeItem({
  accordionState,
  value,
  onValueChange,
  plan,
}: {
  accordionState: string[];
  value: (LehrkraftZeile & { id: string })[];
  onValueChange: (value: (LehrkraftZeile & { id: string })[]) => void;
  plan: WilliStundenplan | null;
}) {
  const open = useMemo(
    () => accordionState.includes("exclude-teachers"),
    [accordionState],
  );

  const lehrer = useMemo(() => plan?.lehrkraefte, [plan]);
  const availableTeachers = useMemo(
    () =>
      lehrer &&
      Object.entries(lehrer)
        .filter(([id]) => !value.find((l) => l.id == id))
        .map(([id, f]) => {
          Object.defineProperty(f, "id", { value: id });
          return f;
        }),
    [lehrer, value],
  ) as (LehrkraftZeile & { id: string })[];

  const [showClear, setShowClear] = useState(false);

  return (
    <>
      <AccordionItem
        value="exclude-teachers"
        className="px-6 border rounded-lg last:border-b-1"
      >
        <AccordionTrigger
          className="text-sm text-foreground max-w-full items-center hover:no-underline \
                                   group"
        >
          <div className="flex gap-4 shrink-0 items-center">
            <Switch
              checked={open || !!value.length}
              onClick={() => {
                if (value.length) setShowClear(true);
              }}
            />
            <span className="group-hover:underline py-0.5">
              Lehrer ausschließen
            </span>
          </div>
          <div
            className="flex grow-1 gap-3 items-center overflow-hidden mask-r-from-30% opacity-0 \
                       data-shown:opacity-100 transition-opacity duration-100"
            data-shown={!open || null}
          >
            {value.map((lehrer, i) => (
              <Badge variant="secondary" key={i}>
                {lehrer.kuerzel}
              </Badge>
            ))}
          </div>
        </AccordionTrigger>
        <AccordionContent>
          <p className="mb-4 text-muted-foreground">
            Geben Sie hier Lehrer an, denen keine Praktikanten zugewiesen werden
            dürfen.
          </p>
          <Label className="gap-8 align-center">
            <span className="shrink-0">Lehrer wählen</span>
            {/* TODO: ComboInput */}
            <ComboInput placeholder="Lehrer suchen" disabled={!plan}>
              <ComboEmpty>Keine Ergebnisse.</ComboEmpty>
              {availableTeachers &&
                availableTeachers.map((s) => (
                  <ComboItem
                    key={s.id}
                    keywords={[s.name || "", s.kuerzel]}
                    value={s.id}
                    onSelect={(val) => {
                      onValueChange([
                        ...value,
                        availableTeachers.find((f) => f.id == val) as {
                          id: string;
                        } & LehrkraftZeile,
                      ]);
                    }}
                  >
                    <strong>{s.kuerzel}</strong>
                    {s.name}
                  </ComboItem>
                ))}
            </ComboInput>
          </Label>
          <div className="flex flex-wrap gap-2 items-center">
            Ausgeschlossene Lehrkräfte:
            {value.map((lehrer, i) => (
              <Badge key={i} variant="secondary">
                {lehrer.kuerzel}
                <Button
                  variant="ghost"
                  size="icon"
                  className="text-muted-foreground hover:text-destructive size-7 hover:bg-transparent"
                  onClick={() =>
                    onValueChange(
                      value.filter((l) => l.kuerzel !== lehrer.kuerzel),
                    )
                  }
                >
                  <X />
                </Button>
              </Badge>
            ))}
          </div>
        </AccordionContent>
      </AccordionItem>
      <AlertDialog open={showClear}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Wirklich ausschalten?</AlertDialogTitle>
            <AlertDialogDescription>
              Ihre auswahl an ausgeschlossenen Lehrkräften wird zurückgesetzt.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel
              onClick={() => {
                setShowClear(false);
              }}
            >
              Abbrechen
            </AlertDialogCancel>
            <AlertDialogAction
              onClick={() => {
                setTimeout(() => setShowClear(false), 0);
                onValueChange([]);
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

/* function TimeoutItem({
 *   accordionState,
 *   value,
 *   onValueChange,
 * }: {
 *   accordionState: string[];
 *   value: number;
 *   onValueChange: (value: number) => void;
 * }) {
 *   const [timeout, setTimeout] = useState(0);
 *   const [enabled, disabled] = useState(false);
 * } */

export interface Solution {
  name: string;
  assignments: (number | null)[][];
}

// HACK: Yay prop-drilling!
function SolveActions({
  selectedSubjects,
  /* studentName, */
  plan,
  solveCallback,
  worker,
  working,
  excludedTeachers,
}: {
  selectedSubjects: (FachZeile & { id: string })[];
  studentName: string | null;
  plan: string | null;
  solveCallback: (plan: string, subjects: string[], e_t: string[]) => void;
  worker: Worker | null;
  working: boolean;
  excludedTeachers: string[];
}) {
  const progress = useProgress(worker);
  console.log("excluding", excludedTeachers);

  return (
    <CardFooter className="flex justify-end gap-4 relative pb-6">
      <Muted>
        {!selectedSubjects.length && "Mindestens ein Fach auswählen"}
        {/* {!selectedSubjects.length && !studentName && " und "} */}
        {/* {!studentName && "Namen eingeben"} */}
        {/*!studentName ||*/ !selectedSubjects.length && ". "}
        {working &&
          `Wird berechnet... (${progress?.best}@${progress?.visited})`}
      </Muted>
      <Button
        disabled={
          !selectedSubjects.length /*|| !studentName*/ || !plan || working
        }
        onClick={() => {
          solveCallback(
            plan!,
            selectedSubjects.map((s) => s.kuerzel),
            excludedTeachers,
          );
        }}
      >
        {working && (
          <Loader className="animate-spin [animation-duration:1.5s]" />
        )}
        Plan erstellen
      </Button>
    </CardFooter>
  );
}
