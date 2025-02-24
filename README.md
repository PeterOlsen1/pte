# THE PTE.

The PTE is a simple text editing program I made in rust, just as a hobby project.
It is by no means a *polished* text editor, and it is basically just the result of me toying around.

TODO:
* Undos
  * Keep an enum present so we can see what action was done? maybe do more / less based on that enum
  * Shift cursor position when undo happens
* Figure out suitable keybinds (or at least some that suffice) (especially for mac)
* Selection (SHIFT + DIRECTION would be nice, CTRL + SHIFT for line select in that direction?) (didn't work. ctrl + shift modifiers weird?)
  * When selected, hit right to put cursor at end, same with left
* Copy
* Smart file opening?
* Delete tabs
* More editor tabs?
* run terminal commands (extra fluff)