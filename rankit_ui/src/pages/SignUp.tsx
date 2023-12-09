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

    const [email, setEmail] = useState('');
    const [username, setUsername] = useState('');
    const [password, setPassword] = useState('');

    const submit = (event: FormEvent) => {
      event.preventDefault();
      console.log('Signing up!!!');
      console.log(event.target);
    }
    return (
      <form className={styles.SignUp} onSubmit={submit}>
        
        <h1>Sign Up</h1>
        <label className={styles.inputWrapper}>
          <span className={styles.label}>Email</span>
          <input type="email" name="email" onChange = {e => setEmail(e.target.value)}/>
        </label>
        <label className={styles.inputWrapper}>
          <span className={styles.label}>Username</span>
          <input type="text" name="username" onChange = {e => setUsername(e.target.value)}/>
        </label>
        <label className={styles.inputWrapper}>
          <span className={styles.label}>Password</span>
          <input type="password" name="password" onChange = {e => setPassword(e.target.value)}/>
        </label>

        <p>Already a member? <Link to="/login" className={styles.link}>Log In</Link></p>
        <Button type="submit">Submit</Button>

      </form>
    );
};

export default SignUp;