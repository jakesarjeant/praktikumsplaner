import {
  ReactNode,
  useId,
  useMemo,
  type SetStateAction,
  type Dispatch,
} from "react";
import {
  DndContext,
  closestCenter,
  type DragEndEvent,
  type UniqueIdentifier,
} from "@dnd-kit/core";
import { restrictToVerticalAxis } from "@dnd-kit/modifiers";
import {
  SortableContext,
  arrayMove,
  verticalListSortingStrategy,
  useSortable,
} from "@dnd-kit/sortable";

export type RenderItem<I extends SortableItem> = (
  args: ReturnType<typeof useSortable> & { item: I; idx: number },
) => ReactNode;

export interface SortableProps<I extends SortableItem> {
  items: I[];
  updateSort: Dispatch<(cur: I[]) => I[]>;
  render: RenderItem<I>;
}

export interface SortableItem {
  id: UniqueIdentifier;
}

export default function Sortable<I extends SortableItem>(
  props: SortableProps<I>,
) {
  const sortableId = useId();

  // TODO: UniqueIdentifier?
  const dataIds = useMemo<UniqueIdentifier[]>(
    () => props.items?.map(({ id }) => id) || [],
    [props.items],
  );

  function handleDragEnd(event: DragEndEvent) {
    const { active, over } = event;
    if (active && over && active.id !== over.id) {
      props.updateSort((data) => {
        const oldIndex = dataIds.indexOf(active.id);
        const newIndex = dataIds.indexOf(over.id);
        return arrayMove(data, oldIndex, newIndex);
      });
    }
  }

  return (
    <DndContext
      collisionDetection={closestCenter}
      modifiers={[restrictToVerticalAxis]}
      onDragEnd={handleDragEnd}
      id={sortableId}
    >
      <SortableContext
        items={props.items}
        strategy={verticalListSortingStrategy}
      >
        {props.items.map((item, idx) => (
          <SortableItem
            render={props.render}
            idx={idx}
            item={item}
            key={item.id}
          />
        ))}
      </SortableContext>
    </DndContext>
  );
}

function SortableItem<I extends SortableItem>({
  render,
  item,
  idx,
}: {
  render: RenderItem<I>;
  item: I;
  idx: number;
}) {
  const sortable = useSortable({
    id: item.id,
  });

  // TODO
  return render({ ...sortable, item, idx });
}
