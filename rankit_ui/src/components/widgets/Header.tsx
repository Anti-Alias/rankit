import './Header.css';
import { NavLink } from "react-router";

export function Header() {
  return (
    <div className="Header">
      <NavLink to="/" className="logo">Rankit</NavLink>
      <div className="center">
        <NavLink to="/" className="nav-item">Home</NavLink>
        <NavLink to="/things" className="nav-item">Things</NavLink>
        <NavLink to="/categories" className="nav-item">Categories</NavLink>
      </div>
      <div className="right">
        <NavLink to="/login" className="nav-item">Log In</NavLink>
        <NavLink to="/signup" className="nav-item">Sign Up</NavLink>
      </div>
    </div>
  )
}
