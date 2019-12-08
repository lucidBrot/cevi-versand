//! This module contains all render objects used in OrbTk. Render objects are used to define how to draw parts of a widget.

use std::{any::Any, cell::RefCell, collections::BTreeMap, rc::Rc};

use crate::{css_engine::*, prelude::*, shell::WindowShell, utils::*};

pub use self::clear::*;
pub use self::default::*;
pub use self::font_icon::*;
pub use self::image::*;
pub use self::pipeline::*;
pub use self::rectangle::*;
pub use self::text::*;

mod clear;
mod default;
mod font_icon;
mod image;
mod pipeline;
mod rectangle;
mod text;

pub trait RenderObject: Any {
    fn render(
        &self,
        shell: &mut WindowShell<WindowAdapter>,
        entity: Entity,
        ecm: &mut EntityComponentManager<Tree, StringComponentStore>,
        render_objects: &RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>,
        layouts: &Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
        handlers: &Rc<RefCell<EventHandlerMap>>,
        states: &Rc<RefCell<BTreeMap<Entity, Box<dyn State>>>>,
        theme: &ThemeValue,
        offsets: &mut BTreeMap<Entity, (f64, f64)>,
        debug: bool,
    ) {
        let mut global_position = Point::default();

        if let Some(parent) = ecm.entity_store().parent[&entity] {
            if let Some(offset) = offsets.get(&parent) {
                global_position = Point::new(offset.0, offset.1);
            }
        }

        if let Ok(visibility) = ecm
            .component_store()
            .get::<Visibility>("visibility", entity)
        {
            if *visibility != Visibility::Visible {
                return;
            }
        } else {
            return;
        }

        shell.render_context_2_d().begin_path();
        shell.render_context_2_d().set_alpha(
            *ecm.component_store()
                .get::<f32>("opacity", entity)
                .unwrap_or(&1.0),
        );

        // Could be unwrap because every widget has the clip property
        let clip = *ecm.component_store().get::<bool>("clip", entity).unwrap();
        if clip {
            if let Ok(bounds) = ecm.component_store().get::<Rectangle>("bounds", entity) {
                shell.render_context_2_d().save();
                shell.render_context_2_d().rect(
                    global_position.x + bounds.x(),
                    global_position.y + bounds.y(),
                    bounds.width(),
                    bounds.height(),
                );
                shell.render_context_2_d().clip();
            }
        }

        self.render_self(
            &mut Context::new(
                (entity, ecm),
                shell,
                &theme,
                render_objects,
                &mut layouts.borrow_mut(),
                &mut handlers.borrow_mut(),
                &states,
                &mut BTreeMap::new(),
            ),
            &global_position,
        );

        let mut global_pos = (0.0, 0.0);

        if let Ok(bounds) = ecm.component_store().get::<Rectangle>("bounds", entity) {
            global_pos = (
                global_position.x + bounds.x(),
                global_position.y + bounds.y(),
            );
            offsets.insert(entity, global_pos);
        }

        if let Ok(g_pos) = ecm
            .component_store_mut()
            .get_mut::<Point>("position", entity)
        {
            g_pos.x = global_pos.0;
            g_pos.y = global_pos.1;
        }

        self.render_children(
            shell,
            entity,
            ecm,
            render_objects,
            layouts,
            handlers,
            states,
            theme,
            offsets,
            debug,
        );

        shell.render_context_2_d().close_path();

        if clip {
            shell.render_context_2_d().restore();
        }

        // render debug border for each widget
        if debug {
            if let Ok(bounds) = ecm.component_store().get::<Rectangle>("bounds", entity) {
                let selector = Selector::from("debug-border");
                let brush = theme.brush("border-color", &selector).unwrap();
                shell.render_context_2_d().begin_path();
                shell.render_context_2_d().set_stroke_style(brush);
                shell.render_context_2_d().stroke_rect(
                    global_position.x + bounds.x(),
                    global_position.y + bounds.y(),
                    bounds.width(),
                    bounds.height(),
                );
                shell.render_context_2_d().close_path();
            }
        }
    }

    fn render_self(&self, _: &mut Context<'_>, _: &Point) {}

    fn render_children(
        &self,
        shell: &mut WindowShell<WindowAdapter>,
        entity: Entity,
        ecm: &mut EntityComponentManager<Tree, StringComponentStore>,
        render_objects: &RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>,
        layouts: &Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
        handlers: &Rc<RefCell<EventHandlerMap>>,
        states: &Rc<RefCell<BTreeMap<Entity, Box<dyn State>>>>,
        theme: &ThemeValue,
        offsets: &mut BTreeMap<Entity, (f64, f64)>,
        debug: bool,
    ) {
        for index in 0..ecm.entity_store().children[&entity].len() {
            let child = ecm.entity_store().children[&entity][index];

            if let Some(render_object) = render_objects.borrow().get(&child) {
                render_object.render(
                    shell,
                    child,
                    ecm,
                    render_objects,
                    layouts,
                    handlers,
                    states,
                    theme,
                    offsets,
                    debug,
                );
            }
        }
    }
}
