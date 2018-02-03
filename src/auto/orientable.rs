// This file was generated by gir (0f1d1c1) from gir-files (77d1f70)
// DO NOT EDIT

use Orientation;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Orientable(Object<ffi::GtkOrientable, ffi::GtkOrientableIface>);

    match fn {
        get_type => || ffi::gtk_orientable_get_type(),
    }
}

pub trait OrientableExt {
    fn get_orientation(&self) -> Orientation;

    fn set_orientation(&self, orientation: Orientation);

    fn connect_property_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Orientable> + IsA<glib::object::Object>> OrientableExt for O {
    fn get_orientation(&self) -> Orientation {
        unsafe {
            from_glib(ffi::gtk_orientable_get_orientation(self.to_glib_none().0))
        }
    }

    fn set_orientation(&self, orientation: Orientation) {
        unsafe {
            ffi::gtk_orientable_set_orientation(self.to_glib_none().0, orientation.to_glib());
        }
    }

    fn connect_property_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::orientation",
                transmute(notify_orientation_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_orientation_trampoline<P>(this: *mut ffi::GtkOrientable, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Orientable> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Orientable::from_glib_borrow(this).downcast_unchecked())
}
