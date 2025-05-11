import './index.css'
import App from './App.tsx'
import { Home } from './components/pages/Home.tsx';
import { ThingList } from './components/pages/ThingList.tsx';
import { CategoryList } from './components/pages/CategoryList.tsx';
import { Thing } from './components/pages/Thing.tsx';
import { SignUp } from './components/pages/SignUp.tsx';
import { Login } from './components/pages/Login.tsx';
import { UnderConstruction } from './components/pages/UnderConstruction.tsx';
import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { BrowserRouter, Routes, Route } from "react-router";

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<App/>}>
          <Route index element={<Home/>}/>
          <Route path="signup" element={<SignUp/>}/>
          <Route path="login" element={<Login/>}/>
          <Route path="things" element={<ThingList/>}/>
          <Route path="things/:id" element={<Thing/>}/>
          <Route path="categories" element={<CategoryList/>}/>
          <Route path="construction" element={<UnderConstruction/>}/>
        </Route>
      </Routes>
    </BrowserRouter>
  </StrictMode>,
)
