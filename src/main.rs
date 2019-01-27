// Draw an image to the screen
extern crate quicksilver;

use quicksilver::{
    Result,
    geom::{Shape, Vector, Rectangle, Transform},
    graphics::{Background::Img, Color, Image}, // We need Image and image backgrounds
    lifecycle::{Asset, Settings, State, Window, run}, // To load anything, we need Asset
    input::Key,
};

mod monster;
mod building;
mod util;

use monster::{MonsterState, Monster};
use building::{Building};
use util::{create_animation_asset};


struct KaijuEngine {
    sky_background: Asset<Image>,
    city_background: Asset<Image>,
    buildings: Vec<Building>,
    monster: Monster,
}

impl KaijuEngine {
    fn render_buildings(&mut self, window: &mut Window) -> Result<()> {
        for building in &mut self.buildings {
            let building_position = building.position;
            let pos_y = building.position.y;
            let frequency = 0.5;
            let start_height = building.start_position.y;
            let rotate = ((pos_y - start_height) * frequency).sin() * 2.0;

            building.image.execute(|image| {
                window.draw_ex(&image.area().with_center(building_position), Img(&image),
                               Transform::rotate(rotate),
                               10);
                Ok(())
            })?;
        }

        Ok(())
    }


    fn render_background(&mut self, window: &mut Window) -> Result<()> {

        self.sky_background.execute(|bg_image| {
            window.draw(&bg_image.area().with_center((400, 300)), Img(&bg_image));
            Ok(())
        })?;

        self.city_background.execute(|bg_image| {
            window.draw(&bg_image.area().with_center((400, 300)), Img(&bg_image));
            Ok(())
        })?;

        Ok(())
    }
}

impl State for KaijuEngine {
    fn new() -> Result<KaijuEngine> {

        let monster = Monster {
            walking_animation: create_animation_asset("monster_2_youngster_green_walk.png", 3),
            idle_animation: create_animation_asset("monster_2_youngster_green_idle.png", 5),
            attack_animation: create_animation_asset("monster_2_youngster_green_attack.png", 3),
            state: MonsterState::Idle,
            position: Vector::new(50, 520),
            facing: 1.0,
        };

        let sky_background = Asset::new(Image::load("sky.png"));
        let city_background = Asset::new(Image::load("city_background.png"));

        let buildings = vec![
            Building::new("building_1.png", (200, 450)),
            Building::new("building_2.png", (280, 525)),
            Building::new("building_3.png", (360, 500)),
            Building::new("building_4.png", (440, 510)),
            Building::new("building_5.png", (520, 550)),
            Building::new("building_6.png", (600, 570)),
            Building::new("building_7.png", (680, 550)),
            Building::new("building_8.png", (760, 560)),
            Building::new("building_9.png", (840, 500)),
        ];

        Ok(KaijuEngine {
            city_background,
            sky_background,
            monster,
            buildings,
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        if window.keyboard()[Key::Right].is_down() {
            self.monster.position.x += 2.5;
            self.monster.facing = -1.0;
            self.monster.state = MonsterState::Walking;
        } else if window.keyboard()[Key::Left].is_down() {
            self.monster.position.x -= 2.5;
            self.monster.facing = 1.0;
            self.monster.state = MonsterState::Walking;
        } else if window.keyboard()[Key::Space].is_down() {
            let monster_rect = Rectangle::new_sized((249, 200)).with_center(self.monster.position);
            // maybe just use contains?
            for building in &mut self.buildings {
                if building.splash_area.overlaps(&monster_rect) {
                    building.position.y += 1.5;
                }
            }
            self.monster.state = MonsterState::Attack;
        } else {
            self.monster.state = MonsterState::Idle;
        }

        Ok(())
    }


    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;

        self.render_background(window)?;

        self.render_buildings(window)?;

        self.monster.render(window)
    }
}

fn main() {
    run::<KaijuEngine>("Kaiju Homes", Vector::new(800, 600), Settings {
        icon_path: Some("favicon.png"),
        ..Settings::default()
    });
}
