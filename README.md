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
| Global hotkey to trigger it | 🚧 next |
| Windows support | ⏳ later |

So right now `cargo run` pops a see-through **overlay** with a hint chip on
every clickable element. Type a hint and it clicks. A global hotkey (so you
trigger it from any app without a terminal) is next.

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
terminal app (Terminal, iTerm, etc.), then run `cargo run` again. Now it lists
the clickable things in whatever window was frontmost:

A translucent overlay covers the screen with a yellow hint chip on each
clickable element. The highest "click-intent" elements (buttons, links) get the
easiest single keys; plain text gets harder or two-key hints.

**Then:**
- Type a hint → **left-click** that element.
- **Shift** + hint → right-click.
- **Option** (⌥) + hint → just move the pointer there, no click.
- **Esc** → cancel, click nothing.

As you type, chips that can't match disappear, so you always see what's left.

> ⚠️ It really moves your mouse and clicks. Run from a terminal, the scanned
> "frontmost window" is the terminal itself. Bound to a hotkey later, it'll
> target whatever app you were actually using.

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

## When the full app exists (preview of how it'll work)

> Not built yet — here so you know where this is going.

1. Press a global hotkey from anywhere.
2. typetap labels every clickable thing in the front window with a hint.
3. Type the hint → it clicks.
   - plain hint = left click
   - a modifier + hint = right click
   - a modifier + hint = just move the pointer there (no click)
4. **macOS will require granting Accessibility permission** the first time:
   System Settings → Privacy & Security → Accessibility → enable typetap.
   (Nothing can read or click other apps' windows without this — it's a macOS
   rule, not optional.)

---

## Troubleshooting

- **`cargo: command not found`** — you skipped step 1, or need to reopen
  Terminal. Try `source "$HOME/.cargo/env"`.
- **`cargo --version` shows 1.71** — old Rust is first in your PATH. Use
  `~/.cargo/bin/cargo` everywhere instead of `cargo`.
- **Compile takes forever the first time** — normal; it's downloading and
  building dependencies once. Subsequent builds are cached.
