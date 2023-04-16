import { NavLink } from "react-router-dom";
import { UserAvatar } from "./user-avatar";

export function LoginHeaderInner() {
  return (
    <>
      <li className="nav-item">
        <NavLink
          to="/editor"
          className={({ isActive }) => `nav-link ${isActive ? "active" : ""}`}
        >
        <i className="ion-compose"></i> New Article
        </NavLink>
      </li>
      <li className="nav-item">
        <NavLink
          to="/settings"
          className={({ isActive }) => `nav-link ${isActive ? "active" : ""}`}
        >
        <i className="ion-gear-a"></i> Settings
        </NavLink>
      </li>
      <UserAvatar />
    </>
  );
}
