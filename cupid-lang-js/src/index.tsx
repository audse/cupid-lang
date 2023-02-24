import { createRoot } from 'react-dom/client'
import { Home } from '@/pages'

function startReact () {
    const root = createRoot(document.querySelector('#reactroot') as HTMLElement)
    root.render(<Home />)
}

globalThis.addEventListener('DOMContentLoaded', () => {
    startReact()
})

startReact()