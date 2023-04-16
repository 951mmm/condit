import { useAtom, useAtomValue } from "jotai";
import { NavLink } from "react-router-dom";
import { atomUser } from "../stores/auth";

export function UserAvatar() {
  // ANCHOR store
  const user = useAtomValue(atomUser);
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
