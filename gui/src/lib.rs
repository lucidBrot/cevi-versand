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
