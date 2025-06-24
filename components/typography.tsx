import { HTMLAttributes } from "react";
import { cn } from "@/lib/utils";

export function H1(props: React.HTMLAttributes<HTMLHeadingElement>) {
  return (
    <h1
      {...props}
      className={cn(
        "scroll-m-20 text-center text-4xl font-extrabold tracking-tight text-balance",
        props.className,
      )}
    />
  );
}

export function H2(props: React.HTMLAttributes<HTMLHeadingElement>) {
  return (
    <h2
      {...props}
      className={cn(
        "scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight first:mt-0",
        props.className,
      )}
    />
  );
}

export function H3(props: React.HTMLAttributes<HTMLHeadingElement>) {
  return (
    <h3
      {...props}
      className={cn(
        "scroll-m-20 text-2xl font-semibold tracking-tight",
        props.className,
      )}
    />
  );
}

export function H4(props: React.HTMLAttributes<HTMLHeadingElement>) {
  return (
    <h4
      {...props}
      className={cn(
        "scroll-m-20 text-xl font-semibold tracking-tight",
        props.className,
      )}
    />
  );
}

export function P(props: React.HTMLAttributes<HTMLParagraphElement>) {
  return <p className="leading-7 [&:not(:first-child)]:mt-6" {...props}></p>;
}

export function Blockquote(props: React.HTMLAttributes<HTMLQuoteElement>) {
  return <blockquote className="mt-6 border-l-2 pl-6 italic" {...props} />;
}

export function Table(props: HTMLAttributes<HTMLTableElement>) {
  return (
    <div className="my-6 w-full overflow-y-auto">
      <table className="w-full" {...props}></table>
    </div>
  );
}

export function TableRow(props: HTMLAttributes<HTMLTableRowElement>) {
  return <tr className="even:bg-muted m-0 border-t p-0" {...props} />;
}

export function TableHeader(props: HTMLAttributes<HTMLTableCellElement>) {
  return (
    <th
      className="border px-4 py-2 text-left font-bold [&[align=center]]:text-center [&[align=right]]:text-right"
      {...props}
    />
  );
}

export function TableData(props: HTMLAttributes<HTMLTableCellElement>) {
  return (
    <td
      className="border px-4 py-2 text-left [&[align=center]]:text-center [&[align=right]]:text-right"
      {...props}
    />
  );
}

export function List(props: HTMLAttributes<HTMLUListElement>) {
  return <ul className="my-6 ml-6 list-disc [&>li]:mt-2" {...props} />;
}

export function Code(props: HTMLAttributes<HTMLSpanElement>) {
  return (
    <code
      className="bg-muted relative rounded px-[0.3rem] py-[0.2rem] font-mono text-sm font-semibold"
      {...props}
    />
  );
}

export function Lead(props: HTMLAttributes<HTMLParagraphElement>) {
  return <p className="text-muted-foreground text-xl" {...props} />;
}

export function Large(props: HTMLAttributes<HTMLDivElement>) {
  return <div className="text-lg font-semibold" {...props} />;
}

export function Small(props: HTMLAttributes<HTMLSpanElement>) {
  return <small className="text-sm leading-none font-medium" {...props} />;
}

export function Muted(props: HTMLAttributes<HTMLParagraphElement>) {
  return (
    <p
      {...props}
      className={cn("text-muted-foreground text-sm", props.className)}
    />
  );
}
