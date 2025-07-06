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
