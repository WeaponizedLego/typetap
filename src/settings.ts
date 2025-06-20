import { app } from 'electron'
import * as fs from 'fs'
import * as path from 'path'

export interface Settings {
  overlayMode: 'grid' | 'label'
  useAnimations: boolean
}

const defaultSettings: Settings = {
  overlayMode: 'grid',
  useAnimations: true
}

let currentSettings: Settings = { ...defaultSettings }

function getSettingsPath(): string {
  return path.join(app.getPath('userData'), 'settings.json')
}

export function loadSettings(): Settings {
  try {
    const settingsPath = getSettingsPath()
    if (fs.existsSync(settingsPath)) {
      const data = fs.readFileSync(settingsPath, 'utf8')
      const parsed = JSON.parse(data) as Settings
      currentSettings = { ...defaultSettings, ...parsed }
    }
  } catch (error) {
    console.error('Failed to load settings:', error)
    currentSettings = { ...defaultSettings }
  }
  return currentSettings
}

export function saveSettings(settings: Settings): void {
  try {
    const settingsPath = getSettingsPath()
    const settingsDir = path.dirname(settingsPath)
    
    // Ensure the directory exists
    if (!fs.existsSync(settingsDir)) {
      fs.mkdirSync(settingsDir, { recursive: true })
    }
    
    fs.writeFileSync(settingsPath, JSON.stringify(settings, null, 2))
    currentSettings = { ...settings }
  } catch (error) {
    console.error('Failed to save settings:', error)
  }
}

export function getSettings(): Settings {
  return { ...currentSettings }
}

export function updateSettings(updates: Partial<Settings>): Settings {
  const newSettings = { ...currentSettings, ...updates }
  saveSettings(newSettings)
  return newSettings
}