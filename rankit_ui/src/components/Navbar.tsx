import styles from './Navbar.module.css';

export default () => (
    <div className={styles.Navbar}>
        <div className={styles.inner}>
            <div className={styles.left}>
                <h2 className={styles.title}>Thingelo</h2>
            </div>
            <div className={styles.right}>
                <div className={styles.loginSignup}>Log in</div>
                |
                <div className={styles.loginSignup}>Sign Up</div>
            </div>
        </div>
    </div>
);