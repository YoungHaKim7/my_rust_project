# Rust Doc std(standard Library DOC)

https://doc.rust-lang.org/std/

<br>

##  DOCS.RS(rust 기타 crate문서들)

https://docs.rs/

<br>

<hr>

# Learn Rust With Entirely Too Many Linked Lists

https://rust-unofficial.github.io/too-many-lists/index.html

<br>

<hr>

# LunarVim InLayHint _____Comment Color

```
// 어두운 빨간색
: hi Comment guifg=#cfe2f3 guibg=#c90076


// 밝은 분홍색 느낌
: hi Comment guifg=#cfe2f3 guibg=#ff439f

```

- color-hex

https://www.color-hex.com/color/ff439f

<br>

## LunarVim Hover 

Control + Space

<br>

```
Shift + K 하면 이상한 문서가 나옴

Control + Space 해야 내가 생각하는 Hover 가 나온다. 

rust-tools.lua 파일 참조 

```

<hr>

-  한글러스트Rust강의_049⭐️Rust개발환경LunarVim세팅하기_LunarVim_macOS_M1_pro #lunarvim #CocInlayHint #rustinlayhint

https://youtu.be/c8FX89jf4To

<br>

<hr>


# Vim Setting (type 빨강색으로 강조하기 칙칙한 검은색 너무 싫다.)

-vim 에서

```
:hi CocInlayHint ctermbg=125

5 밝다
52 어둡다


```

- 내가 원하는 색깔 256 컬러에서 고르자 ㅎㅎ

https://www.ditig.com/256-colors-cheat-sheet

<br>

-  한글러스트Rust강의_048⭐️Rust개발환경Vim세팅하기_Vim_macOS_M1_pro #vim #CocInlayHint #rustinlayhint

https://youtu.be/gIUOkdMjo8o

<br>

<hr>

# vim documentation scroll(Vim key map)

- CTRL-F (PageDown)

- CTRL-D (PageUp)

```
							*CTRL-E*
CTRL-E			Scroll window [count] lines downwards in the buffer.
			Mnemonic: Extra lines.


							*CTRL-D*
CTRL-D			Scroll window Downwards in the buffer.  The number of
			lines comes from the 'scroll' option (default: half a
			screen).  If [count] given, first set 'scroll' option
			to [count].  The cursor is moved the same number of
			lines down in the file (if possible; when lines wrap
			and when hitting the end of the file there may be a
			difference).  When the cursor is on the last line of
			the buffer nothing happens and a beep is produced.
			See also 'startofline' option.
			{difference from vi: Vim scrolls 'scroll' screen
			lines, instead of file lines; makes a difference when
			lines wrap}

<S-Down>	or				*<S-Down>* *<kPageDown>*

<PageDown>	or				*<PageDown>* *CTRL-F*
CTRL-F			Scroll window [count] pages Forwards (downwards) in
			the buffer.  See also 'startofline' option.
			When there is only one window the 'window' option
			might be used.
```

<br>

https://vimdoc.sourceforge.net/htmldoc/scroll.html#scroll-down

<br>

<hr>

# Vim Command

```
:CocCommand

// rust-analyzer 다시 시작
FUZZY > rust-analyzer.reload

// rust-analyzer upgrade
FUZZY > rust-analyzer.upgrade

:CocOpenLog
error log 보기

:CocConfig
VSCode Setting.JSON 과 비슷
```

<br>

# Vim CocInstall (rust-analyzer)

https://github.com/fannheyward/coc-rust-analyzer#highlight-group

```
:CocInstall coc-rust-analyzer


remove rust-analyzer config from coc-settings.json if you've set

NOTE: For Apple Silicon users, you shouldn't use Node.js v15, checkout #975 for more.


// 이렇게 하면 coc-settings.JSON 에 들어간다.
:CocConfig

```

https://rust-analyzer.github.io/manual.html#vimneovim

<br>

# Vim 창 나누기

```
// 창 좌우로 나누기
:vs


// 창 상하로 나누기
:sp


// 가운데 선 아래(Down)으로 이동 (:sp에서 주로 사용)
:ObviousResizeDown

// 가운데 선 위(Up)로 이동 (:sp에서 주로 사용)
:ObviousResizeUp

// 가운데 선 오른쪽(Right)으로 이동(:vs에서 주로 사용)
:ObviousResizeRight

// 가운데 선 왼쪽(Left)으로 이동(:vs에서 주로 사용)
:ObviousResizeLeft
```

- Plug in 설치 없이 사용 가능

```
// Plug In 설치 없이 가능한 명령어
// 위, 아래 크기 조절
:resize +10

// 좌, 우 조절
:vertical resize +10

```

<hr>

<br>

<br>

# tokio

https://tokio.rs/tokio/tutorial/hello-tokio

<br>

## WebScraping

https://stackoverflow.com/questions/70429627/webscraping-a-list-of-items

<br>

# Science : Rust-cookbook

https://rust-lang-nursery.github.io/rust-cookbook/science.html

 <br>

