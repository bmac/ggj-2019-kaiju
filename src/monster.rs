use quicksilver::{
    geom::{Shape, Transform, Vector},
    graphics::{Animation, Background::Img}, // We need Image and image backgrounds
    lifecycle::{Asset, Window},             // To load anything, we need Asset
    Result,
};

pub enum MonsterState {
    // An `enum` may either be `unit-like`,
    Walking,
    Idle,
    Attack,
}

pub struct Monster {
    pub walking_animation: Asset<Animation>,
    pub idle_animation: Asset<Animation>,
    pub attack_animation: Asset<Animation>,
    pub state: MonsterState,
    pub position: Vector,
    pub facing: f32, // 1 or -1 so we can easily pass to scale
}

impl Monster {
    pub fn render(&mut self, window: &mut Window) -> Result<()> {
        let position = self.position;
        let facing = self.facing;

        // move to monster render method
        let animation = match self.state {
            MonsterState::Idle => &mut self.idle_animation,
            MonsterState::Walking => &mut self.walking_animation,
            MonsterState::Attack => &mut self.attack_animation,
        };

        animation.execute(|character_animation| {
            let current_frame = character_animation.current_frame();
            window.draw_ex(
                &current_frame.area().with_center(position),
                Img(&current_frame),
                Transform::scale((facing, 1.0)),
                100,
            );
            character_animation.tick();
            Ok(())
        })
    }

    pub fn mouse_direction(&self, pos: Vector) -> i32 {
        let dead_zone = 10.0;
        if pos.x > (self.position.x + dead_zone) {
            return 1;
        }

        if pos.x < (self.position.x - dead_zone) {
            return -1;
        }

        return 0;
    }
}
