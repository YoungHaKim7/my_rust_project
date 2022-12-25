# Mermaid Diagram tool(Rust module aquamarine)

https://crates.io/crates/aquamarine


# cargo doc 

```
$ cargo doc 

$ cargo doc --open

```

# cargo doc --open


https://frehberg.com/2022/12/docs-as-code-mermaid-inline-diagrams/


```mermaid
graph LR
    s([Source]) --> a[[aquamarine]]
    r[[rustdoc]] --> f([Docs w/ Mermaid!])
    subgraph rustc[Rust Compiler]
    a -. inject mermaid.js .-> r
    end
```


```
#[cfg_attr(doc, aquamarine::aquamarine)]
/// A function showcasing aquamarine defaults
///
/// With aquamarine it's possible to embed Mermaid diagrams into your Rust documentation using the code snippets
/// 
/// ```mermaid
/// graph LR
///     s([Source]) --> a[[aquamarine]]
///     r[[rustdoc]] --> f([Docs w/ Mermaid!])
///     subgraph rustc[Rust Compiler]
///     a -. inject mermaid.js .-> r
///     end
/// ```
///
/// The diagram is going to be located in place of the code snippet
///
/// Dark mode is automatically enabled if `dark` or `ayu` rustdoc theme is selected.
///
/// You might need to reload the page to redraw the diagrams after changing the theme.
pub fn example() {}
```
