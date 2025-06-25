const { ipcRenderer } = require('electron')

document.addEventListener('DOMContentLoaded', () => {
  const input = document.getElementById('mainInput')

  input.addEventListener('keydown', (event) => {
    if (event.key === 'Enter') {
      const value = input.value.trim().toLowerCase()

      if (value === 'grid' || value === 'g') {
        // Trigger grid mode
        ipcRenderer.send('open-grid')
        input.value = ''
        // Hide main window
        ipcRenderer.send('hide-main-window')
      }
    }
  })

  // Focus input when window is shown
  input.focus()
})
