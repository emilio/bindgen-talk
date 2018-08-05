#[macro_use]
extern crate clap;

use std::ffi::CString;
use std::path::Path;

#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
pub mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use bindings::MagickBooleanType_MagickFalse as MagickFalse;
use bindings::MagickBooleanType_MagickTrue as MagickTrue;

/// Ensures that MagickWand is properly released on program exit.
///
/// You need to call `MagickWandGenesis` before creating this, and you can only
/// construct one object of this kind.
struct MagickWandGuard;

impl Drop for MagickWandGuard {
    fn drop(&mut self) {
        unsafe { bindings::MagickWandTerminus() };
    }
}

struct MagickWandInstance(*mut bindings::MagickWand);

impl MagickWandInstance {
    fn new() -> Self {
        MagickWandInstance(unsafe { bindings::NewMagickWand() })
    }

    fn get_exception(&mut self) -> String {
        use std::os::raw::c_char;
        use std::ffi::CStr;

        unsafe {
            let mut severity = 0;
            let description: *mut c_char =
                bindings::MagickGetException(self.0, &mut severity);

            let string = CStr::from_ptr(description).to_string_lossy().into_owned();

            bindings::MagickRelinquishMemory(description as *mut _);

            string
        }
    }

    fn read_image(&mut self, image: &Path) -> Result<(), String> {
        let c_string = CString::new(image.to_string_lossy().as_bytes()).map_err(|_| "Null byte?")?;
        let status = unsafe { bindings::MagickReadImage(self.0, c_string.as_ptr()) };
        if status == MagickFalse {
            Err(self.get_exception())
        } else {
            Ok(())
        }
    }

    fn resize_images(&mut self) {
        const THUMBNAIL_WIDTH: usize = 106;
        const THUMBNAIL_HEIGHT: usize = 80;

        unsafe {
            bindings::MagickResetIterator(self.0);
            while bindings::MagickNextImage(self.0) != MagickFalse {
                bindings::MagickResizeImage(
                    self.0,
                    THUMBNAIL_WIDTH,
                    THUMBNAIL_HEIGHT,
                    bindings::FilterTypes_LanczosFilter,
                    1.0,
                );
            }
        }
    }

    fn write_images(&mut self, output: &Path) -> Result<(), String> {
        let c_string = CString::new(output.to_string_lossy().as_bytes()).map_err(|_| "Null byte?")?;
        let status = unsafe {
            bindings::MagickWriteImages(
                self.0,
                c_string.as_ptr(),
                /* adjoin = */ MagickTrue,
            )
        };

        if status == MagickFalse {
            Err(self.get_exception())
        } else {
            Ok(())
        }
    }
}

impl Drop for MagickWandInstance {
    fn drop(&mut self) {
        unsafe { bindings::DestroyMagickWand(self.0) };
    }
}

fn main() {
    let matches = app_from_crate!()
        .args_from_usage(
            "<input>  'Image to make thumbnail of'
             <output> 'Where to save the thumbnail'"
        )
        .get_matches();

    let input = Path::new(matches.value_of("input").unwrap());
    let output = Path::new(matches.value_of("output").unwrap());

    unsafe { bindings::MagickWandGenesis() };
    let _guard = MagickWandGuard;

    let mut wand = MagickWandInstance::new();
    wand.read_image(input).expect("Couldn't read input image");
    wand.resize_images();
    wand.write_images(output).expect("Couldn't write image output");
}
