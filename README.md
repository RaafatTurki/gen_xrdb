# GENXRDB
a tool that generates xresources from toml

## Installation
###### From Source
clone and `cargo build`
###### From AUR
`paru -S genxrdb`

## Usage
`genxrdb ./input.toml`  
prints the generated xresources equivalent to stdout

###### header.txt
```
! vim: filetype=xdefaults:commentstring=!%s
```

###### footer.txt
```
! fin
```

###### input.toml
```toml
header = "./header"
footer = "./footer"

[defs]
BL          = "#000000"
WH          = "#0A151F"
DPI         = 97

[mods]
Xft.dpi     = "DPI"

[mods.col]
bl          = "BL"
wh          = "WH"

[mods.ALL]
background  = "#0D1017"
foreground  = "#F1F7ED"
color0      = "#252550"
color1      = "#D7263D"
color2      = "#B0DB43"
color3      = "#FFAE03"
```

###### output
```
! vim: filetype=xdefaults:commentstring=!%s

#define BL #000000
#define WH #0A151F
#define DPI 97

Xft.dpi: DPI

col.bl: BL
col.wh: WH

*.background: #0D1017
*.color0: #252550
*.color1: #D7263D
*.color2: #B0DB43
*.color3: #FFAE03
*.foreground: #F1F7ED

! fin
```
