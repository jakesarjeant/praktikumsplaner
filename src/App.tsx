import { H1, Muted } from "@/components/typography";
import { Separator } from "@/components/ui/separator";
import {
  HoverCard,
  HoverCardContent,
  HoverCardTrigger,
} from "@/components/ui/hover-card";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";

import SolverForm, { type Solution } from "./blocks/solver-form";
import UploadForm from "./blocks/upload-form";
import SolutionDialog from "./blocks/solution-dialog";

import { useState, useCallback } from "react";
import { WilliStundenplan } from "willi";

function App() {
  const [rawPlan, setRawPlan] = useState<string | null>(null);
  const [plan, setPlan] = useState<WilliStundenplan | null>(null);

  const [solution, setSolution] = useState<Solution | null>(null);
  const [open, setOpen] = useState(false);

  return (
    <div className="flex flex-col gap-8 w-full max-w-2xl">
      <H1 className="mt-12">Praktikumsplaner</H1>
      <UploadForm setPlan={setPlan} setRawPlan={setRawPlan} />
      <SolverForm
        plan={plan}
        planString={rawPlan}
        setSolution={setSolution}
        setOpen={setOpen}
      />
      <SolutionDialog
        solution={solution}
        open={open}
        setOpen={setOpen}
        plan={plan}
      />
      <div>
        <Separator className="max-w-lg mx-auto mb-4" />
        <Muted className="text-center">
          Entwickelt von{" "}
          <HoverCard>
            <HoverCardTrigger asChild>
              <a
                href="https://github.com/jakesarjeant"
                className="text-foreground underline decoration-dashed hover:decoration-solid"
              >
                Jake Sarjeant
              </a>
            </HoverCardTrigger>
            <HoverCardContent>
              <div className="flex justify-start gap-4">
                <Avatar>
                  <AvatarImage src="https://avatars.githubusercontent.com/u/137307920?v=4" />
                  <AvatarFallback>JS</AvatarFallback>
                </Avatar>
                <div className="space-y-1">
                  <h4 className="text-sm font-semibold">@jakesarjeant</h4>
                  <p className="text-xs text-muted-foreground">
                    jake+pk@sarjeant.me
                  </p>
                </div>
              </div>
            </HoverCardContent>
          </HoverCard>{" "}
          am{" "}
          <HoverCard>
            <HoverCardTrigger asChild>
              <a
                href="https://akg-kt.de/"
                className="text-foreground underline decoration-dashed hover:decoration-solid"
              >
                AKG Kitzingen{" "}
              </a>
            </HoverCardTrigger>
            <HoverCardContent>
              <div className="flex justify-start gap-4">
                <Avatar>
                  <AvatarImage
                    className="bg-white"
                    src="https://armin-knab-gymnasium.de/Home/wp-content/uploads/2016/11/cropped-LogoIcon-192x192.png"
                  />
                  <AvatarFallback>AKG</AvatarFallback>
                </Avatar>
                <div className="space-y-1">
                  <h4 className="text-sm font-semibold">
                    Armin-Knab-Gymnasium
                  </h4>
                  <a
                    className="text-xs text-muted-foreground underline hover:decoration-foreground"
                    href="https://akg-kt.de/"
                  >
                    https://akg-kt.de/
                  </a>
                </div>
              </div>
            </HoverCardContent>
          </HoverCard>
        </Muted>
      </div>
    </div>
  );
}

export default App;
