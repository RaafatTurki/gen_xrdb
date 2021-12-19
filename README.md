# GENXRDB
a tool that generates xresources from toml


#### input.toml
```toml
header = "! vim: filetype=xdefaults:commentstring=!%s"
[defs]
BL          = "#000000"
WH          = "#0A151F"
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

#### output.xrdb
```
! vim: filetype=xdefaults:commentstring=!%s

#define BL #000000
#define WH #0A151F

Xft.dpi: DPI

col.bl: BL
col.wh: WH

*.background: #0D1017
*.color0: #252550
*.color1: #D7263D
*.color2: #B0DB43
*.color3: #FFAE03
*.foreground: #F1F7ED
```
