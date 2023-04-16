import { useAtom } from "jotai";
import { NavLink } from "react-router-dom";
import { atomUser } from "../stores/auth";
import { atomSetFeedQuery } from "../stores/feed";

export function UserAvatar() {
  // ANCHOR store
  const [user] = useAtom(atomUser);
  const [, setFeedType] = useAtom(atomSetFeedQuery);
  return (
    <li className="nav-item">
      <NavLink
        to={encodeURI(`/profile/${user.username}`)}
        className={({ isActive }) => `nav-link ${isActive ? "active" : ""}`}
      >
        <img src={user.image} className="user-pic" />
        {user.username}
      </NavLink>
    </li>
  );
}
