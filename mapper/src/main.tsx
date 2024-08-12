import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import Mapper from './components/Mapper.tsx'
import './index.css'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <Mapper />
  </StrictMode>,
)
