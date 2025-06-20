import { BrowserWindow } from 'electron'

let mainWindow: BrowserWindow | null = null
let isQuitting = false

export function createWindow(): void {
  mainWindow = new BrowserWindow({
    width: 600,
    height: 150,
    webPreferences: {
      nodeIntegration: true
    },
    frame: false,
    transparent: false,
    show: false, // Start hidden
    skipTaskbar: true // Don't show in taskbar when hidden
  })

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
      mainWindow?.close()
    }
  })
}

export function getMainWindow(): BrowserWindow | null {
  return mainWindow
}

export function setQuitting(quitting: boolean): void {
  isQuitting = quitting
}