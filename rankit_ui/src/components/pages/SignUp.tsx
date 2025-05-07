import { NavLink } from 'react-router';
import { ChangeEvent, FormEvent, useState } from 'react';
import styles from './SignUp.module.css';

/// Source: https://emailregex.com/
const emailRegex: RegExp = /^(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])$/;

export function SignUp() {

  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [passwordVerify, setPasswordVerify] = useState('');
  const [submitted, setSubmitted] = useState(false);

  const submit = (event: FormEvent) => {
    event.preventDefault();
    setSubmitted(true);
  }

  return (
    <div className="panel">
      <h1 className={styles.header}>Sign Up</h1>
      <form onSubmit={submit} noValidate>
        <div className="input-group">
          <input
            name="email"
            type="email"
            value={email}
            onChange={(event: ChangeEvent<HTMLInputElement>) => setEmail(event.target.value) }
            placeholder="Email"
            required
          />
          { 
            submitted && !email &&
            <span className="error">Required</span>
          }
          { 
            submitted && email && !email.match(emailRegex) &&
            <span className="error">Invalid Email</span>
          }
        </div>
        <div className="input-group">
          <input
            name="password"
            type="password"
            value={password}
            onChange={(event: ChangeEvent<HTMLInputElement>) => setPassword(event.target.value) }
            placeholder="Password"
            required
          />
          {
            submitted && !password &&
            <span className="error">Required</span>
          }
        </div>
        <div className="input-group">
          <input
            name="password-verify"
            type="password"
            value={passwordVerify}
            onChange={(event: ChangeEvent<HTMLInputElement>) => setPasswordVerify(event.target.value) }
            placeholder="Password Verify"
            required
          />
          {
            submitted && !passwordVerify &&
            <span className="error">Required</span>
          }
          {
            submitted && password && passwordVerify && password != passwordVerify &&
            <span className="error">Passwords do not match</span>
          }
        </div>
        <button className="primary">Submit</button>
      </form>
      <span className={styles.message}>
        Already have an account? <NavLink to="/construction">Log In</NavLink>
      </span>
    </div>
  )
}
