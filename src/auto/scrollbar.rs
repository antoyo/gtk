// This file was generated by gir (https://github.com/gtk-rs/gir @ 6855214)
// from gir-files (https://github.com/gtk-rs/gir-files @ cf55bdc)
// DO NOT EDIT

use Adjustment;
use Buildable;
use Orientable;
use Orientation;
use Range;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Scrollbar(Object<ffi::GtkScrollbar, ffi::GtkScrollbarClass>): Range, Widget, Buildable, Orientable;

    match fn {
        get_type => || ffi::gtk_scrollbar_get_type(),
    }
}

impl Scrollbar {
    pub fn new<'a, P: Into<Option<&'a Adjustment>>>(orientation: Orientation, adjustment: P) -> Scrollbar {
        assert_initialized_main_thread!();
        let adjustment = adjustment.into();
        let adjustment = adjustment.to_glib_none();
        unsafe {
            Widget::from_glib_none(ffi::gtk_scrollbar_new(orientation.to_glib(), adjustment.0)).downcast_unchecked()
        }
    }
}
