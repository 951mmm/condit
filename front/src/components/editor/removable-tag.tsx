import { useSetAtom } from "jotai";
import { atomTagList } from "../../stores/editor";

interface RemovableTagProps {
  name: string;
}
export function RemovableTag({ name }: RemovableTagProps) {
  // ANCHOR store
  const setTagList = useSetAtom(atomTagList);

  // ANCHOR event
  function onDeleteTag() {
    setTagList((tagList) => tagList.filter((tagInner) => tagInner !== name));
  }
  return (
    <span className="tag-default tag-pill">
      <i className="ion-close-round" onClick={onDeleteTag} />
      {` ${name} `}
    </span>
  );
}
