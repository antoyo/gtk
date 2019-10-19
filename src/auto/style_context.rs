// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::ToValue;
use glib::Value;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Border;
use CssSection;
use JunctionSides;
use StateFlags;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use StyleContextPrintFlags;
use StyleProvider;
use TextDirection;
use WidgetPath;

glib_wrapper! {
    pub struct StyleContext(Object<gtk_sys::GtkStyleContext, gtk_sys::GtkStyleContextClass, StyleContextClass>);

    match fn {
        get_type => || gtk_sys::gtk_style_context_get_type(),
    }
}

impl StyleContext {
    pub fn new() -> StyleContext {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(gtk_sys::gtk_style_context_new()) }
    }

    pub fn add_provider_for_screen<P: IsA<StyleProvider>>(
        screen: &gdk::Screen,
        provider: &P,
        priority: u32,
    ) {
        skip_assert_initialized!();
        unsafe {
            gtk_sys::gtk_style_context_add_provider_for_screen(
                screen.to_glib_none().0,
                provider.as_ref().to_glib_none().0,
                priority,
            );
        }
    }

    pub fn remove_provider_for_screen<P: IsA<StyleProvider>>(screen: &gdk::Screen, provider: &P) {
        skip_assert_initialized!();
        unsafe {
            gtk_sys::gtk_style_context_remove_provider_for_screen(
                screen.to_glib_none().0,
                provider.as_ref().to_glib_none().0,
            );
        }
    }

    pub fn reset_widgets(screen: &gdk::Screen) {
        assert_initialized_main_thread!();
        unsafe {
            gtk_sys::gtk_style_context_reset_widgets(screen.to_glib_none().0);
        }
    }
}

impl Default for StyleContext {
    fn default() -> Self {
        Self::new()
    }
}

pub struct StyleContextBuilder {
    direction: Option<TextDirection>,
    paint_clock: Option<gdk::FrameClock>,
    parent: Option<StyleContext>,
    screen: Option<gdk::Screen>,
}

impl StyleContextBuilder {
    pub fn new() -> Self {
        Self {
            direction: None,
            paint_clock: None,
            parent: None,
            screen: None,
        }
    }

    pub fn build(self) -> StyleContext {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref direction) = self.direction {
            properties.push(("direction", direction));
        }
        if let Some(ref paint_clock) = self.paint_clock {
            properties.push(("paint-clock", paint_clock));
        }
        if let Some(ref parent) = self.parent {
            properties.push(("parent", parent));
        }
        if let Some(ref screen) = self.screen {
            properties.push(("screen", screen));
        }
        glib::Object::new(StyleContext::static_type(), &properties)
            .expect("object new")
            .downcast()
            .expect("downcast")
    }

    pub fn direction(mut self, direction: TextDirection) -> Self {
        self.direction = Some(direction);
        self
    }

    pub fn paint_clock(mut self, paint_clock: &gdk::FrameClock) -> Self {
        self.paint_clock = Some(paint_clock.clone());
        self
    }

    pub fn parent<P: IsA<StyleContext>>(mut self, parent: &P) -> Self {
        self.parent = Some(parent.clone().upcast());
        self
    }

    pub fn screen(mut self, screen: &gdk::Screen) -> Self {
        self.screen = Some(screen.clone());
        self
    }
}

pub const NONE_STYLE_CONTEXT: Option<&StyleContext> = None;

