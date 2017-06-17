#[macro_use]
extern crate lithium_core;
extern crate lithium_winit;
extern crate lithium_webrender;
extern crate gleam;
extern crate glutin;
extern crate webrender;
extern crate webrender_traits;

use std::thread;
use gleam::gl;
use webrender_traits::{ColorF, Epoch};
use webrender_traits::{DeviceUintSize, LayoutPoint, LayoutRect, LayoutSize};
use webrender_traits::{PipelineId, TransformStyle};
use webrender_traits::{RenderApi};
use lithium_core::Widget;
use lithium_core::Theme;

struct Notifier;

impl webrender_traits::RenderNotifier for Notifier {
    fn new_frame_ready(&mut self) {
    }

    fn new_scroll_frame_ready(&mut self, _composite_needed: bool) {
    }
}

fn window_thread<W: Widget>(mut widget: W, theme: Theme, window_builder: glutin::WindowBuilder<'static>) {
    let event_loop = glutin::EventsLoop::new();

    let window = window_builder
        .with_vsync()
        .with_gl(glutin::GlRequest::GlThenGles {
            opengl_version: (3, 2),
            opengles_version: (3, 0)
        })
        .build(&event_loop)
        .unwrap();

    // This function (`window_thread`) is not exported and we execute it only from a new thread.
    unsafe {
        window.make_current().ok();
    }

    let gl = match gl::GlType::default() {
        gl::GlType::Gl => unsafe { gl::GlFns::load_with(|symbol| window.get_proc_address(symbol) as *const _) },
        gl::GlType::Gles => unsafe { gl::GlesFns::load_with(|symbol| window.get_proc_address(symbol) as *const _) },
    };

    let (mut width, mut height) = window.get_inner_size_pixels().unwrap();

    let opts = webrender::RendererOptions {
        resource_override_path: None,
        debug: false,
        precache_shaders: true,
        device_pixel_ratio: window.hidpi_factor(),
        .. Default::default()
    };

    let size = DeviceUintSize::new(width, height);
    let (mut renderer, sender) = webrender::renderer::Renderer::new(gl, opts, size).unwrap();
    let api = sender.create_api();

    let notifier = Box::new(Notifier);
    renderer.set_render_notifier(notifier);

    let mut epoch = Epoch(0);
    let pipeline_id = PipelineId(0, 0);

    let mut gui = lithium_core::Gui::new(theme);

    while process_events(&mut gui, &event_loop, &window, &mut width, &mut height) {
        let place = widget.appear(&mut gui);

        let hidpi_factor = window.hidpi_factor() as f64;
        
        add_constraints!(gui.layout, [
            (place.left) == 0.0,
            (place.top) == 0.0,
            (place.right) == width as f64/hidpi_factor,
            (place.bottom) == height as f64/hidpi_factor,
        ]);

        // render
        render(&gui, pipeline_id, (width, height), epoch, &api);
        renderer.update();
        renderer.render(DeviceUintSize::new(width, height));
        epoch.0 += 1;

        window.swap_buffers().ok();

        gui.advance();
    }
}

fn process_events(gui: &mut lithium_core::Gui, event_loop: &glutin::EventsLoop, window: &glutin::Window, width: &mut u32, height: &mut u32) -> bool {
    let mut stop = false;

    event_loop.poll_events(|event| {
        if stop {
            return;
        }

        let glutin::Event::WindowEvent { window_id: _window_id, event } = event;
        if let Some(lithium_event) = lithium_winit::winit_event_to_lithium(&event, window.hidpi_factor() as f64) {
            gui.input.event(&lithium_event);
        } else {
            match event {
                glutin::WindowEvent::Closed => stop = true,
                glutin::WindowEvent::Resized(new_width, new_height) => {
                    *width = new_width;
                    *height = new_height;
                }
                _ => ()
            }
        }
    });

    !stop
}

fn render(gui: &lithium_core::Gui, pipeline_id: PipelineId, (width, height): (u32, u32), epoch: Epoch, api: &RenderApi) {
    let layout_size = LayoutSize::new(width as f32, height as f32);
    let mut builder = webrender_traits::DisplayListBuilder::new(pipeline_id, layout_size);
    let bounds = LayoutRect::new(LayoutPoint::zero(), layout_size);
    builder.push_stacking_context(
        webrender_traits::ScrollPolicy::Fixed,
        bounds,
        None,
        TransformStyle::Flat,
        None,
        webrender_traits::MixBlendMode::Normal,
        Vec::new()
    );

    lithium_webrender::build(gui.scene.commands(), &mut builder);
    
    builder.pop_stacking_context();

    let root_background_color = ColorF::new(0.0, 0.0, 0.0, 1.0);
    api.set_display_list(
        Some(root_background_color),
        epoch,
        LayoutSize::new(width as f32, height as f32),
        builder.finalize(),
        true);
    api.set_root_pipeline(pipeline_id);
    api.generate_frame(None);
}

/// Show window which will contain specified widget as a top-level item.
///
/// While the signature of this function may seem convoluted it is actually not hard to use.
///
/// Arguments:
///
/// * `title` - Text to be displayed at the window title bar.
/// * `theme` - Theme to use for rendering.
/// * `f` - closure in which you you must construct a top-level widget to be displayed in the window; see below for the usage example.
///
/// Every window is using a separate thread. This function returns `std::thread::JoinHandle`; by caling `.join()` on this handle
/// you can wait for that thread to finish and for the window to close.
///
/// Example:
///
/// ```ignore
/// spawn_window("Example Window", theme, |window| {
///     window.show(lithium_core::widgets::ClickArea::new());
///     // Instead of `lithium_core::widgets::ClickArea` you would probably want to use a custom widget defined by you.
/// }).join().unwrap();
/// ```
pub fn spawn_window<S, F, R>(title: S, theme: Theme, f: F) -> thread::JoinHandle<R>
  where
    S: Into<String>,
    F: Send + 'static + FnOnce(Window) -> R,
    R: Send + 'static,
{
    let title = title.into();
    
    spawn_window_with_builder(glutin::WindowBuilder::new().with_title(title), theme, f)
}

/// Similar to `spawn_window` but allows more configuration with `glutin::WindowBuilder`.
///
/// See description of `spawn_window` for more info.
pub fn spawn_window_with_builder<F, R>(window_builder: glutin::WindowBuilder<'static>, theme: Theme, f: F) -> thread::JoinHandle<R>
  where
    F: Send + 'static + FnOnce(Window) -> R,
    R: Send + 'static,
{
    let thread_builder = thread::Builder::new().name("Lithium Window".to_owned());

    thread_builder.spawn(|| {
        let window = Window {
            theme,
            window_builder: window_builder,
        };

        f(window)
    }).unwrap()
}

/// Type passed to the closure passed to the `spawn_window`.
pub struct Window {
    theme: Theme,
    window_builder: glutin::WindowBuilder<'static>,
}

impl Window {
    /// Show window with the supplied widget and wait until it is closed.
    pub fn show<W: Widget>(self, widget: W) {
        window_thread(widget, self.theme, self.window_builder)
    }
}
