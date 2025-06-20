// Main window JavaScript
const { ipcRenderer } = require('electron')

// DOM elements
const settingsButton = document.getElementById('settingsButton')
const mainInput = document.getElementById('mainInput')

// Handle settings button click
settingsButton.addEventListener('click', () => {
  ipcRenderer.send('navigate-to-settings')
})

// Focus input on load
window.addEventListener('DOMContentLoaded', () => {
  mainInput.focus()
})

// Handle escape key
document.addEventListener('keydown', (event) => {
  if (event.key === 'Escape') {
    ipcRenderer.send('hide-window')
  }
})