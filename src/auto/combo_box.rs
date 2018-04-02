// This file was generated by gir (https://github.com/gtk-rs/gir @ 6855214)
// from gir-files (https://github.com/gtk-rs/gir-files @ cf55bdc)
// DO NOT EDIT

use Bin;
use Buildable;
use CellArea;
use CellEditable;
use CellLayout;
use Container;
use ScrollType;
use SensitivityType;
use TreeIter;
use TreeModel;
use Widget;
use ffi;
use gdk;
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
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct ComboBox(Object<ffi::GtkComboBox, ffi::GtkComboBoxClass>): Bin, Container, Widget, Buildable, CellEditable, CellLayout;

    match fn {
        get_type => || ffi::gtk_combo_box_get_type(),
    }
}

impl ComboBox {
    pub fn new() -> ComboBox {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_combo_box_new()).downcast_unchecked()
        }
    }

    pub fn new_with_area<P: IsA<CellArea>>(area: &P) -> ComboBox {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_combo_box_new_with_area(area.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_area_and_entry<P: IsA<CellArea>>(area: &P) -> ComboBox {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_combo_box_new_with_area_and_entry(area.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_entry() -> ComboBox {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_combo_box_new_with_entry()).downcast_unchecked()
        }
    }

    pub fn new_with_model<P: IsA<TreeModel>>(model: &P) -> ComboBox {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_combo_box_new_with_model(model.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_model_and_entry<P: IsA<TreeModel>>(model: &P) -> ComboBox {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_combo_box_new_with_model_and_entry(model.to_glib_none().0)).downcast_unchecked()
        }
    }
}

impl Default for ComboBox {
    fn default() -> Self {
        Self::new()
    }
}

pub trait ComboBoxExt {
    fn get_active(&self) -> i32;

    fn get_active_id(&self) -> Option<String>;

    fn get_active_iter(&self) -> Option<TreeIter>;

    #[cfg_attr(feature = "v3_10", deprecated)]
    fn get_add_tearoffs(&self) -> bool;

    fn get_button_sensitivity(&self) -> SensitivityType;

    fn get_column_span_column(&self) -> i32;

    fn get_entry_text_column(&self) -> i32;

    #[cfg_attr(feature = "v3_20", deprecated)]
    #[cfg(any(not(feature = "v3_20"), feature = "dox"))]
    fn get_focus_on_click(&self) -> bool;

    fn get_has_entry(&self) -> bool;

    fn get_id_column(&self) -> i32;

    fn get_model(&self) -> Option<TreeModel>;

    //fn get_popup_accessible(&self) -> /*Ignored*/Option<atk::Object>;

    fn get_popup_fixed_width(&self) -> bool;

    //fn get_row_separator_func(&self) -> /*Unknown conversion*//*Unimplemented*/TreeViewRowSeparatorFunc;

    fn get_row_span_column(&self) -> i32;

    #[cfg_attr(feature = "v3_10", deprecated)]
    fn get_title(&self) -> Option<String>;

    fn get_wrap_width(&self) -> i32;

    fn popdown(&self);

    fn popup(&self);

    fn popup_for_device<P: IsA<gdk::Device>>(&self, device: &P);

    fn set_active(&self, index_: i32);

    fn set_active_id<'a, P: Into<Option<&'a str>>>(&self, active_id: P) -> bool;

    fn set_active_iter<'a, P: Into<Option<&'a TreeIter>>>(&self, iter: P);

    #[cfg_attr(feature = "v3_10", deprecated)]
    fn set_add_tearoffs(&self, add_tearoffs: bool);

    fn set_button_sensitivity(&self, sensitivity: SensitivityType);

    fn set_column_span_column(&self, column_span: i32);

    fn set_entry_text_column(&self, text_column: i32);

    #[cfg_attr(feature = "v3_20", deprecated)]
    #[cfg(any(not(feature = "v3_20"), feature = "dox"))]
    fn set_focus_on_click(&self, focus_on_click: bool);

    fn set_id_column(&self, id_column: i32);

    fn set_model<'a, P: IsA<TreeModel> + 'a, Q: Into<Option<&'a P>>>(&self, model: Q);

    fn set_popup_fixed_width(&self, fixed: bool);

    //fn set_row_separator_func<'a, P: Into<Option</*Unimplemented*/Fundamental: Pointer>>, Q: Into<Option<&'a /*Ignored*/glib::DestroyNotify>>>(&self, func: /*Unknown conversion*//*Unimplemented*/TreeViewRowSeparatorFunc, data: P, destroy: Q);

    fn set_row_span_column(&self, row_span: i32);

    #[cfg_attr(feature = "v3_10", deprecated)]
    fn set_title(&self, title: &str);

    fn set_wrap_width(&self, width: i32);

    fn get_property_cell_area(&self) -> Option<CellArea>;

    fn get_property_has_frame(&self) -> bool;

    fn set_property_has_frame(&self, has_frame: bool);

    fn get_property_popup_shown(&self) -> bool;

    #[cfg_attr(feature = "v3_10", deprecated)]
    fn get_property_tearoff_title(&self) -> Option<String>;

    #[cfg_attr(feature = "v3_10", deprecated)]
    fn set_property_tearoff_title(&self, tearoff_title: Option<&str>);

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_format_entry_text<F: Fn(&Self, &str) -> String + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_move_active<F: Fn(&Self, ScrollType) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_move_active(&self, scroll_type: ScrollType);

    fn connect_popdown<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_popdown(&self) -> bool;

    fn connect_popup<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_popup(&self);

    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_active_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg_attr(feature = "v3_10", deprecated)]
    fn connect_property_add_tearoffs_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_button_sensitivity_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_cell_area_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_column_span_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_entry_text_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_has_entry_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_has_frame_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_id_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_popup_fixed_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_popup_shown_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_row_span_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg_attr(feature = "v3_10", deprecated)]
    fn connect_property_tearoff_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_wrap_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ComboBox> + IsA<glib::object::Object> + glib::object::ObjectExt> ComboBoxExt for O {
    fn get_active(&self) -> i32 {
        unsafe {
            ffi::gtk_combo_box_get_active(self.to_glib_none().0)
        }
    }

    fn get_active_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_combo_box_get_active_id(self.to_glib_none().0))
        }
    }

    fn get_active_iter(&self) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ret = from_glib(ffi::gtk_combo_box_get_active_iter(self.to_glib_none().0, iter.to_glib_none_mut().0));
            if ret { Some(iter) } else { None }
        }
    }

    fn get_add_tearoffs(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_combo_box_get_add_tearoffs(self.to_glib_none().0))
        }
    }

    fn get_button_sensitivity(&self) -> SensitivityType {
        unsafe {
            from_glib(ffi::gtk_combo_box_get_button_sensitivity(self.to_glib_none().0))
        }
    }

    fn get_column_span_column(&self) -> i32 {
        unsafe {
            ffi::gtk_combo_box_get_column_span_column(self.to_glib_none().0)
        }
    }

    fn get_entry_text_column(&self) -> i32 {
        unsafe {
            ffi::gtk_combo_box_get_entry_text_column(self.to_glib_none().0)
        }
    }

    #[cfg(any(not(feature = "v3_20"), feature = "dox"))]
    fn get_focus_on_click(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_combo_box_get_focus_on_click(self.to_glib_none().0))
        }
    }

    fn get_has_entry(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_combo_box_get_has_entry(self.to_glib_none().0))
        }
    }

    fn get_id_column(&self) -> i32 {
        unsafe {
            ffi::gtk_combo_box_get_id_column(self.to_glib_none().0)
        }
    }

    fn get_model(&self) -> Option<TreeModel> {
        unsafe {
            from_glib_none(ffi::gtk_combo_box_get_model(self.to_glib_none().0))
        }
    }

    //fn get_popup_accessible(&self) -> /*Ignored*/Option<atk::Object> {
    //    unsafe { TODO: call ffi::gtk_combo_box_get_popup_accessible() }
    //}

    fn get_popup_fixed_width(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_combo_box_get_popup_fixed_width(self.to_glib_none().0))
        }
    }

    //fn get_row_separator_func(&self) -> /*Unknown conversion*//*Unimplemented*/TreeViewRowSeparatorFunc {
    //    unsafe { TODO: call ffi::gtk_combo_box_get_row_separator_func() }
    //}

    fn get_row_span_column(&self) -> i32 {
        unsafe {
            ffi::gtk_combo_box_get_row_span_column(self.to_glib_none().0)
        }
    }

    fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_combo_box_get_title(self.to_glib_none().0))
        }
    }

    fn get_wrap_width(&self) -> i32 {
        unsafe {
            ffi::gtk_combo_box_get_wrap_width(self.to_glib_none().0)
        }
    }

    fn popdown(&self) {
        unsafe {
            ffi::gtk_combo_box_popdown(self.to_glib_none().0);
        }
    }

    fn popup(&self) {
        unsafe {
            ffi::gtk_combo_box_popup(self.to_glib_none().0);
        }
    }

    fn popup_for_device<P: IsA<gdk::Device>>(&self, device: &P) {
        unsafe {
            ffi::gtk_combo_box_popup_for_device(self.to_glib_none().0, device.to_glib_none().0);
        }
    }

    fn set_active(&self, index_: i32) {
        unsafe {
            ffi::gtk_combo_box_set_active(self.to_glib_none().0, index_);
        }
    }

    fn set_active_id<'a, P: Into<Option<&'a str>>>(&self, active_id: P) -> bool {
        let active_id = active_id.into();
        let active_id = active_id.to_glib_none();
        unsafe {
            from_glib(ffi::gtk_combo_box_set_active_id(self.to_glib_none().0, active_id.0))
        }
    }

    fn set_active_iter<'a, P: Into<Option<&'a TreeIter>>>(&self, iter: P) {
        let iter = iter.into();
        unsafe {
            ffi::gtk_combo_box_set_active_iter(self.to_glib_none().0, mut_override(iter.to_glib_none().0));
        }
    }

    fn set_add_tearoffs(&self, add_tearoffs: bool) {
        unsafe {
            ffi::gtk_combo_box_set_add_tearoffs(self.to_glib_none().0, add_tearoffs.to_glib());
        }
    }

    fn set_button_sensitivity(&self, sensitivity: SensitivityType) {
        unsafe {
            ffi::gtk_combo_box_set_button_sensitivity(self.to_glib_none().0, sensitivity.to_glib());
        }
    }

    fn set_column_span_column(&self, column_span: i32) {
        unsafe {
            ffi::gtk_combo_box_set_column_span_column(self.to_glib_none().0, column_span);
        }
    }

    fn set_entry_text_column(&self, text_column: i32) {
        unsafe {
            ffi::gtk_combo_box_set_entry_text_column(self.to_glib_none().0, text_column);
        }
    }

    #[cfg(any(not(feature = "v3_20"), feature = "dox"))]
    fn set_focus_on_click(&self, focus_on_click: bool) {
        unsafe {
            ffi::gtk_combo_box_set_focus_on_click(self.to_glib_none().0, focus_on_click.to_glib());
        }
    }

    fn set_id_column(&self, id_column: i32) {
        unsafe {
            ffi::gtk_combo_box_set_id_column(self.to_glib_none().0, id_column);
        }
    }

    fn set_model<'a, P: IsA<TreeModel> + 'a, Q: Into<Option<&'a P>>>(&self, model: Q) {
        let model = model.into();
        let model = model.to_glib_none();
        unsafe {
            ffi::gtk_combo_box_set_model(self.to_glib_none().0, model.0);
        }
    }

    fn set_popup_fixed_width(&self, fixed: bool) {
        unsafe {
            ffi::gtk_combo_box_set_popup_fixed_width(self.to_glib_none().0, fixed.to_glib());
        }
    }

    //fn set_row_separator_func<'a, P: Into<Option</*Unimplemented*/Fundamental: Pointer>>, Q: Into<Option<&'a /*Ignored*/glib::DestroyNotify>>>(&self, func: /*Unknown conversion*//*Unimplemented*/TreeViewRowSeparatorFunc, data: P, destroy: Q) {
    //    unsafe { TODO: call ffi::gtk_combo_box_set_row_separator_func() }
    //}

    fn set_row_span_column(&self, row_span: i32) {
        unsafe {
            ffi::gtk_combo_box_set_row_span_column(self.to_glib_none().0, row_span);
        }
    }

    fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_combo_box_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    fn set_wrap_width(&self, width: i32) {
        unsafe {
            ffi::gtk_combo_box_set_wrap_width(self.to_glib_none().0, width);
        }
    }

    fn get_property_cell_area(&self) -> Option<CellArea> {
        unsafe {
            let mut value = Value::from_type(<CellArea as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "cell-area".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn get_property_has_frame(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "has-frame".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_has_frame(&self, has_frame: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "has-frame".to_glib_none().0, Value::from(&has_frame).to_glib_none().0);
        }
    }

    fn get_property_popup_shown(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "popup-shown".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_tearoff_title(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "tearoff-title".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_tearoff_title(&self, tearoff_title: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "tearoff-title".to_glib_none().0, Value::from(tearoff_title).to_glib_none().0);
        }
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "changed",
                transmute(changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_format_entry_text<F: Fn(&Self, &str) -> String + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &str) -> String + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "format-entry-text",
                transmute(format_entry_text_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_move_active<F: Fn(&Self, ScrollType) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, ScrollType) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "move-active",
                transmute(move_active_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_move_active(&self, scroll_type: ScrollType) {
        let _ = self.emit("move-active", &[&scroll_type]).unwrap();
    }

    fn connect_popdown<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "popdown",
                transmute(popdown_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_popdown(&self) -> bool {
        let res = self.emit("popdown", &[]).unwrap();
        res.unwrap().get().unwrap()
    }

    fn connect_popup<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "popup",
                transmute(popup_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_popup(&self) {
        let _ = self.emit("popup", &[]).unwrap();
    }

    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::active",
                transmute(notify_active_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_active_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::active-id",
                transmute(notify_active_id_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_add_tearoffs_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::add-tearoffs",
                transmute(notify_add_tearoffs_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_button_sensitivity_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::button-sensitivity",
                transmute(notify_button_sensitivity_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_cell_area_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::cell-area",
                transmute(notify_cell_area_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_column_span_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::column-span-column",
                transmute(notify_column_span_column_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_entry_text_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::entry-text-column",
                transmute(notify_entry_text_column_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_has_entry_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::has-entry",
                transmute(notify_has_entry_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_has_frame_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::has-frame",
                transmute(notify_has_frame_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_id_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::id-column",
                transmute(notify_id_column_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::model",
                transmute(notify_model_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_popup_fixed_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::popup-fixed-width",
                transmute(notify_popup_fixed_width_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_popup_shown_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::popup-shown",
                transmute(notify_popup_shown_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_row_span_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::row-span-column",
                transmute(notify_row_span_column_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_tearoff_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::tearoff-title",
                transmute(notify_tearoff_title_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_wrap_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::wrap-width",
                transmute(notify_wrap_width_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn changed_trampoline<P>(this: *mut ffi::GtkComboBox, f: glib_ffi::gpointer)
where P: IsA<ComboBox> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn format_entry_text_trampoline<P>(this: *mut ffi::GtkComboBox, path: *mut libc::c_char, f: glib_ffi::gpointer) -> *mut libc::c_char
where P: IsA<ComboBox> {
    callback_guard!();
    let f: &&(Fn(&P, &str) -> String + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).downcast_unchecked(), &String::from_glib_none(path)).to_glib_full()
}

unsafe extern "C" fn move_active_trampoline<P>(this: *mut ffi::GtkComboBox, scroll_type: ffi::GtkScrollType, f: glib_ffi::gpointer)
where P: IsA<ComboBox> {
    callback_guard!();
    let f: &&(Fn(&P, ScrollType) + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).downcast_unchecked(), from_glib(scroll_type))
}

unsafe extern "C" fn popdown_trampoline<P>(this: *mut ffi::GtkComboBox, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<ComboBox> {
    callback_guard!();
    let f: &&(Fn(&P) -> bool + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).downcast_unchecked()).to_glib()
}

unsafe extern "C" fn popup_trampoline<P>(this: *mut ffi::GtkComboBox, f: glib_ffi::gpointer)
where P: IsA<ComboBox> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_active_trampoline<P>(this: *mut ffi::GtkComboBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ComboBox> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_active_id_trampoline<P>(this: *mut ffi::GtkComboBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ComboBox> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_add_tearoffs_trampoline<P>(this: *mut ffi::GtkComboBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ComboBox> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_button_sensitivity_trampoline<P>(this: *mut ffi::GtkComboBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ComboBox> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_cell_area_trampoline<P>(this: *mut ffi::GtkComboBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ComboBox> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_column_span_column_trampoline<P>(this: *mut ffi::GtkComboBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ComboBox> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_entry_text_column_trampoline<P>(this: *mut ffi::GtkComboBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ComboBox> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_has_entry_trampoline<P>(this: *mut ffi::GtkComboBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ComboBox> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_has_frame_trampoline<P>(this: *mut ffi::GtkComboBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ComboBox> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_id_column_trampoline<P>(this: *mut ffi::GtkComboBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ComboBox> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_model_trampoline<P>(this: *mut ffi::GtkComboBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ComboBox> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_popup_fixed_width_trampoline<P>(this: *mut ffi::GtkComboBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ComboBox> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_popup_shown_trampoline<P>(this: *mut ffi::GtkComboBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ComboBox> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_row_span_column_trampoline<P>(this: *mut ffi::GtkComboBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ComboBox> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_tearoff_title_trampoline<P>(this: *mut ffi::GtkComboBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ComboBox> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_wrap_width_trampoline<P>(this: *mut ffi::GtkComboBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ComboBox> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ComboBox::from_glib_borrow(this).downcast_unchecked())
}
