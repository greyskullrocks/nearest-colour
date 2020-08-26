use wasm_bindgen::prelude::*;

struct Colour {
    distance: f64,
    colour: [u8; 3],
}

impl Colour {
    pub fn new(distance: f64, colour: [u8; 3]) -> Self {
        Colour { distance, colour }
    }
}

#[wasm_bindgen]
pub fn find_nearest_colour() -> Vec<u8> {
    let colour_pixel: &str = "rgb(255,255,134)";
    let colour_pool: [[u8; 3]; 3] = [[0, 0, 0], [255, 255, 255], [000, 255, 010]];

    let mut colour_distance: Vec<Colour> = vec![];

    for colour in colour_pool.iter() {
        let comp_colour: [u8; 3] = extract_rgb(colour_pixel);
        let r1: u8 = comp_colour[0];
        let g1: u8 = comp_colour[1];
        let b1: u8 = comp_colour[2];
        let r2: u8 = colour[0];
        let g2: u8 = colour[1];
        let b2: u8 = colour[2];

        let x = u32::pow((r2 - r1).into(), 2)
            + u32::pow((g2 - g1).into(), 2)
            + u32::pow((b2 - b1).into(), 2);
        let distance = f64::sqrt(x.into());
        colour_distance.push(Colour::new(distance, *colour))
    }
    colour_distance.sort_by(|b, a| b.distance.partial_cmp(&a.distance).unwrap());
    // println!("{:#?}", colour_distance[0].colour);
    return colour_distance[0].colour.to_vec();
}

fn extract_rgb(rgb: &str) -> [u8; 3] {
    let len = rgb.len() - 1;
    let sub = &rgb[4..len];
    let color_arr: Vec<&str> = sub.split(",").collect();

    return [
        color_arr[0].parse().unwrap(),
        color_arr[1].parse().unwrap(),
        color_arr[2].parse().unwrap(),
    ];
}
