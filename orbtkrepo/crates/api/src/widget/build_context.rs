use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use dces::prelude::{Component, ComponentBox, Entity, EntityComponentManager, SharedComponentBox};

use crate::{prelude::*, tree::Tree};

use super::State;

pub type WidgetBuildContext =
    Option<Box<dyn Fn(&mut BuildContext, usize) -> Entity + 'static>>;

/// Used to create an entity for a widget with its properties as components.
#[derive(Constructor)]
pub struct BuildContext<'a> {
    ecm: &'a mut EntityComponentManager<Tree, StringComponentStore>,
    render_objects: &'a RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>,
    layouts: &'a mut BTreeMap<Entity, Box<dyn Layout>>,
    handlers: &'a mut EventHandlerMap,
    states: &'a mut BTreeMap<Entity, Box<dyn State>>,
    theme: &'a ThemeValue,
}

impl<'a> BuildContext<'a> {
    /// Returns a specific widget.
    pub fn get_widget(&mut self, entity: Entity) -> WidgetContainer<'_> {
        WidgetContainer::new(entity, self.ecm, self.theme)
    }

    /// Creates a new entity.
    pub fn create_entity(&mut self) -> Entity {
        self.ecm.create_entity().build()
    }

    /// Appends a child to a parent.
    pub fn append_child(&mut self, parent: Entity, child: Entity) {
        self.ecm
            .entity_store_mut()
            .append_child(parent, child)
            .unwrap();
    }

    /// Registers a property as component.
    pub fn register_property<P: Component>(&mut self, key: &str, widget: Entity, property: P) {
        self.ecm
            .component_store_mut()
            .register(key, widget, property);
    }

    /// Registers a property box as component.
    pub fn register_property_box(&mut self, key: &str, widget: Entity, property: ComponentBox) {
        self.ecm
            .component_store_mut()
            .register_box(key, widget, property);
    }

    /// Registers a shared property. Uses the key as source key
    pub fn register_shared_property<P: Component>(
        &mut self,
        key: &str,
        target: Entity,
        source: Entity,
    ) {
        self.register_shared_property_by_source_key::<P>(key, key, target, source);
    }

    /// Registers a shared property.
    pub fn register_shared_property_by_source_key<P: Component>(
        &mut self,
        key: &str,
        source_key: &str,
        target: Entity,
        source: Entity,
    ) {
        self.ecm
            .component_store_mut()
            .register_shared_by_source_key::<P>(key, source_key, target, source);
    }

    /// Registers a shared component box. Uses the key as source key
    pub fn register_property_shared_box(
        &mut self,
        key: &str,
        widget: Entity,
        property: SharedComponentBox,
    ) {
        self.register_property_shared_box_by_source_key(key, key, widget, property);
    }

    /// Registers a shared component box.
    pub fn register_property_shared_box_by_source_key(
        &mut self,
        key: &str,
        source_key: &str,
        widget: Entity,
        property: SharedComponentBox,
    ) {
        self.ecm
            .component_store_mut()
            .register_shared_box_by_source_key(key, source_key, widget, property);
    }

    /// Registers a state with a widget.
    pub fn register_state(&mut self, widget: Entity, state: Box<dyn State>) {
        self.states.insert(widget, state);
    }

    /// Registers a render object with a widget.
    pub fn register_render_object(&mut self, widget: Entity, render_object: Box<dyn RenderObject>) {
        self.render_objects
            .borrow_mut()
            .insert(widget, render_object);
    }

    /// Registers a event handler with a widget.
    pub fn register_handler(&mut self, widget: Entity, handler: Rc<dyn EventHandler>) {
        if !self.handlers.contains_key(&widget) {
            self.handlers.insert(widget, vec![]);
        }

        self.handlers.get_mut(&widget).unwrap().push(handler);
    }

    /// Registers a layout object with a widget.
    pub fn register_layout(&mut self, widget: Entity, layout: Box<dyn Layout>) {
        self.layouts.insert(widget, layout);
    }
}
