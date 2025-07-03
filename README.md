# gmsh-rs

> [!IMPORTANT]
> This is a very early WIP, the `safe` and `result` module aren't currently usable.

`gmsh-rs` provides safe bindings to the C API of the [GMSH][GMSH] project. The project structure is
inspired by the `cudarc` crate. Three modules are available:

- `safe` - Safe wrapper functions.
- `result` - Basic wrapper to refactor error and return value pointers into proper `Result`s.
- `sys` - Auto-generated raw bindings.

## License

> [!CAUTION]
> Our licensing policy applies only to the Rust crate `gmsh-rs`, not the original GMSH project.
>
> For information about GMSH's licensing policy, refer to the original project's [repository][GMSH].

`gmsh-rs` is distributed under the terms of both the MIT license and the Apache License
(Version 2.0). Refer to `LICENSE-APACHE` and `LICENSE-MIT` for more details.


[GMSH]: https://gitlab.onelab.info/gmsh/gmsh
