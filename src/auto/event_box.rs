// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Bin;
use Buildable;
use Container;
use Widget;
use ffi;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct EventBox(Object<ffi::GtkEventBox, ffi::GtkEventBoxClass, EventBoxClass>) @extends Bin, Container, Widget, @implements Buildable;

    match fn {
        get_type => || ffi::gtk_event_box_get_type(),
    }
}

impl EventBox {
    pub fn new() -> EventBox {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_event_box_new()).unsafe_cast()
        }
    }
}

impl Default for EventBox {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_EVENT_BOX: Option<&EventBox> = None;

pub trait EventBoxExt: 'static {
    fn get_above_child(&self) -> bool;

    fn get_visible_window(&self) -> bool;

    fn set_above_child(&self, above_child: bool);

    fn set_visible_window(&self, visible_window: bool);

    fn connect_property_above_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_visible_window_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<EventBox>> EventBoxExt for O {
    fn get_above_child(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_event_box_get_above_child(self.as_ref().to_glib_none().0))
        }
    }

    fn get_visible_window(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_event_box_get_visible_window(self.as_ref().to_glib_none().0))
        }
    }

    fn set_above_child(&self, above_child: bool) {
        unsafe {
            ffi::gtk_event_box_set_above_child(self.as_ref().to_glib_none().0, above_child.to_glib());
        }
    }

    fn set_visible_window(&self, visible_window: bool) {
        unsafe {
            ffi::gtk_event_box_set_visible_window(self.as_ref().to_glib_none().0, visible_window.to_glib());
        }
    }

    fn connect_property_above_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::above-child\0".as_ptr() as *const _,
                Some(transmute(notify_above_child_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_visible_window_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::visible-window\0".as_ptr() as *const _,
                Some(transmute(notify_visible_window_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_above_child_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkEventBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<EventBox> {
    let f: &F = &*(f as *const F);
    f(&EventBox::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_visible_window_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkEventBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<EventBox> {
    let f: &F = &*(f as *const F);
    f(&EventBox::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for EventBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EventBox")
    }
}
