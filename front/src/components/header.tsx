import { Link, NavLink, useLocation } from "react-router-dom"
import { LoginHeaderInner } from "./login-header-inner"
import { LogoutHeaderInner } from "./logout-header-inner"
import { useAtom } from "jotai"
import { atomIsLogin } from "../stores/auth"
import { SearchBar } from "./common/search-bar"

export function Header() {
  // ANCHOR store
  const [isLogin] = useAtom(atomIsLogin);
  const location = useLocation();

  // ANCHOR render
  return (
    <nav className="navbar navbar-light">
      <div className="container">
        <Link to="/" className="navbar-brand">
          conduit
        </Link>
        {/login|register|settings|editor/.test(location.pathname) ? <></> : <SearchBar />}
        <ul className="nav navbar-nav pull-xs-right">
          <li className="nav-item">
            <NavLink to="/" className={({ isActive }) => `nav-link ${isActive ? 'active ' : ''}`}>
              Home
            </NavLink>
          </li>
          {isLogin ? <LoginHeaderInner /> : <LogoutHeaderInner />}
        </ul>
      </div>
    </nav>
  )
}