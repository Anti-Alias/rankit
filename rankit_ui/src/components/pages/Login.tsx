import { ChangeEvent, FormEvent, useState } from "react";
import { NavLink } from "react-router";

export function Login() {

  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [submitted, setSubmitted] = useState(false);
  const isValid = !!email && !!password;

  const submit = (event: FormEvent) => {
    event.preventDefault();
    setSubmitted(true);
    if(isValid) {
      alert('TODO: Login logic');
    }
  }


  return (
    <div className="panel">
      <h1>Login</h1>
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
          { submitted && !email && <span className="error">Required</span> }
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
          { submitted && !password && <span className="error">Required</span> }
        </div>
        <button className="primary">Submit</button>
      </form>
      <p>Don't have an account? <NavLink to="/signup">Sign Up</NavLink></p>
    </div>
  );
}
