use eframe::egui;
use egui::{Checkbox, DragValue, Ui, Widget, WidgetText};

use crate::{
    ps2,
    types::{Camera, Katamari, Mat4, MotionVectors, Prince, Prop, Vec4},
    ui::definition_file::{DefinitionFile, Field, FieldType},
};

impl Field {
    pub fn label(&self) -> String {
        self.name
            .clone()
            .unwrap_or_else(|| format!("{:#x}", self.offset))
    }

    pub unsafe fn ui<T: 'static>(&self, data: *mut T, ui: &mut Ui) -> egui::Response {
        if std::any::TypeId::of::<T>() != std::any::TypeId::of::<()>() {
            assert!(self.offset < std::mem::size_of::<T>());
        }

        ui.horizontal(|ui| {
            ui.label(self.label());

            unsafe {
                let field = data.cast::<()>().byte_offset(self.offset as isize);
                self.field_type.ui(data, field, ui);
            }
        })
        .response
    }
}

impl FieldType {
    pub unsafe fn ui<P>(&self, parent: *mut P, field: *mut (), ui: &mut Ui) -> egui::Response {
        unsafe fn drag_value<'a, T: egui::emath::Numeric>(ptr: *mut ()) -> DragValue<'a> {
            unsafe {
                let val: &mut T = &mut *ptr.cast::<T>();
                DragValue::new(val)
            }
        }

        unsafe {
            match self {
                FieldType::Float => drag_value::<f32>(field).ui(ui),
                FieldType::Vec4 => (&mut *field.cast::<Vec4>()).ui(ui),
                FieldType::Bool => Checkbox::without_text(&mut *field.cast::<bool>()).ui(ui),
                FieldType::Int8 { signed: false } => drag_value::<u8>(field).ui(ui),
                FieldType::Int8 { signed: true } => drag_value::<i8>(field).ui(ui),
                FieldType::Int16 { signed: false } => drag_value::<u16>(field).ui(ui),
                FieldType::Int16 { signed: true } => drag_value::<i16>(field).ui(ui),
                FieldType::Int32 { signed: false } => drag_value::<u32>(field).ui(ui),
                FieldType::Int32 { signed: true } => drag_value::<i32>(field).ui(ui),
                FieldType::Int64 { signed: false } => drag_value::<u64>(field).ui(ui),
                FieldType::Int64 { signed: true } => drag_value::<i64>(field).ui(ui),
                FieldType::Address => ui.label(identify_addr(parent, *field.cast::<*mut ()>())),
                FieldType::Mat4 => {
                    let m = &mut *field.cast::<Mat4>();
                    let resp = egui::Grid::new("grib").show(ui, |ui| {
                        for row in &mut m.rows {
                            DragValue::new(&mut row.x).ui(ui);
                            DragValue::new(&mut row.y).ui(ui);
                            DragValue::new(&mut row.z).ui(ui);
                            DragValue::new(&mut row.w).ui(ui);
                            ui.end_row();
                        }
                    });
                    resp.response
                }
                FieldType::MotionVectors => {
                    let motion_vectors: &mut MotionVectors = &mut *field.cast();
                    ui.vertical(|ui| motion_vectors.ui(ui)).response
                }
                FieldType::Array(inner, len) => {
                    let stride = inner.size() as isize;
                    let mut elem = field;
                    for _ in 0..*len {
                        inner.ui(parent, elem, ui);
                        elem = elem.byte_offset(stride);
                    }
                    ui.response()
                }
            }
        }
    }
}

fn identify_addr<T>(parent: *mut T, ptr: *mut ()) -> String {
    if ptr.is_null() {
        return "NULL".into();
    }

    if let Some(n) = ptr.addr().checked_sub(parent.addr())
        && n < std::mem::size_of::<T>()
    {
        return format!("self + {n:#x}");
    }

    let arrays: &[(&dyn AmITheParent, &str)] = &[
        (&ps2::raw::katamari_array(), "KATAMARI"),
        (&ps2::raw::prop_array(), "THINGS"),
        (&ps2::raw::prop_data_array(), "THING_DATA"),
    ];

    for (array, name) in arrays {
        if let Some((index, offset)) = array.check_for_child(ptr.cast()) {
            return format!("{name}[{index}] + {offset:#x}");
        }
    }

    if let Some(offset) = ptr.addr().checked_sub(ps2::raw::base_addr().addr()) {
        return format!("DLL + {offset:#X}");
    }

    return format!("{ptr:?}");
}

