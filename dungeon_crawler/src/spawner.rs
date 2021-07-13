pub use crate::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        })
    );
}