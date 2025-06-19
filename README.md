# Typetap

## 🎯 Core Purpose

Typetap is a cross-platform, open-source tool designed to bridge the gap between keyboard and mouse interaction. Rather than replacing the mouse entirely, Typetap offers a fast, low-friction way to move and click the cursor using the keyboard — ideal for those brief moments where switching to the mouse feels like overkill.

Built for developers and keyboard-oriented users experiencing wrist fatigue or aiming to reduce repetitive strain, Typetap prioritizes simplicity over feature bloat. Its goal is to stay out of your way, activate instantly, and get you back to the keyboard with minimal disruption.

## Major Features

## 🖱️ Mouse Control

- **Basic Capabilities:**
  - Move the cursor to a location and optionally click.
  - Cancel movement/action with `Esc`.
- **Click Types via `Enter` Modifier:**
  - `Enter` → Left-click
  - `Shift + Enter` → Right-click
  - `Ctrl + Enter` → Double-click
  - `Alt + Enter` → Triple-click
  - `Alt + Arrow Down` → Scroll down at cursor position
  - `Alt + Arrow Up` → Scroll up at cursor position
- **Movement:**
  - Smooth or instant mouse motion, depending on preference.

---

## 🧠 Overlay System

Typetap supports **two modes** of screen interaction:

### 🟩 Grid Mode

- Divides the active screen into equal square regions.
- Each square is assigned a unique two-letter label (e.g. `"dy"`).
- Typing the label while holding `Shift`:
  - Moves the mouse to the selected square (no click).
  - Zooms in by dividing that square into a finer grid with updated labels.

> Supports recursive zooming for pixel-precise targeting.

---

### 🏷️ Label Mode

- Detects intractable elements using:
  - **OCR**, or
  - **System accessibility APIs**
- Assigns letter-based labels to each detected element.
- Typing the label moves the mouse to the element.
- Click type is determined by modifier + `Enter` (same as Grid mode).

---

## 📋 Command Palette

- **Purpose:** Acts as the entry point for navigation.
- **Functions:** Interprets typed labels or commands to drive overlay behaviour.
- **Limitations:**
  - No program launching
  - No system search
  - Strictly used to interpret overlay input

---

## ⚙️ Configurable Settings (v1)

Initial release will support two core options:

1. **Custom activation hotkey**
2. **Overlay mode preference** (`grid` or `label`)

> Configuration will be handled via a flat file (JSON/YAML). No GUI settings interface in v1.

---
