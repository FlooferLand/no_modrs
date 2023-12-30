#![feature(proc_macro_span)]
#![feature(proc_macro_diagnostic)]

use std::path::PathBuf;

use quote::{quote, format_ident};
use proc_macro::{Span, TokenStream as TokenStream1};
use proc_macro2::TokenStream;
use proc_macro::Diagnostic;

#[proc_macro]
pub fn folder_module(input: TokenStream1) -> TokenStream1 {
    let input = TokenStream::from(input);
    let input_string = input.to_string();
    let args = input_string.split(' ').collect::<Vec<&str>>();

    // Argument parsing
    let modifiers = &args[0..args.len()-1];
    let is_pub = modifiers.contains(&"pub");
    let is_use = modifiers.contains(&"use");
    let is_all = modifiers.contains(&"all") || modifiers.contains(&"*");
    if modifiers.iter().any(|&v| v.starts_with('{') && v.ends_with('}')) {
        // TODO: Implement this
    }

    // File names & path
    let base_name = match args.last() {
        Some(name) => name,
        None => {
            panic!("The module name must be declared. Example: \"folder_module!(my_module)\"");
        },
    };
    let file_path = Span::call_site().source_file().path();
    let base_path = file_path.parent().unwrap().join(&base_name);

    // Optional identifiers
    let pub_ident = if is_pub { quote!(pub) } else { quote!() };
    let base_name_ident = format_ident!("{base_name}");

    // The output
    let (children, has_modrs) = walk(&base_path.to_path_buf(), is_use && is_all);
    let mut output;
    if has_modrs {
        output = quote!(
            #pub_ident mod #base_name_ident;
        );
    } else {
        output = quote!(
            #pub_ident mod #base_name_ident {
                #children
            }
        );
        if is_use && !is_all {
            output = quote!(
                #output
                #pub_ident use #base_name_ident::*;
            );
        }
    }
    println!("\n\n--- OUTPUT ---\n{output}\n--- OUTPUT ---\n\n");
    TokenStream1::from(output)
}

fn walk(folder: &PathBuf, use_all: bool) -> (Option<TokenStream>, bool) {
    let Some(name) = folder.file_name() else { return (None, false) };
    let Some(name) = name.to_str() else { return (None, false) };
    let name = format_ident!("{name}");

    let mut children = quote!();
    
    let dir = match std::fs::read_dir(folder) {
        Ok(dir) => dir,
        Err(err) => {
            panic!("no_modrs: Could not read from directory \"{}\".\n\tError: {err}", folder.display())
        },
    };
    for entry in dir {
        let Ok(entry) = entry else { continue };
        let Ok(info) = entry.metadata() else { continue };
        let path = entry.path();

        let Some(stem) = path.file_stem() else { continue; };
        let Some(stem) = stem.to_str() else { continue; };
        let stem_ident = format_ident!("{stem}");

        println!("path = \"{}\"", path.display());
    
        if info.is_file() {
            if let Some(ext) = path.extension() {
                if ext != "rs" { continue }
            }
            if let Some(stem) = path.file_stem() {
                if stem == "mod" {
                    return (None, true);
                }
            }

            children = quote!(
                #children
                pub mod #stem_ident;
            );
            if use_all {
                children = quote!(
                    #children
                    pub use #stem_ident::*;
                );
            }
        } else if info.is_dir() {
            if path.join("mod.rs").exists() {
                continue;
            }

            let (walked, should_return_modrs) = walk(&path, use_all);
            if should_return_modrs {
                let message = format!("The folder module at path \"{}\" already has a `mod.rs`.\nThe `folder_module!()` macro will import it's `mod.rs` directly.", folder.display());
                Diagnostic::new(proc_macro::Level::Note, message).emit();
                children = quote!(
                    #children
                    pub mod #stem_ident;
                );
            }
            let Some(walked) = walked else { continue };
            children = quote!(
                #children
                pub mod #stem_ident {
                    #walked
                }
            );
        }
    }
    
    (Some(children), false)
}
