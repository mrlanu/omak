use crate::components::Animation;
use specs::{Join, System, WriteStorage};

pub struct AnimationTick;
impl<'a> System<'a> for AnimationTick {
    type SystemData = WriteStorage<'a, Animation>;
    fn run(&mut self, mut anims: Self::SystemData) {
        for ani in (&mut anims).join() {
            ani.animations_tick += 1;
            if ani.animations_tick >= ani.animations_speed {
                ani.animations_tick = 0;
                ani.animations_index += 1;
                if ani.animations_index >= ani.animations_kind.get_index_and_count().1 {
                    ani.animations_index = 0;
                }
            }
        }
    }
}
