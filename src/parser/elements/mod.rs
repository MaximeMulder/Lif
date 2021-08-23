macro_rules! declare_node {
    ( $name:ident ) => {
        pub const $name: Element = Element::new(stringify!($name));
    }
}

pub mod expressions;
pub mod controls;
pub mod ignores;
pub mod jumps;
pub mod keywords;
pub mod literals;
pub mod productions;
pub mod structures;
pub mod symbols;
