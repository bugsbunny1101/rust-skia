use crate::prelude::*;
use crate::{ColorFilter, ImageFilter, ImageFilterCropRect};
use skia_bindings::{C_SkColorFilterImageFilter_Make, SkImageFilter};

pub enum ColorFilterImageFilter {}

impl ColorFilterImageFilter {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<'a, CR: Into<Option<&'a ImageFilterCropRect>>>(
        cf: &ColorFilter,
        input: &ImageFilter,
        crop_rect: CR,
    ) -> Option<ImageFilter> {
        ImageFilter::from_ptr(unsafe {
            C_SkColorFilterImageFilter_Make(
                cf.shared_native(),
                input.shared_native(),
                crop_rect.into().native_ptr_or_null(),
            )
        })
    }
}

impl RCHandle<SkImageFilter> {
    pub fn color_filter<'a, CR: Into<Option<&'a ImageFilterCropRect>>>(
        &self,
        crop_rect: CR,
        cf: &ColorFilter,
    ) -> Option<Self> {
        ColorFilterImageFilter::new(cf, self, crop_rect)
    }
}
