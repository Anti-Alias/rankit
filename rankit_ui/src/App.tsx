import React from 'react';
import logo from './logo.svg';
import './App.css';
import { Header } from './components/header/Header';
import { Outlet } from 'react-router';

function App() {
  return (
    <div className='App'>
      <Header/>
      <div className="content">
        <Outlet/>
      </div>
    </div>
  );
}

export default App;
