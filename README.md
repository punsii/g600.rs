# Logitech G600 Mapping on Linux with X11 (and maybe Wayland)
This is a rust rewrite of [g600.c](https://github.com/mafik/logitech-g600-linux/tree/master), packaged with [nix](https://nixos.org/).
It only supports basic button remapping for now, but support for other utilities could be added fairly easily.
Only tested on nixos with X11. Other linux distros using X11 _should_ work out of the box if this project is installed with nix.


Supports 16 keys and the G-shift button for a total of 32 fast shortcuts.

Before running this program open the Logitech Gaming Software on a Windows or Mac OS machine. Assign the three basic mouse buttons to their standard functions. The G-shift button should be assigned to the G-shift function. All the remaining buttons (scroll left, scroll right, G7, ... G20) should be set to emulate (unique) keyboard keys (but not modifier keys).

# Usage
1. Clone this repository.
2. Open `src/main.rs` and fill in the commands for the keys.
3. Run with `nix run` or build with `nix build`.
Alternatively to `nix run` you can try building with `cargo build`, but in that case you will have to figure out how to provide required libs (like libxdo).
