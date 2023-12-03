import styles from './App.module.css';
import About from './pages/About';
import SignUp from './pages/SignUp';
import Navbar from './components/Navbar';
import { BrowserRouter, Routes, Route, Outlet } from 'react-router-dom';


/** Main application element */
const App = () => (
  <div className={styles.App}>
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

const LogIn = () => (
  <>
    <h1>Log in</h1>
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
