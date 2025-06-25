const { ipcRenderer } = require('electron')

let gridData = []
let currentInput = ''
let gridContainer = null

// Initialize when DOM is ready
document.addEventListener('DOMContentLoaded', () => {
  gridContainer = document.getElementById('grid-container')

  // Listen for grid data from main process
  ipcRenderer.on('initialize-grid', (event, cells) => {
    gridData = cells
    createGridCells()
    animateGridIn()
  })

  // Listen for keyboard input
  document.addEventListener('keydown', handleKeyDown)
})

function createGridCells() {
  // Clear existing cells
  gridContainer.innerHTML = ''

  gridData.forEach((cell, index) => {
    const cellElement = document.createElement('div')
    cellElement.className = 'grid-cell'
    cellElement.style.left = `${cell.x}px`
    cellElement.style.top = `${cell.y}px`
    cellElement.style.width = `${cell.width}px`
    cellElement.style.height = `${cell.height}px`
    cellElement.dataset.label = cell.label
    cellElement.dataset.index = index

    const labelElement = document.createElement('div')
    labelElement.className = 'grid-label'
    labelElement.textContent = cell.label

    cellElement.appendChild(labelElement)
    gridContainer.appendChild(cellElement)
  })
}

function animateGridIn() {
  const cells = document.querySelectorAll('.grid-cell')

  // Calculate stagger delay based on distance from center
  const centerX = window.innerWidth / 2
  const centerY = window.innerHeight / 2

  const cellsWithDistance = Array.from(cells).map((cell, index) => {
    const rect = cell.getBoundingClientRect()
    const cellCenterX = rect.left + rect.width / 2
    const cellCenterY = rect.top + rect.height / 2
    const distance = Math.sqrt(
      Math.pow(cellCenterX - centerX, 2) + Math.pow(cellCenterY - centerY, 2)
    )
    return { cell, distance, index }
  })

  // Sort by distance (closest first)
  cellsWithDistance.sort((a, b) => a.distance - b.distance)

  // Animate cells in staggered fashion
  cellsWithDistance.forEach(({ cell }, index) => {
    setTimeout(() => {
      cell.classList.add('animate-in')
    }, index * 0.5) // 2ms stagger
  })
}

function handleKeyDown(event) {
  // Handle escape key
  if (event.key === 'Escape') {
    ipcRenderer.send('close-grid')
    return
  }

  // Handle backspace
  if (event.key === 'Backspace') {
    currentInput = currentInput.slice(0, -1)
    updateCellHighlights()
    return
  }

  // Handle letter input
  if (event.key.length === 1 && /[a-zA-Z]/.test(event.key)) {
    currentInput = (currentInput + event.key.toLowerCase()).slice(0, 2)
    updateCellHighlights()

    // Check for exact match
    if (currentInput.length === 2) {
      const matchingCell = gridData.find((cell) => cell.label === currentInput)
      if (matchingCell) {
        selectCell(matchingCell)
      }
    }
  }
}

function updateCellHighlights() {
  const cells = document.querySelectorAll('.grid-cell')

  cells.forEach((cell) => {
    const label = cell.dataset.label
    cell.classList.remove('highlight', 'partial-match')

    if (currentInput.length === 0) {
      return
    }

    if (label === currentInput) {
      cell.classList.add('highlight')
    } else if (label.startsWith(currentInput)) {
      cell.classList.add('partial-match')
    }
  })
}

function selectCell(cell) {
  // Calculate cursor position (center of cell)
  const cursorX = cell.x + cell.width / 2
  const cursorY = cell.y + cell.height / 2

  // Send selection to main process
  ipcRenderer.send('grid-cell-selected', {
    label: cell.label,
    x: cursorX,
    y: cursorY
  })

  // Animate cell selection
  const cellElement = document.querySelector(`[data-label="${cell.label}"]`)
  if (cellElement) {
    cellElement.style.transform = 'scale(1.2)'
    cellElement.style.background = 'rgba(76, 175, 80, 0.8)'

    setTimeout(() => {
      ipcRenderer.send('close-grid')
    }, 200)
  }
}
