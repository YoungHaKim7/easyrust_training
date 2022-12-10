# Rust Doc std(standard Library DOC)

https://doc.rust-lang.org/std/

<br>

##  DOCS.RS(rust 기타 crate문서들)

https://docs.rs/

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


# Vim Coc 관리하기 

- 서버가 잘 돌아가는 체크하기 

```
:CocList services

```

## Vim Coc 필요없는 서버 지우기 중복되는거

<br>

```
// Ada 서버 지우는 방법
:CocUninstall coc-als

```

<br>

<hr>

# Vim Setting (type 빨강색으로 강조하기 칙칙한 검은색 너무 싫다.)

-vim 에서

```
:hi CocInlayHint ctermbg=125

5 밝다
52 어둡다

:hi 먹통일때 임시방편
:colorscheme jellybeans 기본 스킨
:colorscheme slate 후보군1
:colorscheme murphy 그나마 낫다?
:colorscheme habamax 전제적 회색 느낌

```

- 내가 원하는 색깔 256 컬러에서 고르자 ㅎㅎ

https://www.ditig.com/256-colors-cheat-sheet

# Vim (:CocConfig) setting.JSON

```
{
  "workbench.colorCustomizations": {
      "[Default Dark+]": {
          "editorInlayHint.foreground": "#868686f0",
          "editorInlayHint.background": "#f700d6",
      "editorInlayHint.typeForeground": "#f700d6",
      "editorInlayHint.parameterForeground": "#fdb6fdf0",
      }
  },
  "workspace.workspaceFolderFallbackCwd": true",
  "rust-analyzer.semanticHighlighting.operator.specialization.enable": true,
  "rust-analyzer.inlayHints.typeHints.enable": true,
  "rust-analyzer.hover.documentation.enable": true,

  // rust-analyzer setting~
  "rust-analyzer.inlayHints.enable": true,
  // "rust-analyzer.cargo.loadOutDirsFromCheck": true,
  "rust-analyzer.procMacro.enable": true,
  "rust-analyzer.lens.enable": true,
  "rust-analyzer.lens.implementations.enable": true,
}

```

<br>

<hr>

## vim documentation scroll(Vim key map)

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

# TrubleToggle 과 비슷한 기능

```
:CocDiagnostics

```

<br>

<hr>

# Rust Projetct 할때 실시간 체크  cargo watch 같은거 

```
bacon
```

Installation
```
cargo install --locked bacon

&
bacon

```

https://dystroy.org/bacon/

<br>

<br>

# NERDtree 단축키 
```
I - 숨긴 파일 확인
R - Reflash
m - 파일 추가 삭제 가능 
```

# Vim 설치된 플러그인 목록 확인

~/.config/coc/extensions/package.json

<br>

:CocInstall 하면 위 폴더 JSON 에서 버젼 관리를 한다. ㅋ 

<br>

https://johngrib.github.io/wiki/vim/coc-nvim/

<br>




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
