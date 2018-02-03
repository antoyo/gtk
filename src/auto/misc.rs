// This file was generated by gir (0f1d1c1) from gir-files (77d1f70)
// DO NOT EDIT

use Buildable;
use Widget;
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
    pub struct Misc(Object<ffi::GtkMisc, ffi::GtkMiscClass>): Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_misc_get_type(),
    }
}

pub trait MiscExt {
    #[cfg_attr(feature = "v3_14", deprecated)]
    fn get_alignment(&self) -> (f32, f32);

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn get_padding(&self) -> (i32, i32);

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn set_alignment(&self, xalign: f32, yalign: f32);

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn set_padding(&self, xpad: i32, ypad: i32);

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn get_property_xalign(&self) -> f32;

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn set_property_xalign(&self, xalign: f32);

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn get_property_xpad(&self) -> i32;

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn set_property_xpad(&self, xpad: i32);

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn get_property_yalign(&self) -> f32;

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn set_property_yalign(&self, yalign: f32);

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn get_property_ypad(&self) -> i32;

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn set_property_ypad(&self, ypad: i32);

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn connect_property_xalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn connect_property_xpad_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn connect_property_yalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn connect_property_ypad_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Misc> + IsA<glib::object::Object>> MiscExt for O {
    fn get_alignment(&self) -> (f32, f32) {
        unsafe {
            let mut xalign = mem::uninitialized();
            let mut yalign = mem::uninitialized();
            ffi::gtk_misc_get_alignment(self.to_glib_none().0, &mut xalign, &mut yalign);
            (xalign, yalign)
        }
    }

    fn get_padding(&self) -> (i32, i32) {
        unsafe {
            let mut xpad = mem::uninitialized();
            let mut ypad = mem::uninitialized();
            ffi::gtk_misc_get_padding(self.to_glib_none().0, &mut xpad, &mut ypad);
            (xpad, ypad)
        }
    }

    fn set_alignment(&self, xalign: f32, yalign: f32) {
        unsafe {
            ffi::gtk_misc_set_alignment(self.to_glib_none().0, xalign, yalign);
        }
    }

    fn set_padding(&self, xpad: i32, ypad: i32) {
        unsafe {
            ffi::gtk_misc_set_padding(self.to_glib_none().0, xpad, ypad);
        }
    }

    fn get_property_xalign(&self) -> f32 {
        unsafe {
            let mut value = Value::from_type(<f32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "xalign".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_xalign(&self, xalign: f32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "xalign".to_glib_none().0, Value::from(&xalign).to_glib_none().0);
        }
    }

    fn get_property_xpad(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "xpad".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_xpad(&self, xpad: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "xpad".to_glib_none().0, Value::from(&xpad).to_glib_none().0);
        }
    }

    fn get_property_yalign(&self) -> f32 {
        unsafe {
            let mut value = Value::from_type(<f32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "yalign".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_yalign(&self, yalign: f32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "yalign".to_glib_none().0, Value::from(&yalign).to_glib_none().0);
        }
    }

    fn get_property_ypad(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "ypad".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_ypad(&self, ypad: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "ypad".to_glib_none().0, Value::from(&ypad).to_glib_none().0);
        }
    }

    fn connect_property_xalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::xalign",
                transmute(notify_xalign_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_xpad_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::xpad",
                transmute(notify_xpad_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_yalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::yalign",
                transmute(notify_yalign_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_ypad_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::ypad",
                transmute(notify_ypad_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_xalign_trampoline<P>(this: *mut ffi::GtkMisc, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Misc> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Misc::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_xpad_trampoline<P>(this: *mut ffi::GtkMisc, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Misc> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Misc::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_yalign_trampoline<P>(this: *mut ffi::GtkMisc, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Misc> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Misc::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_ypad_trampoline<P>(this: *mut ffi::GtkMisc, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Misc> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Misc::from_glib_borrow(this).downcast_unchecked())
}
