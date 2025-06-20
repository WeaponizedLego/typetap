import { Tray, Menu, nativeImage, app } from 'electron'
import { setQuitting, getMainWindow } from './window.js'

let tray: Tray | null = null

export function createTray(): void {
  // Create tray icon
  tray = new Tray(nativeImage.createFromPath('src/assets/tray-icon.png'))
  tray.setToolTip('TypeTap')
  tray.setContextMenu(
    Menu.buildFromTemplate([
      {
        label: 'Settings',
        click: () => {
          const mainWindow = getMainWindow()
          if (mainWindow) {
            mainWindow.loadFile('assets/windows/settings.html')
            mainWindow.show()
          }
        }
      },
      {
        type: 'separator'
      },
      {
        label: 'Quit',
        click: () => {
          setQuitting(true)
          app.quit()
        }
      }
    ])
  )
}

export function getTray(): Tray | null {
  return tray
}