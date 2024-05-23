# global

## constant

Global constants can be used anywhere, but it is important to note the corresponding props' type

```rust
live_design!{
    // global font size
    GLOBAL_FONT_SIZE = 16.0
    COLOR_BG = #xfff8ee

    // ...others
    // use GLOBAL_BG
    ui: <Window>{
        draw_bg: {color: COLOR_BG},
    }
}
```

Resources such as fonts and images also belong to global constants, see `import.md`

---
Next: [import]()
---