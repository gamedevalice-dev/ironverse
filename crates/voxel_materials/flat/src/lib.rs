#![no_std]

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub extern fn get_color(_seed: u64, _x: u64, y: u64, _z: u64, _lod: u64) -> (u32, f32, f32, f32) {
    if y > 0 {
        (1, 1.0, 1.0, 1.0)
    } else {
        (0, 0.0, 0.0, 0.0)
    }
}

// #[no_mangle]
// pub extern fn get_color(_seed: u64, _x: u64, y: u64, _z: u64, _lod: u64) -> Color {
//     if y > 0 {
//         Color::new(1, 1.0, 1.0, 1.0)
//     } else {
//         Color::new(0, 0.0, 0.0, 0.0)
//     }
// }

// #[repr(C)]
// pub struct Color {
//     pub voxel_type: u32,
//     pub r: f32,
//     pub g: f32,
//     pub b: f32,
// }
// impl Color {
//     pub fn new(voxel_type: u32, r: f32, g: f32, b: f32) -> Self {
//         Self {
//             voxel_type,
//             r,
//             g,
//             b,
//         }
//     }
// }