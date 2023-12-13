import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import ThemeProvider from './components/ThemeProvider';
import { BrowserRouter } from 'react-router-dom';
import './index.css';

const htmlRoot = document.getElementById('root') as HTMLElement;
const root = ReactDOM.createRoot(htmlRoot);
root.render(
  <React.StrictMode>
    <BrowserRouter>
        <ThemeProvider>
          <App/>
        </ThemeProvider>
    </BrowserRouter>
  </React.StrictMode>
);