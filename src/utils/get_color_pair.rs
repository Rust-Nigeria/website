pub fn get_color_pair(index: usize, seed: u8) -> (String, String) {
    // Mix the seed into the hue in a simple deterministic way
    let hue = ((index as f32 * 47.0) + seed as f32 * 13.0) % 360.0;

    let bg = format!("hsl({hue:.0}, 70%, 85%)");
    let text = format!("hsl({hue:.0}, 70%, 25%)");

    (bg, text)
}
