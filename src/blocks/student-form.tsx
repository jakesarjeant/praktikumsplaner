import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@/components/ui/command";
import { Label } from "@/components/ui/label";
import {
  Popover,
  PopoverAnchor,
  PopoverContent,
} from "@/components/ui/popover";
import { useEffect, useRef, useState, useMemo, type RefObject } from "react";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import Sortable from "@/components/sortable";

import { WilliStundenplan } from "willi";
import { CSS } from "@dnd-kit/utilities";
import { GripVertical, Trash } from "lucide-react";

// TODO: Accept the whole WilliStundenplan|null instead
export default function StudentForm({ faecher }: { faecher: string[] }) {
  // TODO: Move all this stuff into a ComboInput component
  const popoverRef = useRef<Element>(null);
  const [width, setWidth] = useState(0);

  const inputRef = useRef<HTMLInputElement>(null);
  const [subjInput, setSubjInput] = useState("");

  useEffect(() => {
    const onResize = () => {
      setWidth(popoverRef.current?.getBoundingClientRect().width || 0);
    };

    onResize();

    window.addEventListener("resize", onResize);

    return () => {
      window.removeEventListener("resize", onResize);
    };
  }, [popoverRef]);

  const [subjectsOpen, setSubjectsOpen] = useState(false);
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
        <p className="mb-4">
          Mit der folgenden Suchfunktion können Sie aus den im hochgeladenen
          Stundenplan vorhandenen Fächern auswählen, und sie daraufhin durch
          Ziehen nach priorität sortieren.
        </p>
        <Label className="gap-8 align-center">
          <span className="shrink-0">Fächer Wählen</span>
          <Command
            className="border-1 bg-background"
            ref={popoverRef as RefObject<HTMLDivElement>}
          >
            {/* TODO: Maybe turn this into its own "combo-input" component */}
            <Popover open={subjectsOpen} onOpenChange={setSubjectsOpen}>
              {/* <PopoverTrigger> */}
              <PopoverAnchor>
                <CommandInput
                  placeholder="Fach Suchen (Enter zum Auswählen)"
                  wrapperClassName="border-0"
                  value={subjInput}
                  onValueChange={(v) => {
                    setSubjInput(v);
                    if (!subjectsOpen) {
                      setSubjectsOpen(true);
                    }
                  }}
                  ref={inputRef}
                  onFocus={() => setTimeout(() => setSubjectsOpen(true), 0)}
                  onClick={() => setTimeout(() => setSubjectsOpen(true), 0)}
                  onBlur={() => setSubjectsOpen(false)}
                />
              </PopoverAnchor>
              {/* </PopoverTrigger> */}
              <PopoverContent
                className="bg-background mt-1 p-1"
                style={{ width: `${width}px` }}
                onOpenAutoFocus={(e) => e.preventDefault()}
              >
                <CommandList>
                  <CommandEmpty>Keine Ergebnisse.</CommandEmpty>
                  <CommandGroup>
                    {/* <CommandItem>Chemie</CommandItem>
                        <CommandItem>Physik</CommandItem> */}
                    {availableSubjects.map((s) => (
                      <CommandItem
                        key={s.id}
                        value={s.id}
                        onSelect={(val) => {
                          setSelectedSubjects((s) => [...s, { id: val }]);
                          setSubjectsOpen(false);
                          setSubjInput("");
                          if (inputRef.current != null) {
                            {
                              /* inputRef.current.blur(); */
                            }
                          }
                        }}
                      >
                        {s.id}
                      </CommandItem>
                    ))}
                  </CommandGroup>
                </CommandList>
              </PopoverContent>
            </Popover>
          </Command>
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
      </CardContent>
      <CardFooter className="flex justify-end">
        <Button>Plan Erstellen</Button>
      </CardFooter>
    </Card>
  );
}
