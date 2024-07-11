## Managing Growing Projects

* *Crate*: A create is the smallest amount of code interpreted by the compiler (a collection of codes);
    + binary crate: Code files that can be converted to an executable. Has main().
    + library crate: Code files that cannot be converted to an executable, but defines funcionalities to be shared and used by multiple projects. Doesn't have main().
* *Packages*: Packages contains a configuration file (Cargo.toml) and at least one crate, but at most only one library crate.

* *Declaring Modules and submodules*: A module can be declare in crate using the keyword **mod**, like this: *mod module;*, then the compiler will try to get module from:
    + Inline definition, replacing semicolon with curly brackets;
    + The file *module.rs*, inside the same folder as the current file;
    + The file *mod.rs* (always this name) inside the folder *module* (./module/mod.rs)
* *Using Modules and submodules*: Once declared, the module is part of the crate, so it can be called inside that crate, as long privacy rules allow, like *crate::module::submodule::Newmodule*
* *Private vs public*: Every code inside a module is private by default. To make a module or a part of a module public, use the keyword *pub* before declaration. The *pub* keyword only let the ancestor refer to it, but not access the inner code.
* *use keyword*: In order to reduce the path to use modules and submodules, the keyword *use* can be used. To enable the codes that calls an imported code to use that imported code, *pub* can be used at the beginning of the sentence to re-export Names.

## Modules

Modules are importante to improve organization, readability and reuse of a code. When dividing in modules, private attributes and/or implementations can be left private by external uses, exposing (using *pub*) only the needed parts.
To find a module inside a module tree, the path to it is used. This path can assume two formats:
* *Absolute path*: The full path, from *crate* to the required module: *crate::module::submodule::Newmodule*;
* *Relative path*: The path starts in current path using the keywords *self* and *super*.

## Bringing Paths into Scope

To use an external package, it must be declared inside *Cargo.toml* file (like in b-guessing-game). Then, to bring it into a scope, use the *use* keyword with the name of the crate and the item needed for the scope.
While using multiple items defined in the same crate, to prevent waste of vertical space, it can all be nested inside curly brackets separated by a comma: **use std::{cmp::Ordering, io}**. Nested path can bring itself:
```
Instead of:
use std::io;
use std::io::Write;

Use this:
use std::io{self, Write};
```
There's still the possibility to bring all *public* items from  a module into scope using the Glob Operator: *
```
use std::collections::*;
```
