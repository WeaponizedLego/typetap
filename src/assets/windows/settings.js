// Settings page JavaScript
const { ipcRenderer } = require('electron')

// DOM elements
const backButton = document.getElementById('backButton')
const gridModeRadio = document.getElementById('gridMode')
const labelModeRadio = document.getElementById('labelMode')
const useAnimationsCheckbox = document.getElementById('useAnimations')

// Load current settings when page loads
window.addEventListener('DOMContentLoaded', async () => {
  try {
    const settings = await ipcRenderer.invoke('get-settings')
    
    // Update form with current settings
    if (settings.overlayMode === 'grid') {
      gridModeRadio.checked = true
    } else {
      labelModeRadio.checked = true
    }
    
    useAnimationsCheckbox.checked = settings.useAnimations
  } catch (error) {
    console.error('Failed to load settings:', error)
  }
})

// Handle back button
backButton.addEventListener('click', () => {
  ipcRenderer.send('navigate-to-main')
})

// Handle overlay mode changes
gridModeRadio.addEventListener('change', () => {
  if (gridModeRadio.checked) {
    updateSetting('overlayMode', 'grid')
  }
})

labelModeRadio.addEventListener('change', () => {
  if (labelModeRadio.checked) {
    updateSetting('overlayMode', 'label')
  }
})

// Handle animations checkbox
useAnimationsCheckbox.addEventListener('change', () => {
  updateSetting('useAnimations', useAnimationsCheckbox.checked)
})

// Update a setting
async function updateSetting(key, value) {
  try {
    await ipcRenderer.invoke('update-setting', { [key]: value })
  } catch (error) {
    console.error('Failed to update setting:', error)
  }
}

// Handle escape key to go back
document.addEventListener('keydown', (event) => {
  if (event.key === 'Escape') {
    ipcRenderer.send('navigate-to-main')
  }
})