import style from './App.module.css';
import Content from './components/Content';
import Navbar from './components/Navbar';

const App = () => (
  <div className={style.App}>
    <Navbar/>
    <Content/>
  </div>
);

export default App;
