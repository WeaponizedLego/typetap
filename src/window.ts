import { BrowserWindow, ipcMain } from 'electron'
import { loadSettings, getSettings, updateSettings } from './settings.js'

let mainWindow: BrowserWindow | null = null
let isQuitting = false

export function createWindow(): void {
  mainWindow = new BrowserWindow({
    width: 600,
    height: 200,
    webPreferences: {
      nodeIntegration: true,
      contextIsolation: false
    },
    frame: false,
    transparent: false,
    show: false, // Start hidden
    skipTaskbar: true // Don't show in taskbar when hidden
  })

  // Load settings when window is created
  loadSettings()

  mainWindow.loadFile('assets/windows/index.html')

  // Hide window instead of closing it
  mainWindow.on('close', (event) => {
    if (!isQuitting) {
      event.preventDefault()
      mainWindow?.hide()
    }
  })

  mainWindow.on('closed', () => {
    mainWindow = null
  })

  mainWindow?.webContents.on('before-input-event', (event, input) => {
    if (input.key === 'Escape') {
      mainWindow?.hide()
    }
  })

  // Setup IPC handlers
  setupIpcHandlers()
}

function setupIpcHandlers(): void {
  // Handle navigation between views
  ipcMain.on('navigate-to-settings', () => {
    mainWindow?.loadFile('assets/windows/settings.html')
  })

  ipcMain.on('navigate-to-main', () => {
    mainWindow?.loadFile('assets/windows/index.html')
  })

  ipcMain.on('hide-window', () => {
    mainWindow?.hide()
  })

  // Handle settings operations
  ipcMain.handle('get-settings', () => {
    return getSettings()
  })

  ipcMain.handle('update-setting', (event, updates) => {
    return updateSettings(updates)
  })
}

export function getMainWindow(): BrowserWindow | null {
  return mainWindow
}

export function setQuitting(quitting: boolean): void {
  isQuitting = quitting
}