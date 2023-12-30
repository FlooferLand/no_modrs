# No mod.rs?

![megamind.gif](https://media1.tenor.com/m/NeBbYuvg2EsAAAAC/megamind.gif)

You know what. _Un-fricks your modules_

---

This crate makes folder organization in your Rust projects slightly less terrifying to work with.

If you're used to working with programming languages that have normal file management, this one is for you!

Removes the need for a `mod.rs` in _most_ cases.

## Features

- **Basic**

  ```rs
    folder_module!(my_module);
  ```

  Imports a module

- **Flags**

  ```rs
    folder_module!(pub use * my_module);
  ```

  ---

  There are additional flags you can use
  - `pub` _(makes the module and/or the submodules public)_
  - `use` _(creates a `use my_module::*;` statement. Can be followed by `all` to make an individual `use sub_module::*;` for every sub module inside the main module)_

- **Selector**

  **/!\ Not implemented /!\\**

  ```rs
    folder_module!({one, two} from my_module);
  ```

  ---

  Lets you be more picky about which module you declare
