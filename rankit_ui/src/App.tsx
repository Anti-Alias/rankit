import styles   from './App.module.css';
import Navbar   from './components/Navbar';
import About    from './pages/About';
import SignUp   from './pages/SignUp';
import LogIn    from './pages/LogIn';
import { BrowserRouter, Routes, Route, Outlet } from 'react-router-dom';
import { Context, createContext } from 'react';

type Theme = 'dark' | 'light';
const ThemeContext: Context<Theme> = createContext<Theme>('light');


/** Main application element */
const App = () => (
  <div className={styles.App}>
    <ThemeContext.Provider value="light">
      <BrowserRouter>
        <Navbar/>
        <Routes>
          <Route path="/" element={<Layout/>}>
            <Route index              element={<About/>}/>
            <Route path="login"       element={<LogIn/>}/>
            <Route path="signup"      element={<SignUp/>}/>
            <Route path="things"      element={<Things/>}/>
            <Route path="categories"  element={<Categories/>}/>
            <Route path="*"           element={<PageNotFound/>}/>
          </Route>
        </Routes>
      </BrowserRouter>
    </ThemeContext.Provider>
  </div>
);

const Things = () => (
  <>
    <h1>Things</h1>
    <p>Under construction</p>
  </>
);

const Categories = () => (
  <>
    <h1>Categories</h1>
    <p>Under construction</p>
  </>
);

const PageNotFound = () => (
  <h1>Page Not Found</h1>
);

// Container for page content
const Layout = () => (
  <div className={styles.Layout}>
    <Outlet/>
  </div>
);

export default App;
