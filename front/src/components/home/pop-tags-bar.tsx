import { useState, useEffect } from "react";
import { Tag } from "../../api/tag";
import { errHandler } from "../../utils";
import { LinkTag } from "./link-tag";
import { useAtomValue } from "jotai";
import { atomQueryString } from "../../stores/feed";

export function PopTagsBar() {
    // ANCHOR state
    const [loading, setLoading] = useState(false);
    const [tags, setTags] = useState<string[]>([]);

    // ANCHOR store
    const queryString = useAtomValue(atomQueryString);
  
    // ANCHOR initial effect
    useEffect(() => {
      async function initTags() {
        setLoading(true);
        try {
          const { tags } = await Tag.list.handler(queryString);
          setTags(tags);
        } catch (e) {
          errHandler(e);
        }
        setLoading(false);
      }
      initTags();
    }, [queryString]);
  return (
    <div className="sidebar">
      <p>Popular Tags</p>
      <div className="tag-list">
        {loading ? (
          <p>loading...</p>
        ) : (
          tags.map((tag, index) => <LinkTag key={index} name={tag} />)
        )}
      </div>
    </div>
  );
}
