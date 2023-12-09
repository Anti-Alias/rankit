import styles from './Navbar.module.css';
import { Link } from 'react-router-dom';

const Navbar = () => (
    <div className={styles.Navbar}>
        <div className={styles.inner}>
            <div className={styles.left}>
                <Link to='/' className={styles.site}><h2>Thingelo</h2></Link>
            </div>
            <div className={styles.center}>
                <Link to='/' className={styles.option}>About</Link>
                <Link to='/things' className={styles.option}>Things</Link>
                <Link to='/categories' className={styles.option}>Categories</Link>
            </div>
            <div className={styles.right}>
                <Link to='/login' className={styles.login}>Log In</Link> |
                <Link to='/signup' className={styles.signup}>Sign Up</Link>
            </div>
        </div>
    </div>
);

export default Navbar;