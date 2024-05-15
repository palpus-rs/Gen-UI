```toml
[dependencies]
makepad-widgets = {path = "E:/Rust/learn/makepad/makepad-rik/widgets"}
```

```
{
    node: src/views/root.gen,
    children: [
        {node: src/a.gen, children: [
            {node: src/views/b.gen, children: None},
            {node: src/views/d.gen, children: None},
            {node: src/components/c.gen, children: None}
        ]},
   ]
}
```