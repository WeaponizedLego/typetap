import { BrowserWindow, screen, ipcMain } from 'electron'
import { getMainWindow } from './window.js'

let gridWindow: BrowserWindow | null = null

export interface GridCell {
  label: string
  x: number
  y: number
  width: number
  height: number
}

export function createGridOverlay(): void {
  if (gridWindow) {
    gridWindow.close()
  }

  const display = screen.getPrimaryDisplay()
  const { width, height } = display.workAreaSize

  gridWindow = new BrowserWindow({
    width,
    height,
    x: 0,
    y: 0,
    webPreferences: {
      nodeIntegration: true,
      contextIsolation: false
    },
    frame: false,
    transparent: true,
    alwaysOnTop: true,
    skipTaskbar: true,
    resizable: false,
    movable: false,
    minimizable: false,
    maximizable: false,
    closable: true,
    focusable: true,
    show: false // Start hidden, will show after setup
  })

  // Load the grid HTML
  gridWindow.loadFile('assets/windows/grid.html')

  // Set up IPC handlers for grid interaction
  gridWindow.webContents.once('dom-ready', () => {
    // Generate grid data and send to renderer
    const gridData = generateGridData(width, height)
    gridWindow?.webContents.send('initialize-grid', gridData)

    // Show window and start animation
    gridWindow?.show()
  })

  // Handle escape key to close grid
  gridWindow.webContents.on('before-input-event', (event, input) => {
    if (input.key === 'Escape') {
      closeGridOverlay()
    }
  })

  gridWindow.on('closed', () => {
    gridWindow = null
  })
}

// Initialize IPC handlers
export function initializeGridIPC(): void {
  ipcMain.on('close-grid', () => {
    closeGridOverlay()
  })

  createGridOverlay()

  ipcMain.on('open-grid', () => {
    createGridOverlay()
  })

  ipcMain.on('hide-main-window', () => {
    const mainWindow = getMainWindow()
    if (mainWindow) {
      mainWindow.hide()
    }
  })

  ipcMain.on('grid-cell-selected', (event, data) => {
    console.log(`Grid cell selected: ${data.label} at (${data.x}, ${data.y})`)
    // TODO: Implement mouse movement and clicking
    // For now, just close the grid
    closeGridOverlay()
  })
}

export function closeGridOverlay(): void {
  if (gridWindow) {
    gridWindow.close()
    gridWindow = null
  }
}

export function getGridWindow(): BrowserWindow | null {
  return gridWindow
}

function generateGridData(
  screenWidth: number,
  screenHeight: number
): GridCell[] {
  // Calculate optimal grid size - aim for roughly square cells
  // Start with a reasonable cell size (around 100-150px)
  const targetCellSize = 120
  const cols = Math.ceil(screenWidth / targetCellSize)
  const rows = Math.ceil(screenHeight / targetCellSize)

  const cellWidth = screenWidth / cols
  const cellHeight = screenHeight / rows

  const cells: GridCell[] = []
  const letters = 'abcdefghijklmnopqrstuvwxyz'

  let labelIndex = 0
  for (let row = 0; row < rows; row++) {
    for (let col = 0; col < cols; col++) {
      // Generate two-letter label
      const firstLetter = letters[Math.floor(labelIndex / 26) % 26]
      const secondLetter = letters[labelIndex % 26]
      const label = firstLetter + secondLetter

      cells.push({
        label,
        x: col * cellWidth,
        y: row * cellHeight,
        width: cellWidth,
        height: cellHeight
      })

      labelIndex++
    }
  }

  return cells
}
