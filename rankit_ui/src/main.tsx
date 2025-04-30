import './index.css'
import App from './App.tsx'
import { Home } from './components/pages/Home.tsx';
import { Things } from './components/pages/Things.tsx';
import { Categories } from './components/pages/Categories.tsx';
import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { BrowserRouter, Routes, Route } from "react-router";

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<App />}>
          <Route index element={<Home />}></Route>
          <Route path="things" element={<Things />}></Route>
          <Route path="categories" element={<Categories />}></Route>
        </Route>
      </Routes>
    </BrowserRouter>
  </StrictMode>,
)
