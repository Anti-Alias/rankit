import styles         from './App.module.css';
import Navbar         from './components/Navbar';
import About          from './pages/About';
import SignUp         from './pages/SignUp';
import LogIn          from './pages/LogIn';
import Verification   from './pages/Verification';
import Error          from './pages/Error';
import { Routes, Route, Outlet } from 'react-router-dom';

/** Main application element */
function App() {
  return (
    <div className={styles.App}>
      <Navbar/>
      <Routes>
        <Route path="/" element={<Layout/>}>
          <Route index                element={<About/>}/>
          <Route path="login"         element={<LogIn/>}/>
          <Route path="signup"        element={<SignUp/>}/>
          <Route path="things"        element={<Things/>}/>
          <Route path="categories"    element={<Categories/>}/>
          <Route path="verification"  element={<Verification/>}/>
          <Route path="error"         element={<Error/>}/>
          <Route path="*"             element={<PageNotFound/>}/>
        </Route>
      </Routes>
    </div>
  )
};

function Things() {
  return <>
    <h1>Things</h1>
    <p>Under construction</p>
  </>
}

function Categories() {
  return <>
    <h1>Categories</h1>
    <p>Under construction</p>
  </>
}

function PageNotFound() {
  return <h1>Page Not Found</h1>;
}

// Container for page content
function Layout(): JSX.Element {
  return (
    <div className={styles.Layout}>
      <Outlet/>
    </div>
  );
}

export default App;
