// Settings page TypeScript
const { ipcRenderer } = require('electron')

interface Settings {
  overlayMode: 'grid' | 'label'
  useAnimations: boolean
}

// DOM elements
const backButton = document.getElementById('backButton') as HTMLButtonElement
const gridModeRadio = document.getElementById('gridMode') as HTMLInputElement
const labelModeRadio = document.getElementById('labelMode') as HTMLInputElement
const useAnimationsCheckbox = document.getElementById('useAnimations') as HTMLInputElement

// Load current settings when page loads
window.addEventListener('DOMContentLoaded', async () => {
  try {
    const settings = await ipcRenderer.invoke('get-settings') as Settings
    
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
backButton?.addEventListener('click', () => {
  ipcRenderer.send('navigate-to-main')
})

// Handle overlay mode changes
gridModeRadio?.addEventListener('change', () => {
  if (gridModeRadio.checked) {
    updateSetting('overlayMode', 'grid')
  }
})

labelModeRadio?.addEventListener('change', () => {
  if (labelModeRadio.checked) {
    updateSetting('overlayMode', 'label')
  }
})

// Handle animations checkbox
useAnimationsCheckbox?.addEventListener('change', () => {
  updateSetting('useAnimations', useAnimationsCheckbox.checked)
})

// Update a setting
async function updateSetting(key: keyof Settings, value: Settings[keyof Settings]): Promise<void> {
  try {
    await ipcRenderer.invoke('update-setting', { [key]: value })
  } catch (error) {
    console.error('Failed to update setting:', error)
  }
}

// Handle escape key to go back
document.addEventListener('keydown', (event: KeyboardEvent) => {
  if (event.key === 'Escape') {
    ipcRenderer.send('navigate-to-main')
  }
})