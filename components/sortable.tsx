import { ReactNode, useId, useMemo, type SetStateAction, type Dispatch } from "react"
import { DndContext, closestCenter, type DragEndEvent, type UniqueIdentifier } from "@dnd-kit/core";
import { restrictToVerticalAxis } from "@dnd-kit/modifiers";
import { SortableContext, arrayMove, verticalListSortingStrategy, useSortable } from "@dnd-kit/sortable";

export interface SortableProps<I extends SortableItem> {
  items: I[];
  updateSort: Dispatch<SetStateAction<I[]>>;
  render: (item: I) => ReactNode;
}

export interface SortableItem {
  id: UniqueIdentifier;
}

export default function Sortable<I extends SortableItem>(props: SortableProps<I>) {
  const sortableId = useId();

  // TODO: UniqueIdentifier?
  const dataIds = useMemo<UniqueIdentifier[]>(
    () => props.items?.map(({ id }) => id) || [],
    [props.items]
  )

  function handleDragEnd(event: DragEndEvent) {
    const { active, over } = event
    if (active && over && active.id !== over.id) {
      props.updateSort((data) => {
        const oldIndex = dataIds.indexOf(active.id)
        const newIndex = dataIds.indexOf(over.id)
        return arrayMove(data, oldIndex, newIndex)
      })
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
        {props.items.map(item => (
          <SortableItem render={props.render} item={item} key={item.id} />
        ))}
      </SortableContext>
    </DndContext>
  );
}

function SortableItem<I>({render, item}: {render: (item: I) => ReactNode; item: I}) {
  let {transform, transistion, setNodeRef, isDragging} = useSortable();

    // TODO
  return (
    <>
    </>
  );
}
