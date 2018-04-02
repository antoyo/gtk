// This file was generated by gir (https://github.com/gtk-rs/gir @ 6855214)
// from gir-files (https://github.com/gtk-rs/gir-files @ cf55bdc)
// DO NOT EDIT

use Adjustment;
use CellRenderer;
use CellRendererText;
use ffi;
use glib;
use glib::StaticType;
use glib::Value;
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
    pub struct CellRendererSpin(Object<ffi::GtkCellRendererSpin, ffi::GtkCellRendererSpinClass>): CellRendererText, CellRenderer;

    match fn {
        get_type => || ffi::gtk_cell_renderer_spin_get_type(),
    }
}

impl CellRendererSpin {
    pub fn new() -> CellRendererSpin {
        assert_initialized_main_thread!();
        unsafe {
            CellRenderer::from_glib_none(ffi::gtk_cell_renderer_spin_new()).downcast_unchecked()
        }
    }
}

impl Default for CellRendererSpin {
    fn default() -> Self {
        Self::new()
    }
}

pub trait CellRendererSpinExt {
    fn get_property_adjustment(&self) -> Option<Adjustment>;

    fn set_property_adjustment(&self, adjustment: Option<&Adjustment>);

    fn get_property_climb_rate(&self) -> f64;

    fn set_property_climb_rate(&self, climb_rate: f64);

    fn get_property_digits(&self) -> u32;

    fn set_property_digits(&self, digits: u32);

    fn connect_property_adjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_climb_rate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_digits_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CellRendererSpin> + IsA<glib::object::Object>> CellRendererSpinExt for O {
    fn get_property_adjustment(&self) -> Option<Adjustment> {
        unsafe {
            let mut value = Value::from_type(<Adjustment as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "adjustment".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_adjustment(&self, adjustment: Option<&Adjustment>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "adjustment".to_glib_none().0, Value::from(adjustment).to_glib_none().0);
        }
    }

    fn get_property_climb_rate(&self) -> f64 {
        unsafe {
            let mut value = Value::from_type(<f64 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "climb-rate".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_climb_rate(&self, climb_rate: f64) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "climb-rate".to_glib_none().0, Value::from(&climb_rate).to_glib_none().0);
        }
    }

    fn get_property_digits(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "digits".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_digits(&self, digits: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "digits".to_glib_none().0, Value::from(&digits).to_glib_none().0);
        }
    }

    fn connect_property_adjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::adjustment",
                transmute(notify_adjustment_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_climb_rate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::climb-rate",
                transmute(notify_climb_rate_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_digits_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::digits",
                transmute(notify_digits_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_adjustment_trampoline<P>(this: *mut ffi::GtkCellRendererSpin, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererSpin> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CellRendererSpin::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_climb_rate_trampoline<P>(this: *mut ffi::GtkCellRendererSpin, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererSpin> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CellRendererSpin::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_digits_trampoline<P>(this: *mut ffi::GtkCellRendererSpin, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererSpin> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CellRendererSpin::from_glib_borrow(this).downcast_unchecked())
}