# Science : Rust Blog 

https://m.blog.naver.com/PostList.naver?blogId=phy2sci&categoryName=RUST%20%EC%BD%94%EB%94%A9%EC%9C%BC%EB%A1%9C%20%EB%AC%BC%EB%A6%AC%ED%95%99%EC%9D%84..&categoryNo=15&logCode=0

<br>

<hr>


# Easy Rust 스승님 최고  ❤️

<h1>Updating</h1>

2021-12-10 : Rust 기초 강의 시작<br>

> #### Rust 스승님 Git

> - https://github.com/Dhghomon/easy_rust/

> - 유튜브 주소(한글 강의)
> - 1강
> - https://www.youtube.com/watch?v=W9DO6m8JSSs

<hr>

> - 유튜브 주소(영어 강의)
> - 1강
> - https://www.youtube.com/watch?v=-lYeJeQ11OI&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk&index=1

-Easy Rust eBook

- https://dhghomon.github.io/easy_rust/
<hr>

- Rust 강의 집중!!

<br>

# Rust PlayGround

[https://play.rust-lang.org/](https://play.rust-lang.org/)

<br>

<hr>

<br>

# Rust vs C# primitive type

- 영어 출처
https://learn.microsoft.com/en-us/dotnet/api/system.type.isprimitive?view=net-7.0

<br>

 8bit = 1 bytes

<br>

<table border="1">
    <tr>
    <td colspan="3" align="center">Rust vs C#</td>
    </tr>
    <tr align="center">
        <td>분류(Type) </td>
        <td>Rust</td>
        <td>C# </td>
    </tr>
    <tr align="center">
        <td>Char<br>char</td>
        <td>i8<br>(size: 4 bytes)</td>
        <td>char<br>(size: 2 bytes)</td>
    </tr>
    <tr align="center">
        <td>signed integer<br>8bit</td>
        <td>i8<br>(size: 1 bytes)</td>
        <td>sbyte<br>(size: 1 bytes)</td>
    </tr>
    <tr align="center">
        <td>signed integer<br>16bit</td>
        <td>i16<br>(size: 2 bytes)</td>
        <td>short<br>(size: 2 bytes)</td>
    </tr>
    <tr align="center">
        <td>signed integer<br>32bit</td>
        <td>i32<br>(size: 4 bytes)</td>
        <td>int<br>(size: 4 bytes)</td>
    </tr>
    <tr align="center">
        <td>signed integer<br>64bit</td>
        <td>i64<br>(size: 8 bytes)</td>
        <td>long</td>
    </tr>
    <tr align="center">
        <td>--</td>
        <td>--</td>
        <td>--</td>
    </tr>
    <tr align="center">
        <td>unsigned integer<br>8bit</td>
        <td>u8</td>
        <td>byte</td>
    </tr>
    <tr align="center">
        <td>unsigned integer<br>16bit</td>
        <td>u16</td>
        <td>ushort</td>
    </tr>
    <tr align="center">
        <td>unsigned integer<br>32bit</td>
        <td>u32</td>
        <td>uint</td>
    </tr>
    <tr align="center">
        <td>unsigned integer<br>64bit</td>
        <td>u64</td>
        <td>ulong</td>
    </tr>
    <tr align="center">
        <td>--</td>
        <td>--</td>
        <td>--</td>
    </tr>
    <tr align="center">
        <td>floating point<br>부동 소수점<br>32 bit</td>
        <td>f32<br>(size: 4bytes)</td>
        <td>float<br>(size: 4bytes)</td>
    </tr>
    <tr align="center">
        <td>floating point<br>부동 소수점<br>64 bit</td>
        <td>f64<br>(size: 8bytes)</td>
        <td>double<br>(size: 8bytes)</td>
    </tr>
    <tr align="center">
        <td>--</td>
        <td>--</td>
        <td>--</td>
    </tr>
    <tr align="center">
        <td>Decimal<br>128 bit</td>
        <td>f128</td>
        <td>decimal<br>(size: 16bytes)</td>
    </tr>
</table>

<br>

- C# byte 용량정리 잘됨

https://condor.depaul.edu/sjost/nwdp/notes/cs1/CSDatatypes.htm

<br>

- C# char

https://learn.microsoft.com/ko-kr/dotnet/csharp/language-reference/builtin-types/char

- C# decimal

https://learn.microsoft.com/en-us/dotnet/csharp/language-reference/builtin-types/floating-point-numeric-types


<br>

<hr>

- Rust types

https://dhghomon.github.io/easy_rust/Chapter_7.html

<br>

- 러스트변수용량계산하기_Calculating the variable capacity_Java Hello World_Print#rust

https://youtu.be/ncmbWBs2-WA


<br>

- Rust f32, f64 byte 잘 나옴

https://docs.rs/fsize/latest/fsize/

<br>

<hr>

<br>

<hr>

# Rust Tutorial Full Course

https://youtu.be/ygL_xcavzQ4

<br>
