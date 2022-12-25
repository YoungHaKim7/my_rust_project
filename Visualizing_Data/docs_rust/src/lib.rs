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

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
