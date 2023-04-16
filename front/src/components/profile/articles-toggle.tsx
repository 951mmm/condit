import { useAtom } from "jotai";
import { NavLink, useParams } from "react-router-dom";
import { atomFeedQueryType, atomSetFeedQuery } from "../../stores/feed";
import { useEffect } from "react";

interface ArticlesToggleProps {
  userId: string;
}

export function ArtiflesToggle({ userId }: ArticlesToggleProps) {
  // ANCHOR store
  const [feedType] = useAtom(atomFeedQueryType);
  const [, setFeedType] = useAtom(atomSetFeedQuery);

  // ANCHOR effect
  useEffect(() => {
    setFeedType(["private", userId]);
  }, [userId]);
  return (
    <div className="articles-toggle">
      <ul className="nav nav-pills outline-active">
        <li className="nav-item">
          <NavLink
            className={({ isActive }) => `nav-link ${isActive ? "active" : ""}`}
            end
            to={encodeURI(`/profile/${userId}`)}
            onClick={() => {
              setFeedType(["private", userId]);
            }}
          >
            My Articles
          </NavLink>
        </li>
        <li className="nav-item">
          <NavLink
            className={({ isActive }) => `nav-link ${isActive ? "active" : ""}`}
            end
            to={encodeURI(`/profile/${userId}/favorites`)}
            onClick={() => {
              setFeedType(["favorites", userId]);
            }}
          >
            Favorited Articles
          </NavLink>
        </li>
      </ul>
    </div>
  );
}
