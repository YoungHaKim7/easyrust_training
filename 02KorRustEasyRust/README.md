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
