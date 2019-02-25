// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

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
    pub struct Adjustment(Object<ffi::GtkAdjustment, ffi::GtkAdjustmentClass, AdjustmentClass>);

    match fn {
        get_type => || ffi::gtk_adjustment_get_type(),
    }
}

impl Adjustment {
    pub fn new(value: f64, lower: f64, upper: f64, step_increment: f64, page_increment: f64, page_size: f64) -> Adjustment {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_adjustment_new(value, lower, upper, step_increment, page_increment, page_size))
        }
    }
}

pub const NONE_ADJUSTMENT: Option<&Adjustment> = None;

pub trait AdjustmentExt: 'static {
    #[cfg_attr(feature = "v3_18", deprecated)]
    fn changed(&self);

    fn clamp_page(&self, lower: f64, upper: f64);

    fn configure(&self, value: f64, lower: f64, upper: f64, step_increment: f64, page_increment: f64, page_size: f64);

    fn get_lower(&self) -> f64;

    fn get_minimum_increment(&self) -> f64;

    fn get_page_increment(&self) -> f64;

    fn get_page_size(&self) -> f64;

    fn get_step_increment(&self) -> f64;

    fn get_upper(&self) -> f64;

    fn get_value(&self) -> f64;

    fn set_lower(&self, lower: f64);

    fn set_page_increment(&self, page_increment: f64);

    fn set_page_size(&self, page_size: f64);

    fn set_step_increment(&self, step_increment: f64);

    fn set_upper(&self, upper: f64);

    fn set_value(&self, value: f64);

    #[cfg_attr(feature = "v3_18", deprecated)]
    fn value_changed(&self);

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_value_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_lower_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_page_increment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_page_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_step_increment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_upper_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Adjustment>> AdjustmentExt for O {
    fn changed(&self) {
        unsafe {
            ffi::gtk_adjustment_changed(self.as_ref().to_glib_none().0);
        }
    }

    fn clamp_page(&self, lower: f64, upper: f64) {
        unsafe {
            ffi::gtk_adjustment_clamp_page(self.as_ref().to_glib_none().0, lower, upper);
        }
    }

    fn configure(&self, value: f64, lower: f64, upper: f64, step_increment: f64, page_increment: f64, page_size: f64) {
        unsafe {
            ffi::gtk_adjustment_configure(self.as_ref().to_glib_none().0, value, lower, upper, step_increment, page_increment, page_size);
        }
    }

    fn get_lower(&self) -> f64 {
        unsafe {
            ffi::gtk_adjustment_get_lower(self.as_ref().to_glib_none().0)
        }
    }

    fn get_minimum_increment(&self) -> f64 {
        unsafe {
            ffi::gtk_adjustment_get_minimum_increment(self.as_ref().to_glib_none().0)
        }
    }

    fn get_page_increment(&self) -> f64 {
        unsafe {
            ffi::gtk_adjustment_get_page_increment(self.as_ref().to_glib_none().0)
        }
    }

    fn get_page_size(&self) -> f64 {
        unsafe {
            ffi::gtk_adjustment_get_page_size(self.as_ref().to_glib_none().0)
        }
    }

    fn get_step_increment(&self) -> f64 {
        unsafe {
            ffi::gtk_adjustment_get_step_increment(self.as_ref().to_glib_none().0)
        }
    }

    fn get_upper(&self) -> f64 {
        unsafe {
            ffi::gtk_adjustment_get_upper(self.as_ref().to_glib_none().0)
        }
    }

    fn get_value(&self) -> f64 {
        unsafe {
            ffi::gtk_adjustment_get_value(self.as_ref().to_glib_none().0)
        }
    }

    fn set_lower(&self, lower: f64) {
        unsafe {
            ffi::gtk_adjustment_set_lower(self.as_ref().to_glib_none().0, lower);
        }
    }

    fn set_page_increment(&self, page_increment: f64) {
        unsafe {
            ffi::gtk_adjustment_set_page_increment(self.as_ref().to_glib_none().0, page_increment);
        }
    }

    fn set_page_size(&self, page_size: f64) {
        unsafe {
            ffi::gtk_adjustment_set_page_size(self.as_ref().to_glib_none().0, page_size);
        }
    }

    fn set_step_increment(&self, step_increment: f64) {
        unsafe {
            ffi::gtk_adjustment_set_step_increment(self.as_ref().to_glib_none().0, step_increment);
        }
    }

    fn set_upper(&self, upper: f64) {
        unsafe {
            ffi::gtk_adjustment_set_upper(self.as_ref().to_glib_none().0, upper);
        }
    }

    fn set_value(&self, value: f64) {
        unsafe {
            ffi::gtk_adjustment_set_value(self.as_ref().to_glib_none().0, value);
        }
    }

    fn value_changed(&self) {
        unsafe {
            ffi::gtk_adjustment_value_changed(self.as_ref().to_glib_none().0);
        }
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"changed\0".as_ptr() as *const _,
                Some(transmute(changed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_value_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"value-changed\0".as_ptr() as *const _,
                Some(transmute(value_changed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_lower_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::lower\0".as_ptr() as *const _,
                Some(transmute(notify_lower_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_page_increment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::page-increment\0".as_ptr() as *const _,
                Some(transmute(notify_page_increment_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_page_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::page-size\0".as_ptr() as *const _,
                Some(transmute(notify_page_size_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_step_increment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::step-increment\0".as_ptr() as *const _,
                Some(transmute(notify_step_increment_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_upper_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::upper\0".as_ptr() as *const _,
                Some(transmute(notify_upper_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::value\0".as_ptr() as *const _,
                Some(transmute(notify_value_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn changed_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkAdjustment, f: glib_ffi::gpointer)
where P: IsA<Adjustment> {
    let f: &F = &*(f as *const F);
    f(&Adjustment::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn value_changed_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkAdjustment, f: glib_ffi::gpointer)
where P: IsA<Adjustment> {
    let f: &F = &*(f as *const F);
    f(&Adjustment::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_lower_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkAdjustment, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Adjustment> {
    let f: &F = &*(f as *const F);
    f(&Adjustment::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_page_increment_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkAdjustment, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Adjustment> {
    let f: &F = &*(f as *const F);
    f(&Adjustment::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_page_size_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkAdjustment, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Adjustment> {
    let f: &F = &*(f as *const F);
    f(&Adjustment::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_step_increment_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkAdjustment, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Adjustment> {
    let f: &F = &*(f as *const F);
    f(&Adjustment::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_upper_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkAdjustment, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Adjustment> {
    let f: &F = &*(f as *const F);
    f(&Adjustment::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_value_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkAdjustment, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Adjustment> {
    let f: &F = &*(f as *const F);
    f(&Adjustment::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for Adjustment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Adjustment")
    }
}
