import styles from './Navbar.module.css';

const Navbar = () => (
    <div className={styles.Navbar}>
        <div className={styles.inner}>

            {/* Title */}
            <div className={styles.left}>
                <h2>Thingelo</h2>
            </div>

            {/* Option list */}
            <div className={styles.center}>
                <div className={styles.option}>About</div>
                <div className={styles.option}>Things</div>
                <div className={styles.option}>Categories</div>
            </div>
            
            {/* Login / Sign up */}
            <div className={styles.right}>
                <div className={styles.login}>Log in</div>
                |
                <div className={styles.signup}>Sign Up</div>
            </div>
        </div>
    </div>
);

export default Navbar;