//! This component is only there to re-export libraries used in the rest of the sub-crates
//! without having to add them to each `Cargo.toml`. This way, updating a library version only requires
//! modifying one crate instead of eg updating Tera in 5 sub crates using it. It also means if you want
//! to define features, it is done in a single place.
//! It doesn't work for crates exporting macros like `serde` or dev deps but that's ok for most.

pub use ahash;
pub use ammonia;
pub use atty;
pub use base64;
pub use csv;
pub use elasticlunr;
pub use filetime;
pub use gh_emoji;
pub use glob;
pub use globset;
pub use grass;
pub use image;
pub use lexical_sort;
pub use minify_html;
pub use nom_bibtex;
pub use num_format;
pub use once_cell;
pub use percent_encoding;
pub use pulldown_cmark;
pub use pulldown_cmark_escape;
pub use quickxml_to_serde;
pub use rayon;
pub use regex;
pub use relative_path;
pub use reqwest;
pub use serde_json;
pub use serde_yaml;
pub use sha2;
pub use slug;
pub use svg_metadata;
pub use syntect;
pub use tera;
pub use termcolor;
pub use time;
pub use toml;
pub use unic_langid;
pub use unicode_segmentation;
pub use url;
pub use walkdir;
pub use webp;

pub mod consts {
    macro_rules! dirs {
        (
            $vis:vis const $id:ident = $v:literal;
            $($rest:tt)*
        ) => (
            $vis const $id: &str = $v;
            dirs!($($rest)*);
        );
        (
            $vis:vis const ($id:ident, $slashid:ident) = $v:literal;
            $($rest:tt)*
        ) => (
            dirs!($vis const $id = $v;);
            $vis const $slashid: &str = concat!("/", $v);
            dirs!($($rest)*);
        );
        (
            $vis:vis const ($id:ident, $slashid:ident, $idslash:ident) = $v:literal;
            $($rest:tt)*
        ) => (
            dirs!($vis const ($id, $slashid) = $v;);
            $vis const $idslash: &str = concat!($v, "/");
            dirs!($($rest)*);
        );
        () => ();
    }
    dirs! {
        pub const (CONTENT_DIR, SLASH_CONTENT_DIR, CONTENT_DIR_SLASH) = "content";
        pub const (SASS_DIR, SLASH_SASS_DIR) = "sass";
        pub const (STATIC_DIR, SLASH_STATIC_DIR) = "static";
        pub const (TEMPLATES_DIR, SLASH_TEMPLATES_DIR) = "templates";
        pub const (THEMES_DIR, SLASH_THEMES_DIR) = "themes";
        pub const PUBLIC_DIR = "public";
    }
}
