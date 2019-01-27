use quicksilver::{
    geom::{Vector, Scalar, Rectangle, Shape},
    lifecycle::{Asset}, // To load anything, we need Asset
    graphics::{Image}, // We need Image and image backgrounds
};


pub struct Building {
    pub image: Asset<Image>,
    pub position: Vector,
    pub start_position: Vector,
    pub splash_area: Rectangle,
}

impl Building {
    pub fn new(file_name: &'static str, position: (impl Scalar, impl Scalar)) -> Building {
        let splash_zone = (50, 300);
        Building {
            image: Asset::new(Image::load(file_name)),
            start_position: Vector::new(position.0, position.1),
            position: Vector::new(position.0, position.1),
            splash_area: Rectangle::new_sized(splash_zone).with_center(position)
        }
    }
}
