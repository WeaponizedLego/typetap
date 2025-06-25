// import { screen } from 'electron'
// // import robot from 'robotjs'

// export function moveCursorTo(x: number, y: number, smooth: boolean = true): void {
//   try {
//     if (smooth) {
//       // Get current mouse position
//       const currentPos = robot.getMousePos()
//       const steps = 20
//       const stepX = (x - currentPos.x) / steps
//       const stepY = (y - currentPos.y) / steps

//       // Animate movement over several steps
//       for (let i = 1; i <= steps; i++) {
//         setTimeout(() => {
//           const newX = Math.round(currentPos.x + stepX * i)
//           const newY = Math.round(currentPos.y + stepY * i)
//           robot.moveMouse(newX, newY)
//         }, i * 5) // 5ms between each step
//       }
//     } else {
//       robot.moveMouse(x, y)
//     }
//   } catch (error) {
//     console.error('Failed to move cursor:', error)
//   }
// }

// export function clickAt(x: number, y: number, button: 'left' | 'right' = 'left', double: boolean = false): void {
//   try {
//     robot.moveMouse(x, y)

//     if (double) {
//       robot.mouseClick(button, true) // double click
//     } else {
//       robot.mouseClick(button, false) // single click
//     }
//   } catch (error) {
//     console.error('Failed to click:', error)
//   }
// }

// export function scrollAt(x: number, y: number, direction: 'up' | 'down', clicks: number = 3): void {
//   try {
//     robot.moveMouse(x, y)
//     robot.scrollMouse(1, direction === 'up' ? 'up' : 'down')
//   } catch (error) {
//     console.error('Failed to scroll:', error)
//   }
// }
