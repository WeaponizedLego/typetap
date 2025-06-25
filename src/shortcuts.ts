import { globalShortcut } from 'electron'
import { getMainWindow } from './window.js'
import { createGridOverlay } from './grid.js'

export function registerShortcuts(): void {
  // Register global hotkey for main window
  globalShortcut.register('Shift+Command+Space', () => {
    const mainWindow = getMainWindow()
    if (mainWindow) {
      mainWindow.show()
    }
  })

  // Register global hotkey for grid mode
  globalShortcut.register('Shift+Command+G', () => {
    createGridOverlay()
  })
}

export function unregisterShortcuts(): void {
  // Unregister all shortcuts
  globalShortcut.unregisterAll()
}
