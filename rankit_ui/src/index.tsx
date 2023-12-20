import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import ThemeProvider from './components/ThemeProvider';
import AppDataProvider from './components/AppDataProvider';
import { BrowserRouter } from 'react-router-dom';
import './index.css';
import AccountProvider from './components/AccountProvider';

const htmlRoot = document.getElementById('root') as HTMLElement;
const root = ReactDOM.createRoot(htmlRoot);
root.render(
  <React.StrictMode>
    <BrowserRouter>
      <AccountProvider>
        <AppDataProvider>
          <ThemeProvider>
            <App/>
          </ThemeProvider>
        </AppDataProvider>
      </AccountProvider>
    </BrowserRouter>
  </React.StrictMode>
);