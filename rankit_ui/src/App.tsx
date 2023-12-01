import styles from './App.module.css';
import About from './pages/About';
import Navbar from './components/Navbar';
import { BrowserRouter, Routes, Route, Outlet } from 'react-router-dom';


const App = () => (
  <div className={styles.App}>
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Layout/>}>
          <Route index      element={<About/>}/>
          <Route path="*"   element={<PageNotFound/>}/>
        </Route>
      </Routes>
    </BrowserRouter>
  </div>
);

const Layout = () => (
  <>
    <Navbar/>
    <div className={styles.Content}>
      <Outlet/>
    </div>
  </>
);

const PageNotFound = () => (
  <h1>Page Not Found</h1>
);

export default App;
