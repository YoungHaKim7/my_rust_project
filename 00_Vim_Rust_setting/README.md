# Unicode Vim

- Ex commands:

```
:UnicodeTable    - Print Unicode Table in new window
:Digraphs        - Search for specific digraph char
:UnicodeSearch   - Search for specific unicode char
:UnicodeSearch!  - Search for specific unicode char (and add at current cursor position)
:UnicodeName     - Identify character under cursor (like ga command)
:DownloadUnicode - Download (or update) Unicode data
:UnicodeCache    - Create cache file

```
<hr>

<br>

- Normal mode commands:
```
<C-X><C-G>  - Complete Digraph char
<C-X><C-Z>  - Complete Unicode char
<F4>        - Combine characters into digraphs

```

<hr>

<br>

- Scripting Functions:

```
unicode#FindUnicodeBy() - Find unicode characters
unicode#FindDigraphBy() - Find Digraph char
unicode#Digraph()       - Returns digraph char
unicode#UnicodeName()   - Identifies unicode character (by value)
```

https://github.com/chrisbra/unicode.vim

<hr>

<br>

<hr>

# AstroVim(AstroNeoVim)

https://astronvim.github.io/

https://github.com/AstroNvim/AstroNvim

## Requirements

- Nerd Fonts (Optional with manual intervention: See Recipes/Customizing Icons)

- Neovim 0.8 (Not including nightly)

- Tree-sitter CLI (Note: This is only necessary if you want to use auto_install feature with Treesitter)

- A clipboard tool is necessary for the integration with the system clipboard (see :help clipboard-tool for supported solutions)

- Terminal with true color support (for the default theme, otherwise it is dependent on the theme you are using)

- Optional Requirements:
  - ripgrep - live grep telescope search (<leader>fw)
  - lazygit - git ui toggle terminal (<leader>tl or <leader>gg)
  - go DiskUsage() - disk usage toggle terminal (<leader>tu)
  - bottom - process viewer toggle terminal (<leader>tt)
  - Python - python repl toggle terminal (<leader>tp)
  - Node - node repl toggle terminal (<leader>tn)

<br>

# LunarVim(LunarNeoVim)

- 🌙 LunarVim is an IDE layer for Neovim. Completely free and community driven.

https://www.lunarvim.org/

https://github.com/LunarVim/LunarVim

<hr>

# Rust Vim Setting

<br>

https://github.com/YoungHaKim7/rust_vim_setting

<br>

<table border="1">
    <tr>
    <td colspan="3" align="center">Rust Vim & gVim & LunarVim Setting</td>
    </tr>
    <tr align="center">
        <td>OS </td>
        <td>Title</td>
        <td>Link</td>
    </tr>
    <tr align="center">
        <td rowspan="2">macOS</td></a>
        <td>LunarVim Setting</td>
        <td><a href="https://github.com/YoungHaKim7/rust_vim_setting/tree/main/LunarVim_Rust_setting">https://github.com/YoungHaKim7/rust_vim_setting/LunarVim_Rust_setting</a></td>
    </tr>
    <tr align="center">
        <td>Vim Setting</td>
        <td><a href="https://github.com/YoungHaKim7/rust_vim_setting/tree/main/Vim_Rust_macOS_setting">Vim_Setting</a></td>
    </tr>
    <tr align="center">
        <td rowspan="2">WindowsOS</td></a>
        <td>gVim</td>
        <td><a href="https://github.com/YoungHaKim7/rust_vim_setting/tree/main/gVim_Win11_OS_Setting">gVim_Setting_Win11</a></td>
    </tr>
    <tr align="center">
        <td>nVim</td></a>
        <td></td>
    </tr>
</table>

# WindowsOS Setting

- part1

WindowsOS*윈도우*개발자세팅\_파워셀\_Vim_Rust_VSCode_VS_WindowsSetting_Windows_Terminal_PowerShell #settings

https://youtu.be/TQQbT9W_M0A

<br>

- part2

WindowsOS*윈도우*개발자세팅part2_NeoVim & Vim_Rust_VS_WindowsSetting_Windows_Terminal_PowerShell #settings

https://youtu.be/ClChV_1SAho

- part3

WindowsOS\_윈도우\_Vim개발자세팅part3_surround_vim #surround #settings

https://youtu.be/hgeHORtxod0

<br>

<hr>

# macOS Setting

<br>

- 한글러스트Rust강의\_048⭐️Rust개발환경Vim세팅하기\_Vim_macOS_M1_pro #vim #CocInlayHint #rustinlayhint

https://youtu.be/gIUOkdMjo8o

<br>

- 한글러스트Rust강의\_049⭐️Rust개발환경LunarVim세팅하기\_LunarVim_macOS_M1_pro #lunarvim #CocInlayHint #rustinlayhint

https://youtu.be/c8FX89jf4To

<hr>
