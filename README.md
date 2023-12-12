# Link

<a href="https://github.com/YoungHaKim7/easyrust_training#rust-gitignore-macos-linuxos"> echo로 .gitignore 넣기 (Rust)</a>

<hr>

# <em><strong>유료버젼</em></strong> 🖥💰 Easy Rust Korean / Rust in a Month of Lunches<a href="https://github.com/YoungHaKim7/easyrust_training#link"><↑ Top ↑🔝></a>

- https://www.manning.com/books/learn-rust-in-a-month-of-lunches


<hr>

<hr>

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
- Easy Rust eBook

- https://dhghomon.github.io/easy_rust/

- 영상 모아보기
  - KR한국어버젼
    - https://www.youtube.com/playlist?list=PLfllocyHVgsSJf1zO6k6o3SX2mbZjAqYE

  - Eng.영어버젼 Easy Rust / Rust in a Month of Lunches: bite-sized Rust tutorials
    - https://youtube.com/playlist?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk&si=y29g1LlLc4-M5OAf
 
<hr>

## Rust Atomics (Easy rust_ mithradates)


- Rust Atomics and Locks 같이 읽기(한국어 버젼 KR)
  - https://youtube.com/playlist?list=PLfllocyHVgsRLUtjznx-OX3mQYL7pHKyO&si=r8QGLcjwI_PWqTrt
- Reading Rust Atomics and Locks (영어 버젼 Eng.)
  - https://youtube.com/playlist?list=PLfllocyHVgsR_MVp_RyBwujomqYyIGDdE&si=eJrDocd6ABtRZDSC

<hr>

<hr>

- Rust 강의 집중!!

<br>

# Rust PlayGround<a href="https://github.com/YoungHaKim7/easyrust_training#link"><↑ Top ↑🔝></a>

[https://play.rust-lang.org/](https://play.rust-lang.org/)

# Vim Coc 관리하기 <a href="https://github.com/YoungHaKim7/easyrust_training#link"><↑ Top ↑🔝></a>

- 서버가 잘 돌아가는 체크하기 

```
:CocList services

```

## Vim Coc 필요없는 서버 지우기 중복되는거<a href="https://github.com/YoungHaKim7/easyrust_training#link"><↑ Top ↑🔝></a>


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

# Vim (:CocConfig) setting.JSON<a href="https://github.com/YoungHaKim7/easyrust_training#link"><↑ Top ↑🔝></a>

```
{
//  "workbench.colorCustomizations": {
//    // Name of the theme you are currently using
//    "[Default Dark+]": {
//      "editorInlayHint.foreground": "#868686f0",
//      "editorInlayHint.background": "#f700d6",
//
//      // Overrides for specific kinds of inlay hints
//      "editorInlayHint.typeForeground": "#f700d6",
//      "editorInlayHint.parameterForeground": "#fdb6fdf0",
//    }
  //}
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

## vim documentation scroll(Vim key map)<a href="https://github.com/YoungHaKim7/easyrust_training#link"><↑ Top ↑🔝></a>


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

<br>

# Vim Command<a href="https://github.com/YoungHaKim7/easyrust_training#link"><↑ Top ↑🔝></a>


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

# Vim CocInstall (rust-analyzer)<a href="https://github.com/YoungHaKim7/easyrust_training#link"><↑ Top ↑🔝></a>


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

# Vim 창 나누기<a href="https://github.com/YoungHaKim7/easyrust_training#link"><↑ Top ↑🔝></a>


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

# NERDtree 단축키 <a href="https://github.com/YoungHaKim7/easyrust_training#link"><↑ Top ↑🔝></a>


```
I - 숨긴 파일 확인
R - Reflash
m - 파일 추가 삭제 가능 
```

# Vim 설치된 플러그인 목록 확인<a href="https://github.com/YoungHaKim7/easyrust_training#link"><↑ Top ↑🔝></a>



~/.config/coc/extensions/package.json

<br>

:CocInstall 하면 위 폴더 JSON 에서 버젼 관리를 한다. ㅋ 

<br>

https://johngrib.github.io/wiki/vim/coc-nvim/

<br>

<hr>


# Rust ```.gitignore``` (macOS, LinuxOS)<a href="https://github.com/YoungHaKim7/easyrust_training#link"><↑ Top ↑🔝></a>


- gitignore(Rust)

- https://github.com/github/gitignore/blob/main/Rust.gitignore

- https://github.com/github/gitignore


```bash
echo "# Result\xa\xa\x60\x60\x60\xa\xa\x60\x60\x60" >> README.md &&

echo "# A collection of useful .gitignore templates " >> .gitignore &&
echo "# https://github.com/github/gitignore\xa" >> .gitignore &&
echo "# General" >> .gitignore &&
echo ".DS_Store" >> .gitignore &&
echo "dir/otherdir/.DS_Store\xa" >> .gitignore &&

echo "# VS Code files for those working on multiple tools" >> .gitignore &&
echo ".vscode/\xa" >> .gitignore &&
echo "# Generated by Cargo" >> .gitignore  &&
echo "# will have compiled files and executables" >> .gitignore &&
echo "debug/" >> .gitignore &&
echo "target/\xa" >> .gitignore &&

echo "# Remove Cargo.lock from gitignore if creating an executable, leave it for libraries" >> .gitignore &&
echo "# More information here https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html" >> .gitignore &&
echo "Cargo.lock\xa" >> .gitignore &&

echo "# These are backup files generated by rustfmt" >> .gitignore &&
echo "**/*.rs.bk\xa" >> .gitignore &&

echo "# MSVC Windows builds of rustc generate these, which store debugging information" >> .gitignore &&
echo "*.pdb\xa" >> .gitignore &&

echo "# WASM" >> .gitignore &&
echo "pkg/" >> .gitignore &&
echo "/wasm-pack.log" >> .gitignore &&
echo "dist/" >> .gitignore
```

- ```WindowsOS```(PowerShell)

```
echo "# Result`r`n`r`n```````n`r`n```````n" >> README.md &&
echo "/target`r`nCargo.lock`r`n" >> .gitignore
```

- ```WindowsOS```(PowerShell) ver.2
```
echo "# A collection of useful .gitignore templates/`r`n# https://github.com/github/gitignore/`r`n# General/`r`n.DS_Store/`r`n `r`n# Generated by Cargo`r`n# will have compiled files and executables`r`ndebug/`r`ntarget/`r`n `r`n# Remove Cargo.lock from gitignore if creating an executable, leave it for libraries`r`n# More information here https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html`r`nCargo.lock`r`n `r`n# These are backup files generated by rustfmt`r`n**/*.rs.bk`r`n `r`n# MSVC Windows builds of rustc generate these, which store debugging information`r`n*.pdb`r`n" >> .gitignore && echo "# Result`r`n`r`n```````n`r`n```````n" >> README.md
```

https://devblogs.microsoft.com/scripting/powertip-new-lines-with-powershell/
