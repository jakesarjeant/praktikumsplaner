import * as React from "react";
import { Clock } from "lucide-react";

import { AccordionItem, AccordionTrigger, AccordionContent } from "@/components/ui/accordion";
import { Switch } from "@/components/ui/switch";
import { Badge } from "@/components/ui/badge";
import { Slider } from "@/components/ui/slider";
import { humanTime } from "@/lib/utils";

export function AnytimeItem({
  accordionState,
  setAccordionState,
  value,
  onValueChange,
  enabled,
  setEnabled,
  progress,
  startTime,
  working,
}: {
  accordionState: string[],
  setAccordionState: (state: string[]) => void;
  value: number,
  onValueChange: (v: number) => void;
  enabled: boolean,
  setEnabled: (e: boolean) => void;
  progress: number,
  startTime: number | null,
  working: boolean,
}) {
  /* const open = React.useMemo(
   *   () => accordionState.includes("anytime"),
   *   [accordionState],
   * ); */

  const setOpen = React.useCallback((open: boolean) => {
    if (open) {
      setEnabled(true);
      setAccordionState([...accordionState, "anytime"]);
    } else
      setAccordionState(accordionState.filter(s => s != "anytime"));
  }, [setEnabled, setAccordionState, accordionState]);

  return (
    <AccordionItem
      value="anytime"
      className="px-6 border rounded-lg last:border-b-1 relative overflow-hidden"
    >
      <AccordionTrigger
        className="text-sm text-foreground max-w-full items-center hover:no-underline \
                                   group"
      >
        <div className="flex gap-4 shrink-0 items-center">
          <Switch
            checked={enabled}
            onClick={(e) => {
              const s = !enabled;
              setOpen(s);
              setEnabled(s);
              e.stopPropagation();
            }}
          />
          <span className="group-hover:underline py-0.5">
            Zeitbegrenzung
          </span>
        </div>
        <div
          className="flex grow-1 gap-3 items-center overflow-hidden mask-r-from-30% opacity-0 \
                       data-shown:opacity-100 transition-opacity duration-100"
          data-shown={true}
        >
          <Badge variant={enabled ? ((working && !!startTime) ? "destructive" : "default") : "secondary"}>
            {
                (working && !!startTime && enabled) ? <>
                  <Clock />
                  {humanTime(Math.ceil((Date.now() - startTime - value) / -1000))}
                </> : humanTime(Math.floor(value / 1000))}
          </Badge>
        </div>
      </AccordionTrigger>
      <AccordionContent>
        <p>
          Wenn die Rechenzeit die angegebene Begrenzung überschreitet, wird die Berechnung
          abgebrochen und die bisher beste Lösung sofort angezeigt.
        </p>
        <Slider
          min={10000}
          max={300000}
          step={1000}
          value={[value]}
          onValueChange={(v) => onValueChange(v[0])}
          disabled={working}
          className="mt-8 mb-4"
        />
      </AccordionContent>
      <div
        className="absolute bottom-0 left-0 h-2 bg-foreground opacity-100 data-[hidden=true]:opacity-0 duration-300"
        data-hidden={progress == 1 || !enabled}
        style={{ width: `${Math.ceil(10000 * progress) / 100}%` }}
      />
    </AccordionItem>
  );
}
