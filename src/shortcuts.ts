import { globalShortcut } from 'electron'
import { getMainWindow } from './window.js'

export function registerShortcuts(): void {
  // Register global hotkey
  globalShortcut.register('Shift+Command+Space', () => {
    const mainWindow = getMainWindow()
    if (mainWindow) {
      mainWindow.show()
    }
  })
}

export function unregisterShortcuts(): void {
  // Unregister all shortcuts
  globalShortcut.unregisterAll()
}