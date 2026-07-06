# typetap

Click anything on screen using only the keyboard — like Shortcat, but no
telemetry, and the hint letters are chosen so the **easiest keys land on the
things you're most likely to click**.

macOS first; Windows planned later (the smart-hint logic is already
OS-independent).

---

## Status (honest)

| Part | State |
|------|-------|
| Smart hint-letter assignment | ✅ done + tested |
| Find clickable elements on screen (macOS Accessibility) | ✅ working |
| Perform the click / right-click / move | ✅ working |
| On-screen hint overlay | ✅ working |
| Grid mode (click anywhere) | ✅ working |
| Global hotkey to trigger it | ✅ working |
| Windows support | ⏳ later |

`cargo run` now launches typetap as a **background daemon**. Leave it running
and press **⇧⌘Space** (Shift+Cmd+Space) from *any* app — a see-through overlay
drops a hint chip on every clickable element. Type a hint and it clicks, then
the overlay hides again, ready for the next press. (On Windows the hotkey is
Shift+Alt+Space.)

---

## 1. Install Rust (one time)

You need a recent Rust. Paste this into Terminal and press Enter:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
```

Then **close and reopen Terminal** (or run `source "$HOME/.cargo/env"`) so the
`cargo` command is found.

Check it worked:

```sh
cargo --version
```

You should see `cargo 1.9x` or higher. If you see `1.71`, an older Rust is
shadowing the new one — use the full path `~/.cargo/bin/cargo` instead of
`cargo` in the commands below.

---

## 2. Get the code

```sh
cd ~/_code/personal/typetap
```

(That's this folder. If you cloned it elsewhere, `cd` there instead.)

---

## 3. Run the demo

```sh
cargo run
```

The first run downloads dependencies and compiles — that takes a minute or two.
Later runs are fast.

**The first time, macOS will block it** and you'll see:

```
typetap needs Accessibility permission.
System Settings -> Privacy & Security -> Accessibility -> enable your terminal,
then run again.
```

Open **System Settings → Privacy & Security → Accessibility**, switch on your
terminal app (Terminal, iTerm, etc.), then run `cargo run` again. It stays
running and prints `typetap ready`. Switch to any app, press **⇧⌘Space**, and a
translucent overlay covers that app's display with a yellow hint chip on each
clickable element. The highest "click-intent" elements (buttons, links) get the
easiest single keys; plain text gets harder or two-key hints.

**Then:**
- Type a hint → **left-click** that element.
- **Shift** + hint → right-click.
- **Option** (⌥) + hint → just move the pointer there, no click.
- **Esc** → cancel, click nothing.

As you type, chips that can't match disappear, so you always see what's left.

### Grid mode — click *anywhere*

Some apps (games, canvas/Electron apps, remote desktops, or a plain terminal)
expose no clickable elements for the overlay to label. Grid mode covers the
whole screen with a labeled grid instead, so you can aim a click at any point:

```sh
cargo run -- grid
```

Now ⇧⌘Space covers the display under your cursor with a labeled grid. Every cell
has a two-key hint — the first key picks the row, the second the column, both in
the same home-row-first order. Type the two keys to click that cell's center
(Shift = right-click, ⌥ = move only, Esc = cancel, same as above).

> ⚠️ It really moves your mouse and clicks — on whatever app was frontmost when
> you pressed the hotkey.

---

## 4. Run the tests

```sh
cargo test
```

All tests should say `ok`. These prove the hint letters are always unambiguous
and that the best keys go to the most important elements.

---

## How the smart hints work

1. Each on-screen element gets a **priority** — a button or link scores higher
   than plain text, because you're more likely to want to click it.
2. Keys are ranked by how easy they are to type (home-row index/middle fingers
   first: `f j d k s l a g h …`, awkward pinky/bottom keys last).
3. The easiest keys are handed to the highest-priority elements. If there are
   more elements than keys, the least-important ones get two-key combos like
   `qf` instead of a single key.

All of this lives in `src/hints.rs` and has no macOS code in it, so the same
logic will drive the future Windows version unchanged.

---

## Still rough / next up

- **Runs from a terminal, not as a bundled app.** So the Accessibility
  permission you grant is your *terminal's*. A proper `.app` bundle (and a
  menu-bar icon instead of a live terminal) comes later.
- **Element mode vs grid mode is chosen at launch** (`cargo run` vs
  `cargo run -- grid`), not switchable at runtime yet — a second hotkey to flip
  between them would be the natural next step.
- **Windows support** is stubbed: the hint logic, grid, and clicker are already
  OS-independent; only `macos.rs` (element scanning) needs a Windows sibling.

---

## Troubleshooting

- **`cargo: command not found`** — you skipped step 1, or need to reopen
  Terminal. Try `source "$HOME/.cargo/env"`.
- **`cargo --version` shows 1.71** — old Rust is first in your PATH. Use
  `~/.cargo/bin/cargo` everywhere instead of `cargo`.
- **Compile takes forever the first time** — normal; it's downloading and
  building dependencies once. Subsequent builds are cached.
