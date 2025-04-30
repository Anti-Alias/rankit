import './Header.css';
import { NavLink } from "react-router";

export function Header() {
  return (
    <div className="Header">
      <NavLink to="/" className="logo">Rankit</NavLink>
      <div className="center">
        <NavLink to="/" className="item">Home</NavLink>
        <NavLink to="/things" className="item">Things</NavLink>
        <NavLink to="/categories" className="item">Categories</NavLink>
      </div>
      <div className="right">
        <button type="button" className='signin-login'>Sign In</button>
        <button type="button" className='signin-login'>Sign Up</button>
      </div>
    </div>
  )
}
