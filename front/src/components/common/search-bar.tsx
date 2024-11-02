import { useAtom, useSetAtom } from "jotai";
import { ChangeEvent, CSSProperties, FormEvent, useState} from "react";
import { atomQueryString } from "../../stores/feed";

const searchbarStyle: CSSProperties = {
  width: "50%",
  display: "inline-block",
};

export function SearchBar() {
  // ANCHOR state
  const [query, setQuery] = useState("");

  // ANCHOR store
  const setQueryString = useSetAtom(atomQueryString);

  // ANCHOR event
  function onChange(e: ChangeEvent<HTMLInputElement>) {
    const { value } = e.target;
    setQuery(value);
  }

  function onSubmit(e: FormEvent<HTMLFormElement>) {
    e.preventDefault()
    setQueryString(query);
  }

// ANCHOR render
  return (
    <form style={searchbarStyle} onSubmit={onSubmit}>
      <input 
      className="form-control" 
      name="search" 
      placeholder="search..."
      value={query}
      onChange={onChange} 
      />
    </form>
  );
}
