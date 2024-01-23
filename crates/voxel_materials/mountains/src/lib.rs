use noise::{NoiseFn, Perlin};

pub fn get_color(seed: u64, x: u64, y: u64, z: u64, _lod: u64) -> (u32, f32, f32, f32) {
    let perlin = Perlin::new(seed as u32);
    let val = perlin.get([x as f64, y as f64, z as f64]);
    if val > 0.5 {
        (1, 1.0, 1.0, 1.0)
    } else {
        (0, 0.0, 0.0, 0.0)
    }
}
