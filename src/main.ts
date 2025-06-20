import { app } from 'electron'
import { createWindow, getMainWindow } from './window.js'
import { createTray } from './tray.js'
import { registerShortcuts, unregisterShortcuts } from './shortcuts.js'

app.on('ready', () => {
  createWindow()
  createTray()
  registerShortcuts()
})

app.on('will-quit', () => {
  unregisterShortcuts()
})

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit()
  }
})

app.on('activate', () => {
  const mainWindow = getMainWindow()
  if (mainWindow === null) {
    createWindow()
  }
})
