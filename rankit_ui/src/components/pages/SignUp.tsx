import { NavLink } from 'react-router';
import { ChangeEvent, FormEvent, useState } from 'react';
import { validateEmail, validatePassword, validatePasswordVerify } from '../../utils/validation.ts';

export function SignUp() {

  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [passwordVerify, setPasswordVerify] = useState('');
  const [submitted, setSubmitted] = useState(false);

  const emailError          = validateEmail(email);
  const passwordError       = validatePassword(password);
  const passwordVerifyError = validatePasswordVerify(passwordVerify, password);
  const isValid = !emailError && !passwordError && !passwordVerifyError;

  const submit = (event: FormEvent) => {
    event.preventDefault();
    setSubmitted(true);
    if(!isValid) {
      setSubmitted(true);
    }
    else {
      alert('TODO: Submit logic');
    }
  }

  return (
    <div className="panel">
      <h1>Sign Up</h1>
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
          { submitted && emailError && <span className="error">{emailError}</span> }
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
          { submitted && passwordError && <span className="error">{passwordError}</span> }
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
          { submitted && !passwordError && passwordVerifyError && <span className="error">{passwordVerifyError}</span> }
        </div>
        <button className="primary">Submit</button>
      </form>
      <p>Already have an account? <NavLink to="/login">Log In</NavLink></p>
    </div>
  )
}
