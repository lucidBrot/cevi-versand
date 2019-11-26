### Run Their Example

 Latest commit [78438e1](https://github.com/PistonDevelopers/conrod/commit/78438e196ff1499f843c5ba6eb11085e062a7bb7) 23 days ago. Today is 26.11.2019

```bash
git clone https://github.com/PistonDevelopers/conrod.git
cd conrod
cargo build # fails
```
Edit `conrod/Cargo.toml` so that it looks like this:
```bash
$ cat Cargo.toml
[workspace]
members = [
    "conrod_core",
    "conrod_derive",
    "backends/conrod_example_shared",
    "backends/conrod_winit",
    "backends/conrod_gfx",
    "backends/conrod_glium",
    "backends/conrod_piston",
#    "backends/conrod_vulkano",
]
```
```bash
cargo build # succeeds
cargo run --example all_winit_glium
```

examples are found in `backends/conrod_glium`.

### Steal Their Example

Their examples are made to be run _within_ their crate. We want to get this to run in our own crate.

```bash
# cd SOMEWHERE
# create a new crate called gui
cargo new --lib gui
```

