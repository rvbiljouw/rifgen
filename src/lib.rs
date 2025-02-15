//! This crate works with [`flapigen`] to provide an easy to use rust code with other programming languages
//!
//! [`flapigen`]: https://docs.rs/flapigen
//!
//! This crate uses procedural macros such as `#generate_interface` to locate methods and types for which interface
//! code would be generated automatically
//! Suppose you have the following Rust code:
//! ```rust
//! struct Foo {
//!     data: i32
//! }
//!
//! impl Foo {
//!     fn new(val: i32) -> Foo {
//!         Foo{data: val}
//!     }
//!
//!     fn f(&self, a: i32, b: i32) -> i32 {
//!         self.data + a + b
//!     }
//!
//!     ///Custom doc comment
//!     fn set_field(&mut self, v: i32) {
//!         self.data = v;
//!     }
//! }
//!
//! ```
//! Using [`flapigen`], you need to generate an interface file with contents:
//!
//! [`flapigen`]: https://docs.rs/flapigen
//! ```ignore
//! foreign_class!(class Foo {
//!     self_type Foo;
//!     constructor Foo::new(_: i32) -> Foo;
//!     ///Custom doc comment
//!     fn Foo::set_field(&mut self, _: i32);
//!     fn Foo::f(&self, _: i32, _: i32) -> i32;
//! });
//! ```
//!
//! Using this crate, you can simply annotate the methods using either `#[generate_interface]`, `#[generate_interface(constructor)]` or `#[generate_interface_doc]`
//!
//! First add the appropriate dependencies
//!
//! In Cargo.toml
//! ```toml
//! [dependencies]
//! rifgen = "*"
//! [build-dependencies]
//! rifgen = "*"
//! ```
//!
//! In build.rs
//!```no_run
//! //place this code before flapigen swig_expand function
//! use rifgen::{Generator, TypeCases, Language};
//! let source_folder = "/user/projects"; //use your projects folder
//! let out_file = "/user/projects/glue.in";
//! Generator::new(TypeCases::CamelCase,Language::Java,source_folder)
//! .generate_interface(out_file)
//! ```
//!
//! Using the example above, the modified code would be
//! ```
//! use rifgen::rifgen_attr::*;
//!
//! struct Foo {
//!     data: i32
//! }
//!
//! impl Foo {
//!     #[generate_interface(constructor)]
//!     fn new(val: i32) -> Foo {
//!         Foo{data: val}
//!     }
//!     #[generate_interface]
//!     fn f(&self, a: i32, b: i32) -> i32 {
//!         self.data + a + b
//!     }
//!
//!     ///Custom doc comment
//!     #[generate_interface]
//!     fn set_field(&mut self, v: i32) {
//!         self.data = v;
//!     }
//! }
//! ```
//!
//! This crate works with doc comments so all doc comments would be preserved
//! Use `#[generate_interface_doc]` on <b>structs only</b> to preserve the doc comment of the struct
//! ```
//! ///Data holder
//! # use rifgen_attr::generate_interface_doc;
//! #[generate_interface_doc]
//! struct Foo {
//!     data: i32
//! }
//! ```
//!
//! For `trait` just annotate the trait definition
//! ```
//! ///MyCallback documentation
//! # use rifgen_attr::generate_interface;
//! #[generate_interface]
//! trait MyCallback {
//!     
//!     fn on_click(&self) {
//!     }
//! }
//! ```
//! For `enum`, it's similar to `trait`
//! ```
//! # use rifgen_attr::generate_interface;
//! #[generate_interface]
//! enum MyEnum {
//!     One,
//!     Two
//! }
//! ```
mod enums;
mod generator_lib;
mod maps;
mod text_formatter;
mod traits;
mod types_structs;

pub extern crate rifgen_attr;

use crate::generator_lib::FileGenerator;
use std::path::Path;

/// The various type cases to use when generating interface files
/// i.e CamelCase or snake_case or just leave the style unchanged
#[derive(Copy, Clone)]
pub enum TypeCases {
    /// Various names of methods and variants are untouched.
    /// This is the default setting
    Default,
    /// Convert all method names to CamelCase
    CamelCase,
    /// Convert all method method names to snake_case
    SnakeCase,
}

/// The builder to use in build.rs file to generate the interface file
pub struct Generator<P: AsRef<Path>> {
    type_case: TypeCases,
    scr_folder: P,
    language: Language,
}

///Supported languages for now
pub enum Language {
    Java,
    Cpp,
}

impl<S: AsRef<Path>> Generator<S> {
    /// Creates a new generator instance
    ///
    /// `scr_folder` refers to the starting folder where it is recursively walked
    ///through to find other files
    pub fn new(type_case: TypeCases, language: Language, scr_folder: S) -> Generator<S> {
        Generator {
            type_case,
            scr_folder,
            language,
        }
    }

    ///`interface_file_path` refers to the path of the output file.
    /// If it exists, it would be overwritten
    pub fn generate_interface<I: AsRef<Path>>(self, interface_file_path: I) {
        FileGenerator::new(self.type_case, interface_file_path, self.scr_folder)
            .build(self.language);
    }
}

/*#[cfg(test)]
mod tests {
    use crate::{Generator, Language, TypeCases};

    #[test]
    fn test() {
        for i in 0..20 {
            Generator::new(TypeCases::CamelCase, Language::Java, "C:\\Users\\taimoor\\IdeaProjects\\rifgen\\test folder")
                .generate_interface(format!("C:\\Users\\taimoor\\IdeaProjects\\rifgen\\src\\TEST{}.in", i))
        }
    }
}*/