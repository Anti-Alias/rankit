import style from './App.module.css';
import Content from './components/Content';
import Navbar from './components/Navbar';

export default () => (
  <div className={style.App}>
    <Navbar/>
    <Content/>
  </div>
);
