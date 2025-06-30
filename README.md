
## Desktop File Organizer

A simple Rust tool to automatically organize files and folders on your Desktop into a structured `archives/` directory by date and file type.
### This program does not categorize files by its type, but rather organize file by its extensions.
This tool helps declutter your Desktop by:

- Moving **all top-level files** into `archives/yyyyMM/<file_extension>/`
- Organizing files based on their **last modified date** and **file extension**
---
https://github.com/user-attachments/assets/5a332e4a-a05f-48ca-a3c0-cfa7633b1c2d
---
Tested on: Windows 11.
```bash
# If using cargo
cargo run --release
# Or
run the executable from anywhere.
```
## Example
```
Before:
Desktop/
  notes.txt
  photo.jpg
  
After:
Desktop/
  archives/
    202506/
      txt/
        notes.txt
      jpg/
        photo.jpg
```




