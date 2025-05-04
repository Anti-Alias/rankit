import { NavLink } from 'react-router';
import styles from './SignUp.module.css';

export function SignUp() {
  return (
    <div className="panel">
      <h1 className={styles.header}>Sign Up</h1>
      <form>
        <input name="email" type="email" placeholder="Email"/>
        <input name="password" type="password" placeholder="Password"/>
        <input name="password-verify" type="password" placeholder="Password Verify"/>
        <button className="primary">Submit</button>
      </form>
      <span className={styles.message}>
        Already have an account? <NavLink to="/construction">Log In</NavLink>
      </span>
    </div>
  )
}
