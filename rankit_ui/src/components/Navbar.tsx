import Button from './Button';
import styles from './Navbar.module.css';
import { Link } from 'react-router-dom';
import { useContext } from 'react';
import { ThemeNameContext } from './ThemeProvider';
import { capitalize } from '../utils/string';
import { DisplayModeContext } from './DisplayModeProvider';

function Navbar() {

    const [themeName, setThemeName] = useContext(ThemeNameContext);
    const displayMode = useContext(DisplayModeContext);

    const toggleTheme = () => {
        if(themeName === 'light') {
            setThemeName('dark');
        }
        else {
            setThemeName('light');
        }
    }


    const innerClassName = `${styles.inner} ${styles[displayMode]}`;
    return (
        <div className={styles.Navbar}>
            <div className={innerClassName}>
                <div className={styles.left}>
                    <Link to='/' className={styles.logo}><h2>Thingelo</h2></Link>
                </div>
                { displayMode !== 'mobile' &&
                    <div className={styles.center}>
                        <Link to='/' className={styles.option}>About</Link>
                        <Link to='/things' className={styles.option}>Things</Link>
                        <Link to='/categories' className={styles.option}>Categories</Link>
                    </div>
                }
                <div className={styles.right}>
                    <Link to='/login' className={styles.login}>Log In</Link> |
                    <Link to='/signup' className={styles.signup}>Sign Up</Link>
                    <Button size="small" color="secondary" onClick={toggleTheme}>
                        {capitalize(themeName)}
                    </Button>
                </div>
            </div>
        </div>
    )
};

export default Navbar;