pub trait StyleContextExt: 'static {
    fn add_class(&self, class_name: &str);

    fn add_provider<P: IsA<StyleProvider>>(&self, provider: &P, priority: u32);

    //fn get(&self, state: StateFlags, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    #[cfg_attr(feature = "v3_16", deprecated)]
    fn get_background_color(&self, state: StateFlags) -> gdk::RGBA;

    fn get_border(&self, state: StateFlags) -> Border;

    #[cfg_attr(feature = "v3_16", deprecated)]
    fn get_border_color(&self, state: StateFlags) -> gdk::RGBA;

    fn get_color(&self, state: StateFlags) -> gdk::RGBA;

    fn get_frame_clock(&self) -> Option<gdk::FrameClock>;

    fn get_junction_sides(&self) -> JunctionSides;

    fn get_margin(&self, state: StateFlags) -> Border;

    fn get_padding(&self, state: StateFlags) -> Border;

    fn get_parent(&self) -> Option<StyleContext>;

    fn get_path(&self) -> Option<WidgetPath>;

    fn get_scale(&self) -> i32;

    fn get_screen(&self) -> Option<gdk::Screen>;

    fn get_section(&self, property: &str) -> Option<CssSection>;

    fn get_state(&self) -> StateFlags;

    //fn get_style(&self, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    //fn get_style_valist(&self, args: /*Unknown conversion*//*Unimplemented*/Unsupported);

    //fn get_valist(&self, state: StateFlags, args: /*Unknown conversion*//*Unimplemented*/Unsupported);

    fn has_class(&self, class_name: &str) -> bool;

    fn list_classes(&self) -> Vec<GString>;

    fn lookup_color(&self, color_name: &str) -> Option<gdk::RGBA>;

    fn remove_class(&self, class_name: &str);

    fn remove_provider<P: IsA<StyleProvider>>(&self, provider: &P);

    fn restore(&self);

    fn save(&self);

    #[cfg_attr(feature = "v3_18", deprecated)]
    fn set_background<P: IsA<gdk::Window>>(&self, window: &P);

    fn set_frame_clock(&self, frame_clock: &gdk::FrameClock);

    fn set_junction_sides(&self, sides: JunctionSides);

    fn set_parent<P: IsA<StyleContext>>(&self, parent: Option<&P>);

    fn set_path(&self, path: &WidgetPath);

    fn set_scale(&self, scale: i32);

    fn set_screen(&self, screen: &gdk::Screen);

    fn set_state(&self, flags: StateFlags);

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn to_string(&self, flags: StyleContextPrintFlags) -> GString;

    fn get_property_direction(&self) -> TextDirection;

    fn set_property_direction(&self, direction: TextDirection);

    fn get_property_paint_clock(&self) -> Option<gdk::FrameClock>;

    fn set_property_paint_clock(&self, paint_clock: Option<&gdk::FrameClock>);

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_paint_clock_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_screen_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<StyleContext>> StyleContextExt for O {
    fn add_class(&self, class_name: &str) {
        unsafe {
            gtk_sys::gtk_style_context_add_class(
                self.as_ref().to_glib_none().0,
                class_name.to_glib_none().0,
            );
        }
    }

    fn add_provider<P: IsA<StyleProvider>>(&self, provider: &P, priority: u32) {
        unsafe {
            gtk_sys::gtk_style_context_add_provider(
                self.as_ref().to_glib_none().0,
                provider.as_ref().to_glib_none().0,
                priority,
            );
        }
    }

    //fn get(&self, state: StateFlags, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call gtk_sys:gtk_style_context_get() }
    //}

    fn get_background_color(&self, state: StateFlags) -> gdk::RGBA {
        unsafe {
            let mut color = gdk::RGBA::uninitialized();
            gtk_sys::gtk_style_context_get_background_color(
                self.as_ref().to_glib_none().0,
                state.to_glib(),
                color.to_glib_none_mut().0,
            );
            color
        }
    }

    fn get_border(&self, state: StateFlags) -> Border {
        unsafe {
            let mut border = Border::uninitialized();
            gtk_sys::gtk_style_context_get_border(
                self.as_ref().to_glib_none().0,
                state.to_glib(),
                border.to_glib_none_mut().0,
            );
            border
        }
    }

    fn get_border_color(&self, state: StateFlags) -> gdk::RGBA {
        unsafe {
            let mut color = gdk::RGBA::uninitialized();
            gtk_sys::gtk_style_context_get_border_color(
                self.as_ref().to_glib_none().0,
                state.to_glib(),
                color.to_glib_none_mut().0,
            );
            color
        }
    }

    fn get_color(&self, state: StateFlags) -> gdk::RGBA {
        unsafe {
            let mut color = gdk::RGBA::uninitialized();
            gtk_sys::gtk_style_context_get_color(
                self.as_ref().to_glib_none().0,
                state.to_glib(),
                color.to_glib_none_mut().0,
            );
            color
        }
    }

    fn get_frame_clock(&self) -> Option<gdk::FrameClock> {
        unsafe {
            from_glib_none(gtk_sys::gtk_style_context_get_frame_clock(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_junction_sides(&self) -> JunctionSides {
        unsafe {
            from_glib(gtk_sys::gtk_style_context_get_junction_sides(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_margin(&self, state: StateFlags) -> Border {
        unsafe {
            let mut margin = Border::uninitialized();
            gtk_sys::gtk_style_context_get_margin(
                self.as_ref().to_glib_none().0,
                state.to_glib(),
                margin.to_glib_none_mut().0,
            );
            margin
        }
    }

    fn get_padding(&self, state: StateFlags) -> Border {
        unsafe {
            let mut padding = Border::uninitialized();
            gtk_sys::gtk_style_context_get_padding(
                self.as_ref().to_glib_none().0,
                state.to_glib(),
                padding.to_glib_none_mut().0,
            );
            padding
        }
    }

    fn get_parent(&self) -> Option<StyleContext> {
        unsafe {
            from_glib_none(gtk_sys::gtk_style_context_get_parent(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_path(&self) -> Option<WidgetPath> {
        unsafe {
            from_glib_none(gtk_sys::gtk_style_context_get_path(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_scale(&self) -> i32 {
        unsafe { gtk_sys::gtk_style_context_get_scale(self.as_ref().to_glib_none().0) }
    }

    fn get_screen(&self) -> Option<gdk::Screen> {
        unsafe {
            from_glib_none(gtk_sys::gtk_style_context_get_screen(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_section(&self, property: &str) -> Option<CssSection> {
        unsafe {
            from_glib_none(gtk_sys::gtk_style_context_get_section(
                self.as_ref().to_glib_none().0,
                property.to_glib_none().0,
            ))
        }
    }

    fn get_state(&self) -> StateFlags {
        unsafe {
            from_glib(gtk_sys::gtk_style_context_get_state(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn get_style(&self, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call gtk_sys:gtk_style_context_get_style() }
    //}

    //fn get_style_valist(&self, args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call gtk_sys:gtk_style_context_get_style_valist() }
    //}

    //fn get_valist(&self, state: StateFlags, args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call gtk_sys:gtk_style_context_get_valist() }
    //}

    fn has_class(&self, class_name: &str) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_style_context_has_class(
                self.as_ref().to_glib_none().0,
                class_name.to_glib_none().0,
            ))
        }
    }

    fn list_classes(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(gtk_sys::gtk_style_context_list_classes(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn lookup_color(&self, color_name: &str) -> Option<gdk::RGBA> {
        unsafe {
            let mut color = gdk::RGBA::uninitialized();
            let ret = from_glib(gtk_sys::gtk_style_context_lookup_color(
                self.as_ref().to_glib_none().0,
                color_name.to_glib_none().0,
                color.to_glib_none_mut().0,
            ));
            if ret {
                Some(color)
            } else {
                None
            }
        }
    }

    fn remove_class(&self, class_name: &str) {
        unsafe {
            gtk_sys::gtk_style_context_remove_class(
                self.as_ref().to_glib_none().0,
                class_name.to_glib_none().0,
            );
        }
    }

    fn remove_provider<P: IsA<StyleProvider>>(&self, provider: &P) {
        unsafe {
            gtk_sys::gtk_style_context_remove_provider(
                self.as_ref().to_glib_none().0,
                provider.as_ref().to_glib_none().0,
            );
        }
    }

    fn restore(&self) {
        unsafe {
            gtk_sys::gtk_style_context_restore(self.as_ref().to_glib_none().0);
        }
    }

    fn save(&self) {
        unsafe {
            gtk_sys::gtk_style_context_save(self.as_ref().to_glib_none().0);
        }
    }

    fn set_background<P: IsA<gdk::Window>>(&self, window: &P) {
        unsafe {
            gtk_sys::gtk_style_context_set_background(
                self.as_ref().to_glib_none().0,
                window.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_frame_clock(&self, frame_clock: &gdk::FrameClock) {
        unsafe {
            gtk_sys::gtk_style_context_set_frame_clock(
                self.as_ref().to_glib_none().0,
                frame_clock.to_glib_none().0,
            );
        }
    }

    fn set_junction_sides(&self, sides: JunctionSides) {
        unsafe {
            gtk_sys::gtk_style_context_set_junction_sides(
                self.as_ref().to_glib_none().0,
                sides.to_glib(),
            );
        }
    }

    fn set_parent<P: IsA<StyleContext>>(&self, parent: Option<&P>) {
        unsafe {
            gtk_sys::gtk_style_context_set_parent(
                self.as_ref().to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_path(&self, path: &WidgetPath) {
        unsafe {
            gtk_sys::gtk_style_context_set_path(
                self.as_ref().to_glib_none().0,
                path.to_glib_none().0,
            );
        }
    }

    fn set_scale(&self, scale: i32) {
        unsafe {
            gtk_sys::gtk_style_context_set_scale(self.as_ref().to_glib_none().0, scale);
        }
    }

    fn set_screen(&self, screen: &gdk::Screen) {
        unsafe {
            gtk_sys::gtk_style_context_set_screen(
                self.as_ref().to_glib_none().0,
                screen.to_glib_none().0,
            );
        }
    }

    fn set_state(&self, flags: StateFlags) {
        unsafe {
            gtk_sys::gtk_style_context_set_state(self.as_ref().to_glib_none().0, flags.to_glib());
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn to_string(&self, flags: StyleContextPrintFlags) -> GString {
        unsafe {
            from_glib_full(gtk_sys::gtk_style_context_to_string(
                self.as_ref().to_glib_none().0,
                flags.to_glib(),
            ))
        }
    }

    fn get_property_direction(&self) -> TextDirection {
        unsafe {
            let mut value = Value::from_type(<TextDirection as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"direction\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `direction` getter")
                .unwrap()
        }
    }

    fn set_property_direction(&self, direction: TextDirection) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"direction\0".as_ptr() as *const _,
                Value::from(&direction).to_glib_none().0,
            );
        }
    }

    fn get_property_paint_clock(&self) -> Option<gdk::FrameClock> {
        unsafe {
            let mut value = Value::from_type(<gdk::FrameClock as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"paint-clock\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `paint-clock` getter")
        }
    }

    fn set_property_paint_clock(&self, paint_clock: Option<&gdk::FrameClock>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"paint-clock\0".as_ptr() as *const _,
                Value::from(paint_clock).to_glib_none().0,
            );
        }
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkStyleContext,
            f: glib_sys::gpointer,
        ) where
            P: IsA<StyleContext>,
        {
            let f: &F = &*(f as *const F);
            f(&StyleContext::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"changed\0".as_ptr() as *const _,
                Some(transmute(changed_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_direction_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkStyleContext,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<StyleContext>,
        {
            let f: &F = &*(f as *const F);
            f(&StyleContext::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::direction\0".as_ptr() as *const _,
                Some(transmute(notify_direction_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_paint_clock_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_paint_clock_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkStyleContext,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<StyleContext>,
        {
            let f: &F = &*(f as *const F);
            f(&StyleContext::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::paint-clock\0".as_ptr() as *const _,
                Some(transmute(notify_paint_clock_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_parent_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkStyleContext,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<StyleContext>,
        {
            let f: &F = &*(f as *const F);
            f(&StyleContext::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::parent\0".as_ptr() as *const _,
                Some(transmute(notify_parent_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_screen_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_screen_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkStyleContext,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<StyleContext>,
        {
            let f: &F = &*(f as *const F);
            f(&StyleContext::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::screen\0".as_ptr() as *const _,
                Some(transmute(notify_screen_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for StyleContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StyleContext")
    }
}
