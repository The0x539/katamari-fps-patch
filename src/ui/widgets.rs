use eframe::egui;
use egui::{Checkbox, DragValue, Ui, Widget, WidgetText};

use crate::{
    ps2,
    types::{Katamari, MotionVectors, Prop, Vec4},
    ui::definition_file::{DefinitionFile, Field, FieldType},
};

impl Field {
    pub fn label(&self) -> String {
        self.name
            .clone()
            .unwrap_or_else(|| format!("{:#x}", self.offset))
    }

    pub fn ui<T>(&self, data: *mut T, ui: &mut Ui) -> egui::Response {
        assert!(self.offset < std::mem::size_of::<T>());

        ui.horizontal(|ui| {
            ui.label(self.label());

            unsafe {
                let ptr = data.cast::<()>().byte_offset(self.offset as isize);
                // ui.label(format!("{:#x}: {:?}", self.offset, ptr));
                self.field_type.ui(ptr, ui);
            }
        })
        .response
    }
}

impl FieldType {
    pub unsafe fn ui(&self, ptr: *mut (), ui: &mut Ui) -> egui::Response {
        unsafe fn drag_value<'a, T: egui::emath::Numeric>(ptr: *mut ()) -> DragValue<'a> {
            unsafe {
                let val: &mut T = &mut *ptr.cast::<T>();
                DragValue::new(val)
            }
        }

        unsafe {
            match self {
                FieldType::Float => drag_value::<f32>(ptr).ui(ui),
                FieldType::Vec4 => (&mut *ptr.cast::<Vec4>()).ui(ui),
                FieldType::Mat4 => todo!(),
                FieldType::Bool => Checkbox::without_text(&mut *ptr.cast::<bool>()).ui(ui),
                FieldType::Int8 { signed: false } => drag_value::<u8>(ptr).ui(ui),
                FieldType::Int8 { signed: true } => drag_value::<i8>(ptr).ui(ui),
                FieldType::Int16 { signed: false } => drag_value::<u16>(ptr).ui(ui),
                FieldType::Int16 { signed: true } => drag_value::<i16>(ptr).ui(ui),
                FieldType::Int32 { signed: false } => drag_value::<u32>(ptr).ui(ui),
                FieldType::Int32 { signed: true } => drag_value::<i32>(ptr).ui(ui),
                FieldType::Int64 { signed: false } => drag_value::<u64>(ptr).ui(ui),
                FieldType::Int64 { signed: true } => drag_value::<i64>(ptr).ui(ui),
                // TODO: Find addresses within known arrays
                FieldType::Address => ui.label(format!("{:?}", *ptr.cast::<*mut ()>())),
                FieldType::Array(inner, len) => {
                    let stride = inner.size() as isize;
                    let mut cursor = ptr;
                    for _ in 0..*len {
                        inner.ui(cursor, ui);
                        cursor = cursor.offset(stride);
                    }
                    ui.response()
                }
            }
        }
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

pub unsafe trait Research: Sized {
    fn get_section(defs: &DefinitionFile) -> &[Field];

    fn fields(&mut self, ui: &mut Ui, defs: &DefinitionFile) -> egui::Response {
        self.extra_before(ui);
        // ui.label(format!("{:#?}", &raw mut *self));
        for field in Self::get_section(defs) {
            field.ui::<Self>(&raw mut *self, ui);
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

    fn extra_after(&mut self, ui: &mut Ui) {
        ui.label("motion vectors (main/previous)");
        ui.horizontal(|ui| {
            self.motion.ui(ui);
            self.prev_motion.ui(ui);
        });
    }
}

unsafe impl Research for Prop {
    fn get_section(defs: &DefinitionFile) -> &[Field] {
        &defs.thing
    }

    fn extra_before(&mut self, ui: &mut Ui) {
        unsafe {
            let idx = self.mono_name_idx;
            let data = &*ps2::prop_data_array(idx as usize);
            let name = std::ffi::CStr::from_ptr(data.name).to_str().unwrap();
            ui.label(format!("{idx}: {name}"));
        }
    }
}

pub struct WithDefinitions<'a, T>(T, &'a DefinitionFile);

impl<T: Research> Widget for WithDefinitions<'_, &mut T> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        self.0.fields(ui, &self.1)
    }
}
