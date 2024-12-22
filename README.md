```
d8888b. db   db d88888b db    db d88888b d8888b.
88  `8D 88   88 88'     `8b  d8' 88'     88  `8D
88oobY' 88ooo88 88ooooo  `8bd8'  88ooooo 88   88
88`8b   88~~~88 88~~~~~  .dPYb.  88~~~~~ 88   88
88 `88. 88   88 88.     .8P  Y8. 88.     88  .8D
88   YD YP   YP Y88888P YP    YP Y88888P Y8888D'
```


# rHexeD - a WIP hex file editor
At the same time, I have the (almost) same project but in [C](https://github.com/LittleB0xes/hexed)

rHexeD is a small hex editor using the awesome [Crossterm](https://github.com/crossterm-rs/crossterm) as Terminal Library.



![rhexed](http://url/to/img.pn](https://github.com/LittleB0xes/rhexed/blob/main/screenshots/screenshot_1.png)

## Usage
You can work on one file
> ./rhexed my_file

or, if you need, on several files in the same time, with the ability to navigate from file to file.
> ./rhexed my_file_1 my_file_2 my_file_3 ...


## Command
Some commands are available, and others will come later

```
- hjkl or arrow     move 
- g                 move to the beginning of the file
- G                 move to the end of the file
- (                 move to the beginning of the line
- )                 move to the end of the line
- [                 move to the beginning of the page
- ]                 move to the end of the page
- n                 go to the next page
- b                 go to the previous page
- N                 go to the next file
- B                 go to the previous file
- J                 go to a specified address
- a                 insert a byte after cursor position
- x                 cut a byte
- y                 copy a byte or a range of selected bytes
- p                 paste a byte or a range of selected bytes
- i                 insert mode
- <ESC>             quit insert mode
- <TAB>             show / hide title
- r                 reload file
- w                 write file
- q                 quit
```

