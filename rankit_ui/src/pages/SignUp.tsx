import { FormEvent, useState } from 'react';
import Button from '../components/Button';
import styles from './SignUp.module.css';
import { Link } from 'react-router-dom';

interface Input {
  email?: string,
  username?: string,
  password?: string,
};

const SignUp = () => {

    const [email, setEmail] = useState({});
    const [username, setUsername] = useState({});
    const [password, setPassword] = useState({});

    const signUp = (event: FormEvent) => {
      event.preventDefault();
      console.log("Signing up");
    }

    console.log('Rendering sign up');

    return (
      <form className={styles.SignUp} onSubmit={signUp}>
        
        <h1>Sign Up</h1>
        <div className={styles.inputWrapper}>
          <span className={styles.label}>Email</span>
          <input type="email" name="email" onChange = {e => setEmail(e.target.value)}/>
        </div>
        <div className={styles.inputWrapper}>
          <span className={styles.label}>Username</span>
          <input type="text" name="username" onChange = {e => setUsername(e.target.value)}/>
        </div>
        <div className={styles.inputWrapper}>
          <span className={styles.label}>Password</span>
          <input type="password" name="password" onChange = {e => setPassword(e.target.value)}/>
        </div>

        <p>Already a member? <Link to="/login" className={styles.link}>Log In</Link></p>
        <Button type="submit">Submit</Button>

      </form>
    );
};

export default SignUp;