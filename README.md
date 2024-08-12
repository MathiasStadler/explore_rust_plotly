# init project

```bash
cd \
&& mkdir rust_project \
&& cd $_ \
&& touch README.md \
&& ln -s README.md README \
&& cargo init "." \
&& cargo add rustfmt \
&& rustup component add rustfmt \
&& mkdir examples \
&& cp src/main.rs examples/example.rs \
&& sed -i -e 's/world/example/g' examples/example.rs \
&& rustup  show \
&& rustup  check \
&& rustup toolchain uninstall stable \
&& rustup toolchain install stable \
&& rustup update  --force \
&& rustup show \
&& cargo add rustfmt \
&& rustup component add rustfmt \
# cold update 
# https://github.com/rust-lang/rustup/issues/2729
# rustup toolchain uninstall stable
# rustup toolchain install stable
&& rustup show \
&& touch FROM_HERE.md \ 
&& cargo build 
```

## add crate for this project

-  https://github.com/plotly/plotly.rs/blob/main/plotly/Cargo.toml

```bash
cargo add plotly



cargo add plotly_kaleido
This is an internal crate which implements the kaleido feature for Plotly.rs.
The kaleido feature enables Plot conversion to the following output formats: png, jpeg, webp, svg, pdf and eps.
See examples/ for usage demonstrations.
```