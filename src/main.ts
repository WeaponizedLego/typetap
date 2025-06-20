import {
  app,
  BrowserWindow,
  Tray,
  globalShortcut,
  Menu,
  nativeImage
} from 'electron'

let mainWindow: BrowserWindow | null = null
let tray: Tray | null = null
let isQuitting = false

function createWindow() {
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

app.on('ready', () => {
  createWindow()

  // Create tray icon
  tray = new Tray(nativeImage.createFromPath('src/assets/tray-icon.png'))
  tray.setToolTip('TypeTap')
  tray.setContextMenu(
    Menu.buildFromTemplate([
      {
        label: 'Quit',
        click: () => {
          isQuitting = true
          app.quit()
        }
      }
    ])
  )

  // Register global hotkey
  globalShortcut.register('Shift+Command+Space', () => {
    if (mainWindow) {
      mainWindow.show()
    }
  })
})

app.on('will-quit', () => {
  // Unregister all shortcuts
  globalShortcut.unregisterAll()
})

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit()
  }
})

app.on('activate', () => {
  if (mainWindow === null) {
    createWindow()
  }
})
