import { Link, NavLink } from "react-router-dom"
import { LoginHeaderInner } from "./login-header-inner"
import { LogoutHeaderInner } from "./logout-header-inner"
import { useAtom } from "jotai"
import { atomIsLogin } from "../stores/auth"

export function Header() {
  const [isLogin] = useAtom(atomIsLogin);

  return (
    <nav className="navbar navbar-light">
      <div className="container">
        <Link to="/" className="navbar-brand">
          conduit
        </Link>
        <ul className="nav navbar-nav pull-xs-right">
          <li className="nav-item">
            <NavLink to="/" className={( {isActive} ) => `nav-link ${isActive ? 'active ': ''}`}>
              Home
            </NavLink>
          </li>
          {isLogin ? <LoginHeaderInner /> : <LogoutHeaderInner />}
        </ul>
      </div>
    </nav>
  )
}