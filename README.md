# rust-mactors
some good macros to make my life easier 

## example use:
```rust
    eterm!(clear);
    eterm!(color(bg,Green));
    eterm!(color(fg,Red)); 
    let (rows,cols) = eterm!(size);
    eterm!(raw);
```
