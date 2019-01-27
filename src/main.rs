// Draw an image to the screen
extern crate quicksilver;

use quicksilver::{
    Future,
    Result,
    geom::{Shape, Vector, Rectangle, Transform},
    graphics::{Background::Img, Color, Image, Animation}, // We need Image and image backgrounds
    lifecycle::{Asset, Settings, State, Window, run}, // To load anything, we need Asset
    input::Key,
};

struct KaijuEngine {
    monster: Monster,
}

enum MonsterState {
    // An `enum` may either be `unit-like`,
    Walking,
    Idle,
    Attack,
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

        Ok(KaijuEngine {
            monster
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
                1);
            character_animation.tick();
            Ok(())
        })
    }
}

fn main() {
    run::<KaijuEngine>("Kaiju Homes", Vector::new(800, 600), Settings::default());
}
