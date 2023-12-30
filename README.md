# No mod.rs?

![megamind.gif](https://media1.tenor.com/m/NeBbYuvg2EsAAAAC/megamind.gif)

You know what. _Un-fricks your modules_

---

This crate makes folder organization in your Rust projects slightly less terrifying to work with.

If you're used to working with programming languages that have normal file management, this one is for you!

Removes the need for a `mod.rs` in _most_ cases.

## Features

Lets imagine the following file structure:
```tree
src/
├── main.rs
└── my_module/
    ├── one.rs
    └── two.rs
```

- **Basic**

  ```rs
    folder_module!(my_module);
  ```

  Creates a definition for a module; recursively defining any sub-modules inside it as well.

- **Flags**

  ```rs
    folder_module!(pub use * my_module);
  ```

  ---

  There are additional flags you can use
  - `pub` _(makes the module and/or the submodules public)_
  - `use` _(creates a `use my_module::*;` statement. Can be followed by either `all` or `*` to make an individual `use sub_module::*;` statement for every sub module inside the main module)_

- **Selector**

  **/!\ Not implemented /!\\**

  ```rs
    folder_module!({one, two} from my_module);
  ```

  ---

  Lets you be more picky about which module you declare
