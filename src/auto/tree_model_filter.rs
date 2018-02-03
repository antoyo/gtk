// This file was generated by gir (0f1d1c1) from gir-files (77d1f70)
// DO NOT EDIT

use TreeIter;
use TreeModel;
use TreePath;
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
    pub struct TreeModelFilter(Object<ffi::GtkTreeModelFilter, ffi::GtkTreeModelFilterClass>): TreeModel;

    match fn {
        get_type => || ffi::gtk_tree_model_filter_get_type(),
    }
}

pub trait TreeModelFilterExt {
    fn clear_cache(&self);

    fn convert_child_iter_to_iter(&self, child_iter: &TreeIter) -> Option<TreeIter>;

    fn convert_child_path_to_path(&self, child_path: &TreePath) -> Option<TreePath>;

    fn convert_iter_to_child_iter(&self, filter_iter: &TreeIter) -> TreeIter;

    fn convert_path_to_child_path(&self, filter_path: &TreePath) -> Option<TreePath>;

    fn get_model(&self) -> Option<TreeModel>;

    fn refilter(&self);

    //fn set_modify_func<'a, P: Into<Option</*Unimplemented*/Fundamental: Pointer>>, Q: Into<Option<&'a /*Ignored*/glib::DestroyNotify>>>(&self, types: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 30 }, func: /*Unknown conversion*//*Unimplemented*/TreeModelFilterModifyFunc, data: P, destroy: Q);

    fn set_visible_column(&self, column: i32);

    //fn set_visible_func<'a, P: Into<Option</*Unimplemented*/Fundamental: Pointer>>, Q: Into<Option<&'a /*Ignored*/glib::DestroyNotify>>>(&self, func: /*Unknown conversion*//*Unimplemented*/TreeModelFilterVisibleFunc, data: P, destroy: Q);

    fn get_property_child_model(&self) -> Option<TreeModel>;

    fn connect_property_child_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TreeModelFilter> + IsA<glib::object::Object>> TreeModelFilterExt for O {
    fn clear_cache(&self) {
        unsafe {
            ffi::gtk_tree_model_filter_clear_cache(self.to_glib_none().0);
        }
    }

    fn convert_child_iter_to_iter(&self, child_iter: &TreeIter) -> Option<TreeIter> {
        unsafe {
            let mut filter_iter = TreeIter::uninitialized();
            let ret = from_glib(ffi::gtk_tree_model_filter_convert_child_iter_to_iter(self.to_glib_none().0, filter_iter.to_glib_none_mut().0, mut_override(child_iter.to_glib_none().0)));
            if ret { Some(filter_iter) } else { None }
        }
    }

    fn convert_child_path_to_path(&self, child_path: &TreePath) -> Option<TreePath> {
        unsafe {
            from_glib_full(ffi::gtk_tree_model_filter_convert_child_path_to_path(self.to_glib_none().0, mut_override(child_path.to_glib_none().0)))
        }
    }

    fn convert_iter_to_child_iter(&self, filter_iter: &TreeIter) -> TreeIter {
        unsafe {
            let mut child_iter = TreeIter::uninitialized();
            ffi::gtk_tree_model_filter_convert_iter_to_child_iter(self.to_glib_none().0, child_iter.to_glib_none_mut().0, mut_override(filter_iter.to_glib_none().0));
            child_iter
        }
    }

    fn convert_path_to_child_path(&self, filter_path: &TreePath) -> Option<TreePath> {
        unsafe {
            from_glib_full(ffi::gtk_tree_model_filter_convert_path_to_child_path(self.to_glib_none().0, mut_override(filter_path.to_glib_none().0)))
        }
    }

    fn get_model(&self) -> Option<TreeModel> {
        unsafe {
            from_glib_none(ffi::gtk_tree_model_filter_get_model(self.to_glib_none().0))
        }
    }

    fn refilter(&self) {
        unsafe {
            ffi::gtk_tree_model_filter_refilter(self.to_glib_none().0);
        }
    }

    //fn set_modify_func<'a, P: Into<Option</*Unimplemented*/Fundamental: Pointer>>, Q: Into<Option<&'a /*Ignored*/glib::DestroyNotify>>>(&self, types: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 30 }, func: /*Unknown conversion*//*Unimplemented*/TreeModelFilterModifyFunc, data: P, destroy: Q) {
    //    unsafe { TODO: call ffi::gtk_tree_model_filter_set_modify_func() }
    //}

    fn set_visible_column(&self, column: i32) {
        unsafe {
            ffi::gtk_tree_model_filter_set_visible_column(self.to_glib_none().0, column);
        }
    }

    //fn set_visible_func<'a, P: Into<Option</*Unimplemented*/Fundamental: Pointer>>, Q: Into<Option<&'a /*Ignored*/glib::DestroyNotify>>>(&self, func: /*Unknown conversion*//*Unimplemented*/TreeModelFilterVisibleFunc, data: P, destroy: Q) {
    //    unsafe { TODO: call ffi::gtk_tree_model_filter_set_visible_func() }
    //}

    fn get_property_child_model(&self) -> Option<TreeModel> {
        unsafe {
            let mut value = Value::from_type(<TreeModel as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "child-model".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn connect_property_child_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::child-model",
                transmute(notify_child_model_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_child_model_trampoline<P>(this: *mut ffi::GtkTreeModelFilter, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TreeModelFilter> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TreeModelFilter::from_glib_borrow(this).downcast_unchecked())
}
