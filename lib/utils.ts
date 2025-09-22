import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function download(file: File) {
  console.log("downloading", file);

  const a = document.createElement("a");
  a.style.display = "none";

  a.href = URL.createObjectURL(file);
  a.download = file.name;

  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
}

export function humanTime(seconds) {
  const levels = [
    [Math.floor(seconds / 31536000), 'Jahre'],
    [Math.floor((seconds % 31536000) / 86400), 'Tage'],
    [Math.floor(((seconds % 31536000) % 86400) / 3600), 'Stunden'],
    [Math.floor((((seconds % 31536000) % 86400) % 3600) / 60), 'Minuten'],
    [(((seconds % 31536000) % 86400) % 3600) % 60, 'Sekunden'],
  ];
  let returntext = '';

  for (let i = 0, max = levels.length; i < max; i++) {
    if (levels[i][0] === 0) continue;
    returntext +=
      ' ' + levels[i][0]
      + ' '
      + (levels[i][0] === 1
          ? (levels[i][1] as string).substr(0, (levels[i][1] as string).length - 1)
          : levels[i][1]);
  };
  return returntext.trim();
}
