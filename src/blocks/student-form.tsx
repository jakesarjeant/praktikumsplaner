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
import { useEffect, useRef, useState, type RefObject } from "react";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";

export default function StudentForm() {
  const popoverRef = useRef<Element>(null);
  const [width, setWidth] = useState(0);

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
                  onFocus={() => setTimeout(() => setSubjectsOpen(true), 0)}
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
                    <CommandItem>Chemie</CommandItem>
                    <CommandItem>Physik</CommandItem>
                  </CommandGroup>
                </CommandList>
              </PopoverContent>
            </Popover>
          </Command>
        </Label>
      </CardContent>
      <CardFooter className="flex justify-end">
        <Button>Plan Erstellen</Button>
      </CardFooter>
    </Card>
  );
}
