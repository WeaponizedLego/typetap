// Main window TypeScript
const { ipcRenderer } = require('electron')

// DOM elements
const settingsButton = document.getElementById('settingsButton') as HTMLButtonElement
const mainInput = document.getElementById('mainInput') as HTMLInputElement

// Handle settings button click
settingsButton?.addEventListener('click', () => {
  ipcRenderer.send('navigate-to-settings')
})

// Focus input on load
window.addEventListener('DOMContentLoaded', () => {
  mainInput?.focus()
})

// Handle escape key
document.addEventListener('keydown', (event: KeyboardEvent) => {
  if (event.key === 'Escape') {
    ipcRenderer.send('hide-window')
  }
})