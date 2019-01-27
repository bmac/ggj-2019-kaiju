use quicksilver::{
    Future,
    geom::{Rectangle},
    graphics::{Image, Animation}, // We need Image and image backgrounds
    lifecycle::{Asset}, // To load anything, we need Asset
};


pub fn create_animation_asset(file_name: &'static str, rows: usize) -> Asset<Animation> {

    let mut animation_positions = vec![
        Rectangle::new((0, 0), (249, 200)), Rectangle::new((249, 0), (249, 200)), Rectangle::new((498, 0), (249, 200)), Rectangle::new((747, 0), (249, 200)), Rectangle::new((996, 0), (249, 200)),
        Rectangle::new((0, 200), (249, 200)), Rectangle::new((249, 200), (249, 200)), Rectangle::new((498, 200), (249, 200)), Rectangle::new((747, 200), (249, 200)), Rectangle::new((996, 200), (249, 200)),
        Rectangle::new((0, 400), (249, 200)), Rectangle::new((249, 400), (249, 200)), Rectangle::new((498, 400), (249, 200)), Rectangle::new((747, 400), (249, 200)), Rectangle::new((996, 400), (249, 200)),
        Rectangle::new((0, 600), (249, 200)), Rectangle::new((249, 600), (249, 200)), Rectangle::new((498, 600), (249, 200)), Rectangle::new((747, 600), (249, 200)), Rectangle::new((996, 600), (249, 200)),
        Rectangle::new((0, 800), (249, 200)), Rectangle::new((249, 800), (249, 200)), Rectangle::new((498, 800), (249, 200)), Rectangle::new((747, 800), (249, 200)), Rectangle::new((996, 800), (249, 200)),
    ];

    animation_positions.truncate(5 * rows);

    let character_image = Image::load(file_name).map(move |character_image| {
        Animation::from_spritesheet(character_image.to_owned(), animation_positions, 4)
    });
    Asset::new(character_image)
}
