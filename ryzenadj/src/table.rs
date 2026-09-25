use core::slice;

use crate::{
    RyzenAdj,
    error::{Result, check_code},
};
use ryzenadj_sys as sys;

pub struct PowerTable<'a> {
    owner: &'a mut RyzenAdj,
}

impl<'a> From<&'a mut RyzenAdj> for PowerTable<'a> {
    fn from(owner: &'a mut RyzenAdj) -> Self {
        Self { owner }
    }
}

impl PowerTable<'_> {
    pub fn refresh(&mut self) -> Result<()> {
        check_code(unsafe { sys::refresh_table(self.owner.as_raw()) })
    }

    pub fn version(&self) -> u32 {
        unsafe { sys::get_table_ver(self.owner.as_raw()) }
    }

    pub fn values(&self) -> &[f32] {
        let ptr = unsafe { sys::get_table_values(self.owner.as_raw()) };

        let bytes = unsafe { sys::get_table_size(self.owner.as_raw()) };

        unsafe { slice::from_raw_parts(ptr, bytes / size_of::<f32>()) }
    }
}
