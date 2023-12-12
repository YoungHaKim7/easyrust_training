# Result

```bash
$ g++ main.cpp -std=c++2b

main.cpp:7:9: error: redefinition of 'x'
    int x = x + 1;
        ^
main.cpp:5:9: note: previous definition is here
    int x = 5;
        ^
1 error generated.

```
