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
cargo run --example all_winit_glium # cool example
cargo run --example hello_world # simple example
```

examples are found in `backends/conrod_glium`.

### Steal Their Example

Their examples are made to be run _within_ their crate. We want to get this to run in our own crate.

```bash
# cd SOMEWHERE
# create a new crate called gui
cargo new --lib gui
```

Have a look at `conrod/backends/conrod_glium/hello_world.rs`. It's written in edition 2015 I think, so some syntax in there is no longer encouraged today. It should still compile though.

Let us start with the first few lines of `hello_world.rs`:

```rust
//! A simple example that demonstrates using conrod within a basic `winit` window loop, using
//! `glium` to render the `conrod_core::render::Primitives` to screen.

#[macro_use] extern crate conrod_core;
extern crate conrod_glium;
#[macro_use] extern crate conrod_winit;
extern crate find_folder;
extern crate glium;

mod support;

use conrod_core::{widget, Colorable, Positionable, Widget};
use glium::Surface;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 200;
```

I copied them over to my own crate in `gui/src/lib.rs` and try to build that.

```bash
cargo build
# fails because it can't find crate "conrod_core"
```

The `extern crate` is no longer needed in the 2018 edition though:

>  Now, to add a new crate to your project, you can add it to your `Cargo.toml`, and then there is no step two. If you're not using Cargo, you already had to pass `--extern` flags to give `rustc` the location of external crates, so you'd just keep doing what you were doing there as well. 
>
> [Source]( https://doc.rust-lang.org/edition-guide/rust-2018/module-system/path-clarity.html#an-exception )

So let us depend on conrod by adding the line to `Cargo.toml`:

```bash
$ cat Cargo.toml
[package]
name = "gui"
version = "0.1.0"
authors = ["eric <eric@mink.li>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
conrod = { version = "0.61", features = ["winit", "glium"] }
```

That does not help finding `conrod_core`. Luckily they have a hidden [third chapter]( https://docs.piston.rs/conrod/src/conrod_core/guide/chapter_3.rs.html ) to their guide (from 2017), which is not linked in the README. And it turns out that we need to do quite a few things differently when we are not within the `conrod` crate. So let's follow that guide instead.

> For simplicity, I'm
> specifying the backend in `cargo.toml` because we then won't have to think
> about it in the code itself.

> I'm using the glium backend. [Glium](https://github.com/glium/glium) is a
> cross-platform, safe wrapper for OpenGL, written in Rust. It is listed as not-
> maintained on Github, but limited maintenance is being undertaken by the
> developer of Conrod. Glium has a very good, gentle
> [introduction]( https://github.com/glium/glium/tree/master/book ) to OpenGL - enough to
> convince me that I really want Conrod to do the work for me!

After following the guide up to line 196, we have a file that actually builds with our one dependency in `Cargo.toml` above.

```rust
/// gui/src/lib.rs
/// A Hello World based and annotated with help of https://docs.piston.rs/conrod/src/conrod_core/guide/chapter_3.rs.html
use conrod::backend::glium::glium;
/*
 `Surface` is a trait required for glium, specifically for the call to
`target.clear_color` which is coming later.
 */
use glium::Surface;

/*
 The first chunk of boilerplate creates an event loop, which will handle
interaction with the UI, then a window, then a context, then finally links the
event loop, window and context together into a display. The display is the
home for the UI, and is an OpenGL context provided by glium.
*/

const WIDTH: u32 = 400;
const HEIGHT: u32 = 200;
const TITLE: &str = "Hello Conrod!";


pub fn main() {
    // Build the window.
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title(TITLE)
        .with_dimensions((WIDTH, HEIGHT).into());
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    /*
       Now create the UI itself. Conrod has a builder that contains and looks after
       the UI for the user.
       */
    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    /*
       Conrod can use graphics. It stores these in a map. The system needs the map,
       even though it doesn't contain anything at this time, so create it:
       */
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    /*
       Finally, Conrod needs to render its UI. It uses a renderer to do this, so
       create one:
       */
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    'render: loop {
        // Draw the UI if it has changed
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 1.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}

```

Let us try what happens when we run this.

```bash
# create a file in gui/examples/hello.rs
$ cat examples/hello.rs

pub fn main(){
    gui::main();
}

# run that example
cargo run --example hello
```

That results in a green window. Closing it with the X does nothing, so kill it again using `CTRL+C` in the terminal.

![HowToConrod_hello_world_1](.\HowToConrod_hello_world_1.png)