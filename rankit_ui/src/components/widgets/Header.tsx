import './Header.css';
import { NavLink } from "react-router";

export function Header() {
  return (
    <div className="Header">
      <NavLink to="/" className="logo">Rankit</NavLink>
      <div className="center">
        <NavLink to="/" className="navbar-item">Home</NavLink>
        <NavLink to="/things" className="navbar-item">Things</NavLink>
        <NavLink to="/categories" className="navbar-item">Categories</NavLink>
      </div>
      <div className="right">
        <NavLink to="/login" className="navbar-item">Log In</NavLink>
        <NavLink to="/signup" className="navbar-item">Sign Up</NavLink>
      </div>
    </div>
  )
}
