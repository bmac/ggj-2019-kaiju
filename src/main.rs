// Draw an image to the screen
extern crate quicksilver;
use std::f32;

use quicksilver::{
    Future,
    Result,
    geom::{Shape, Vector, Rectangle, Transform},
    graphics::{Background, Background::Img, Color, Image, Animation}, // We need Image and image backgrounds
    lifecycle::{Asset, Settings, State, Window, run}, // To load anything, we need Asset
    input::Key,
};

struct KaijuEngine {
    sky_background: Asset<Image>,
    city_background: Asset<Image>,
    building: Building,
    monster: Monster,
}

enum MonsterState {
    // An `enum` may either be `unit-like`,
    Walking,
    Idle,
    Attack,
}

struct Building {
    image: Asset<Image>,
    position: Vector,
    splash_area: Rectangle,
}

struct Monster {
    walking_animation: Asset<Animation>,
    idle_animation: Asset<Animation>,
    attack_animation: Asset<Animation>,
    state: MonsterState,
    position: Vector,
    facing: f32, // 1 or -1 so we can easily pass to scale
}


fn create_animation_asset(file_name: &'static str, rows: usize) -> Asset<Animation> {
    // x, y
        // let animation_positions = vec![
        //     Rectangle::new((0, 0), (743, 596)), Rectangle::new((743, 0), (743, 596)), Rectangle::new((1486, 0), (743, 596)), Rectangle::new((2229, 0), (743, 596)), Rectangle::new((2972, 0), (743, 596)),
        // Rectangle::new((0, 596), (743, 596)), Rectangle::new((743, 596), (743, 596)), Rectangle::new((1486, 596), (743, 596)), Rectangle::new((2229, 596), (743, 596)), Rectangle::new((2972, 596), (743, 596)),
        // Rectangle::new((0, 1192), (743, 596)), Rectangle::new((743, 1192), (743, 596)), Rectangle::new((1486, 1192), (743, 596)), Rectangle::new((2229, 1192), (743, 596)), Rectangle::new((2972, 1192), (743, 596))
        // ];


    // 249
    // 249 + 249 = 498 + 249 = 747 + 249 = 996
    // height 200
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

impl State for KaijuEngine {
    fn new() -> Result<KaijuEngine> {

        let monster = Monster {
            walking_animation: create_animation_asset("monster_2_youngster_green_walk.png", 3),
            idle_animation: create_animation_asset("monster_2_youngster_green_idle.png", 5),
            attack_animation: create_animation_asset("monster_2_youngster_green_attack.png", 3),
            state: MonsterState::Idle,
            position: Vector::new(50, 500),
            facing: 1.0,
        };

        let sky_background = Asset::new(Image::load("sky.png"));
        let city_background = Asset::new(Image::load("city_background.png"));

        let image = Asset::new(Image::load("building_1.png"));
        let building = Building {
            image,
            position: Vector::new(200, 450),
            splash_area: Rectangle::new_sized((200, 300)).with_center((200, 450))
        };

        Ok(KaijuEngine {
            city_background,
            sky_background,
            monster,
            building,
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
            if self.building.splash_area.overlaps(&monster_rect) {
                self.building.position.y += 1.5;
            }
            self.monster.state = MonsterState::Attack;
        } else {
            self.monster.state = MonsterState::Idle;
        }

        Ok(())
    }


    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        let position = self.monster.position;
        let facing = self.monster.facing;

        self.sky_background.execute(|bg_image| {
            window.draw(&bg_image.area().with_center((400, 300)), Img(&bg_image));
            Ok(())
        })?;

        self.city_background.execute(|bg_image| {
            window.draw(&bg_image.area().with_center((400, 300)), Img(&bg_image));
            Ok(())
        })?;


        let building_position = self.building.position;
        let pos_y = self.building.position.y;
        let frequency = 0.5;
        let rotate = ((pos_y - 450.0) * frequency).sin() * 2.0;

        // let splash_area = self.building.splash_area;
        // window.draw_ex(&splash_area, Background::Col(Color::BLUE), Transform::IDENTITY, 100);
        // let monster_rect = Rectangle::new_sized((249, 200)).with_center(self.monster.position);
        //window.draw_ex(&monster_rect, Background::Col(Color::GREEN), Transform::IDENTITY, 100);

        
        self.building.image.execute(|image| {
            window.draw_ex(&image.area().with_center(building_position), Img(&image),
                           Transform::rotate(rotate),
            1);
            Ok(())
        })?;

        let animation = match self.monster.state {
            MonsterState::Idle => &mut self.monster.idle_animation,
            MonsterState::Walking => &mut self.monster.walking_animation,
            MonsterState::Attack => &mut self.monster.attack_animation,
        };

        animation.execute(|character_animation| {
            let current_frame = character_animation.current_frame();
            window.draw_ex(
                &current_frame.area().with_center(position),
                Img(&current_frame),
                Transform::scale((facing, 1.0)),
                10);
            character_animation.tick();
            Ok(())
        })
    }
}

fn main() {
    run::<KaijuEngine>("Kaiju Homes", Vector::new(800, 600), Settings::default());
}
