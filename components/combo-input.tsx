import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandSeparator,
  CommandList,
} from "@/components/ui/command";
import {
  Popover,
  PopoverAnchor,
  PopoverContent,
} from "@/components/ui/popover";
import {
  useEffect,
  useRef,
  useState,
  useMemo,
  forwardRef,
  createContext,
  useContext,
  type ComponentProps,
  type RefObject,
} from "react";

const ComboContext = createContext(null);

export const ComboInput = forwardRef<
  HTMLInputElement,
  React.PropsWithChildren<
    React.ComponentProps<typeof CommandInput> &
      React.HTMLAttributes<HTMLInputElement> & {
        open?: boolean;
        setOpen?: React.Dispatch<React.SetStateAction<boolean>>;
      }
  >
>(function (
  { children, open: givenOpen, setOpen: givenSetOpen, ...props },
  ref,
) {
  const popoverRef = useRef<Element>(null);
  const [width, setWidth] = useState(0);

  const [inputVal, setInputVal] = useState("");

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

  // Use own open by default, but allow it to be controlled
  const defaultOpen = useState(false);
  const [open, setOpen] =
    !!givenOpen && !!givenSetOpen ? [givenOpen, givenSetOpen] : defaultOpen;

  return (
    <Command
      className="border-1 bg-background"
      ref={popoverRef as RefObject<HTMLDivElement>}
    >
      <Popover open={open} onOpenChange={setOpen}>
        <PopoverAnchor>
          <CommandInput
            wrapperClassName="border-0"
            value={inputVal}
            onValueChange={(v) => {
              setInputVal(v);
              if (!open) {
                setOpen(true);
              }
            }}
            ref={ref}
            onFocus={() => setTimeout(() => setOpen(true), 0)}
            onClick={() => setTimeout(() => setOpen(true), 0)}
            onBlur={() => setOpen(false)}
            {...props}
          />
        </PopoverAnchor>
        <ComboContext value={[setOpen, setInputVal]}>
          <PopoverContent
            className="bg-background mt-1 p-1 shadow-lg"
            style={{ width: `${width}px` }}
            onOpenAutoFocus={(e) => e.preventDefault()}
          >
            <CommandList>{children}</CommandList>
          </PopoverContent>
        </ComboContext>
      </Popover>
    </Command>
  );
});

export const ComboEmpty = CommandEmpty;
export const ComboGroup = CommandGroup;
export const ComboSeparator = CommandSeparator;

export function ComboItem({
  onSelect,
  ...props
}: React.ComponentProps<typeof CommandItem>) {
  const [setOpen, setInputVal] = useContext(ComboContext);

  return (
    <CommandItem
      onSelect={(val) => {
        setOpen(false);
        setInputVal("");
        onSelect(val);
      }}
      {...props}
    />
  );
}
