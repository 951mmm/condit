import { useAtom, useAtomValue, useSetAtom } from "jotai";
import {
  atomFeedQueryType,
  atomTagName,
  atomSetFeedQuery,
} from "../../stores/feed";
import { atomIsLogin } from "../../stores/auth";
import { useEffect } from "react";
import { Link } from "react-router-dom";

export function FeedToggle() {
  // ANCHOR store
  const isLogin = useAtomValue(atomIsLogin);
  const feedType = useAtomValue(atomFeedQueryType);
  const tagName = useAtomValue(atomTagName);
  const setFeedType = useSetAtom(atomSetFeedQuery);

  // ANCHOR initialize
  useEffect(() => {
    if (isLogin) setFeedType(["user"]);
    else setFeedType(["global"]);
  }, []);
  return (
    <div className="feed-toggle">
      <ul className="nav nav-pills outline-active">
        {isLogin && (
          <li className="nav-item">
            <Link
              to={"/"}
              className={`nav-link ${feedType === "user" ? "active" : ""}`}
              onClick={() => {
                setFeedType(["user"]);
              }}
            >
              Your Feed
            </Link>
          </li>
        )}

        <li className="nav-item">
          <Link
            to={"/"}
            className={`nav-link ${feedType === "global" ? "active" : ""}`}
            onClick={() => {
              setFeedType(["global"]);
            }}
          >
            Global Feed
          </Link>
        </li>

        {feedType === "tag" && (
          <li className="nav-item">
            <a href="" className="nav-link active">
              <i className="ion-pound" />
              {tagName}{" "}
            </a>
          </li>
        )}
      </ul>
    </div>
  );
}
