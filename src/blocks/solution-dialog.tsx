import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Alert, AlertTitle } from "@/components/ui/alert";
import { download } from "@/lib/utils";

import { type Solution } from "./solver-form";

import { useMemo, useCallback, useRef } from "react";
import { type WilliStundenplan } from "willi";
import * as XLSX from "xlsx";
import { Printer, BadgeInfo } from "lucide-react";

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

  const tableRef = useRef<HTMLTableElement | null>(null);

  const downloadSheet = useCallback(() => {
    if (!tableRef.current) return;

    const sheet = XLSX.utils.table_to_book(tableRef.current, {
      raw: true,
    });

    const xlsx_data = XLSX.writeXLSX(sheet, {
      type: "array",
    });

    const file = new File(
      [xlsx_data],
      `stundenplan-${solution!.name.toLowerCase().replace(" ", "-")}.xlsx`,
    );

    download(file);
  }, [solution]);

  const print = useCallback(() => {
    if (!tableRef.current) return;

    const html = tableRef.current.getHTML();

    const printWindow = window.open("", "_blank", "width=800,height=600")!;
    printWindow.document.write(`
      <html>
      <head>
        <title>Stundenplan ${solution?.name || ""}</title>
        <style>
        @page { size: portrait; }
        body { font-family: sans-serif; padding: 20px; }
        table, th, td { border: 1px solid #000000; border-collapse: collapse; }
        table { width: 100%; }
        th, td { padding: 8px; }
        .border-r-0 { border-right: 0px; }
        .border-l-0 { border-left: 0px; }
        </style>
      </head>
      <body>
        <strong>${solution?.name}</strong>
        <table>
          ${html}
        </table>
        <script>
        window.onload = function() {
          window.focus();
          window.print();
          window.close();
        };
        </script>
      </body>
      </html>
   `);
    printWindow.document.close();
  }, [tableRef, solution]);

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent className="sm:max-w-6xl sm:w-[80vw] max-h-[90vh]">
        <DialogHeader>
          <DialogTitle>Stundenplan erstellt</DialogTitle>
          <DialogDescription>
            Hier sehen Sie eine Vorschau Ihres Stundenplans. Sie können diesen
            nun ausdrucken oder als Excel-Datei (.xlsx) herunterladen, um ihn
            weiter zu bearbeiten.
          </DialogDescription>
        </DialogHeader>
        <h4>
          Plan für: <strong>{solution?.name}</strong>
        </h4>
        <div className="border rounded-lg max-w-full overflow-auto">
          <table className="w-full border-0 border-collapse" ref={tableRef}>
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
                            <br />
                            <span>{cell?.fach}</span>
                          </div>
                        </td>
                        <td
                          key={i * 2 + 1}
                          className="border p-2 border-l-0 last:border-r-0"
                        >
                          <div className="flex flex-col">
                            <span>{cell?.klasse}</span>
                            <br />
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
        <Alert>
          <BadgeInfo />
          <AlertTitle>
            Für ein sauberes Erscheinungsbild können Sie im Druckfenster "Kopf-
            und Fußzeilen drucken" deaktivieren.
          </AlertTitle>
        </Alert>
        <DialogFooter className="flex-row-reverse sm:flex-row-reverse justify-start sm:justify-start">
          <Button onClick={print}>
            <Printer />
            Drucken
          </Button>
          <Button
            variant="outline"
            onClick={() => {
              downloadSheet();
            }}
          >
            Als .xlsx herunterladen
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
