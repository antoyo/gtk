// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use TextBuffer;
use ffi;
use glib::GString;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct TextMark(Object<ffi::GtkTextMark, ffi::GtkTextMarkClass, TextMarkClass>);

    match fn {
        get_type => || ffi::gtk_text_mark_get_type(),
    }
}

impl TextMark {
    pub fn new(name: Option<&str>, left_gravity: bool) -> TextMark {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_text_mark_new(name.to_glib_none().0, left_gravity.to_glib()))
        }
    }
}

pub const NONE_TEXT_MARK: Option<&TextMark> = None;

pub trait TextMarkExt: 'static {
    fn get_buffer(&self) -> Option<TextBuffer>;

    fn get_deleted(&self) -> bool;

    fn get_left_gravity(&self) -> bool;

    fn get_name(&self) -> Option<GString>;

    fn get_visible(&self) -> bool;

    fn set_visible(&self, setting: bool);
}

impl<O: IsA<TextMark>> TextMarkExt for O {
    fn get_buffer(&self) -> Option<TextBuffer> {
        unsafe {
            from_glib_none(ffi::gtk_text_mark_get_buffer(self.as_ref().to_glib_none().0))
        }
    }

    fn get_deleted(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_mark_get_deleted(self.as_ref().to_glib_none().0))
        }
    }

    fn get_left_gravity(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_mark_get_left_gravity(self.as_ref().to_glib_none().0))
        }
    }

    fn get_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::gtk_text_mark_get_name(self.as_ref().to_glib_none().0))
        }
    }

    fn get_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_mark_get_visible(self.as_ref().to_glib_none().0))
        }
    }

    fn set_visible(&self, setting: bool) {
        unsafe {
            ffi::gtk_text_mark_set_visible(self.as_ref().to_glib_none().0, setting.to_glib());
        }
    }
}

impl fmt::Display for TextMark {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TextMark")
    }
}
