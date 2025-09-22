import { WilliStundenplan } from "willi";

/** Computes a map of short names to number of recorded periods. Subjects not appearing in this map
 * are dead files. */
export function computeSubjectCounts(plan: WilliStundenplan): { [kuerzel: string]: number } {
  const result = {};

  for (const entry of Object.values(plan.stunden_lehrerplan())) {
    result[entry.fach] = (result[entry.fach] || 0) + 1;
  }

  return result;
}
