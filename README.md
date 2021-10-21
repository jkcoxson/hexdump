# hexdump
An alternative to hexdump specifically for comparing two files

This program can print out the hex of a file just like vanilla hex dump, but it can also compare the hex between two files. 

### What it actually does
It starts by going up the file, seeing what is the same and stops when it finds a different byte.
It then goes down the file and stops at different bytes again.
It then colors the bytes in a nice pretty way to see what is different.

You can append ``--compact`` or ``-c`` to add the matching numbers, specifically for analyzing large files.

### Example

```abcdefghij``` dumps to ```61 62 63 64 65 66 67 68 69 6A 0A```

```abc99ghij``` dumps to ```61 62 63 39 39 67 68 69 6A 0A```
When you compare them both you get
```
61 62 63 | 64 65 66 | 67 68 69 6A 0A
61 62 63 | 39 39 67 | 68 69 6A 0A
```
*Note: it is color coded instead of seperated by |, but colors aren't a thing on GitHub*
