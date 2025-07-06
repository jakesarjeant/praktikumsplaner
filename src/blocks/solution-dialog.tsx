import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";

import { type Solution } from "./solver-form";

import { useMemo, useCallback } from "react";
import { type WilliStundenplan } from "willi";

export default function SolutionDialog({
  solution,
  open,
  setOpen,
  plan,
}: {
  solution: Solution | null;
  open: boolean;
  setOpen: (open: boolean) => void;
  plan: WilliStundenplan | null;
}) {
  const transpose = useMemo(() => {
    if (!solution || !plan) return [];

    const pl_lines = plan.stunden_lehrerplan();

    const longest = solution.assignments
      .map((day) => day.length)
      .reduce((acc, cur) => Math.max(acc, cur), 0);

    return [...Array(longest).keys()].map((period) =>
      solution.assignments.map((day) => {
        const idx = day[period];
        if (!idx) return null;
        return pl_lines[idx];
      }),
    );
  }, [solution, plan]);

  const downloadSheet = useCallback(() => {}, [transpose]);

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent className="sm:max-w-6xl sm:w-[80vw] max-h-[90vh]">
        <DialogHeader>
          <DialogTitle>Stundenplan Herunterladen</DialogTitle>
          <DialogDescription>
            Hier sehen sie eine Vorschau Ihres Stundenplans. Sie können diesen
            nun als CSV-Datei herunterladen, um ihn weiter zu bearbeiten oder zu
            drucken.
          </DialogDescription>
        </DialogHeader>
        <div className="border rounded-lg max-w-full overflow-auto">
          <table className="w-full border-0 border-collapse">
            <thead>
              <tr>
                <th className="border-b" />
                {[
                  "Montag",
                  "Dienstag",
                  "Mittwoch",
                  "Donnerstag",
                  "Freitag",
                  "Samstag",
                  "Sonntag",
                ]
                  .filter((_, i) => !!solution?.assignments[i].length)
                  .map((day, i) => (
                    <th
                      key={i}
                      className="p-2 font-normal border first:border-l-0 last:border-r-0 border-t-0"
                      colSpan={2}
                    >
                      {day}
                    </th>
                  ))}
              </tr>
            </thead>
            <tbody>
              {transpose.map((days, i) => (
                <tr key={i} className="last:[&>td]:border-b-0">
                  <td className="border-b p-2 text-center">{i + 1}</td>
                  {days
                    .filter((_, i) => !!solution?.assignments[i].length)
                    .map((cell, i) => (
                      <>
                        <td key={i * 2} className="border p-2 border-r-0">
                          <div className="flex flex-col">
                            <span>{cell?.raum}</span>
                            <span>{cell?.fach}</span>
                          </div>
                        </td>
                        <td
                          key={i * 2 + 1}
                          className="border p-2 border-l-0 last:border-r-0"
                        >
                          <div className="flex flex-col">
                            <span>{cell?.klasse}</span>
                            <span>{cell?.lehrkraft}</span>
                          </div>
                        </td>
                      </>
                    ))}
                </tr>
              ))}
            </tbody>
          </table>
        </div>
        <DialogFooter>
          <Button
            onClick={() => {
              downloadCsv();
              setOpen(false);
            }}
          >
            Als .csv herunterladen
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