trait AmITheParent {
    fn check_for_child(&self, ptr: *mut ()) -> Option<(usize, usize)>;
}

impl<T, const N: usize> AmITheParent for *mut [T; N] {
    fn check_for_child(&self, ptr: *mut ()) -> Option<(usize, usize)> {
        let ptr = ptr.addr();
        let base = (*self).addr();

        let offset = ptr.checked_sub(base)?;

        if offset >= std::mem::size_of::<[T; N]>() {
            return None;
        }
        let elem_size = std::mem::size_of::<T>();
        Some((offset / elem_size, offset % elem_size))
    }
}

pub trait WidgetExt: Widget + Sized {
    fn label_ui(self, ui: &mut Ui, label: impl Into<WidgetText>) -> egui::Response {
        ui.horizontal(|ui| {
            ui.label(label);
            self.ui(ui);
        })
        .response
    }
}

impl<T: Widget> WidgetExt for T {}

impl Widget for &mut Vec4 {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        ui.horizontal(|ui| {
            DragValue::new(&mut self.x).label_ui(ui, "x");
            DragValue::new(&mut self.y).label_ui(ui, "y");
            DragValue::new(&mut self.z).label_ui(ui, "z");
            DragValue::new(&mut self.w).label_ui(ui, "w");
        })
        .response
    }
}

impl Widget for &mut MotionVectors {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        ui.vertical(|ui| {
            for (vec, label) in self.as_slice_mut().iter_mut().zip('A'..) {
                vec.label_ui(ui, label.to_string());
            }
        })
        .response
    }
}

pub unsafe trait Research: Sized + 'static {
    fn get_section(defs: &DefinitionFile) -> &[Field];

    unsafe fn fields(&mut self, ui: &mut Ui, defs: &DefinitionFile) -> egui::Response {
        self.extra_before(ui);
        // ui.label(format!("{:#?}", &raw mut *self));
        for field in Self::get_section(defs) {
            unsafe {
                field.ui::<Self>(&raw mut *self, ui);
            }
        }
        self.extra_after(ui);
        ui.response()
    }

    fn extra_before(&mut self, _ui: &mut Ui) {}
    fn extra_after(&mut self, _ui: &mut Ui) {}

    fn with<'a>(&mut self, defs: &'a DefinitionFile) -> WithDefinitions<'a, &mut Self> {
        WithDefinitions(self, defs)
    }
}

unsafe impl Research for Katamari {
    fn get_section(defs: &DefinitionFile) -> &[Field] {
        &defs.katamari
    }
}

unsafe impl Research for Prince {
    fn get_section(defs: &DefinitionFile) -> &[Field] {
        &defs.prince
    }
}

unsafe impl Research for Camera {
    fn get_section(defs: &DefinitionFile) -> &[Field] {
        &defs.camera
    }
}

unsafe impl Research for Prop {
    fn get_section(defs: &DefinitionFile) -> &[Field] {
        &defs.thing
    }

    fn extra_before(&mut self, ui: &mut Ui) {
        unsafe {
            let idx = self.mono_ctrl_idx;
            let name_idx = self.mono_name_idx;
            let data = &*ps2::prop_data_array(name_idx as usize);
            let name = std::ffi::CStr::from_ptr(data.name).to_str().unwrap();
            ui.label(format!("[{idx}]: {name} ({name_idx})"));
        }
    }
}

pub struct WithDefinitions<'a, T>(T, &'a DefinitionFile);

impl<T: Research> Widget for WithDefinitions<'_, &mut T> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        unsafe { self.0.fields(ui, &self.1) }
    }
}
