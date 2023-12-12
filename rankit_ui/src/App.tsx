import styles         from './App.module.css';
import Navbar         from './components/Navbar';
import About          from './pages/About';
import SignUp         from './pages/SignUp';
import LogIn          from './pages/LogIn';
import { Routes, Route, Outlet } from 'react-router-dom';
import { DisplayModeContext } from './components/DisplayModeProvider';
import { useContext } from 'react';

/** Main application element */
function App() {
  return (
    <div className={styles.App}>
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
    </div>
  )
};

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
function Layout(): JSX.Element {
  const displayMode = useContext(DisplayModeContext);
  const className = `${styles.Layout} ${styles[displayMode]}`;
  return (
    <div className={className}>
      <Outlet/>
    </div>
  );
}

export default App;
