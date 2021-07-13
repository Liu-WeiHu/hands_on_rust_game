use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TitleType {
    Wall,
    Floor,
}

pub struct Map {
    pub titles: Vec<TitleType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            titles: vec![TitleType::Floor; NUM_TILES],
        }
    }

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(0);
        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                if self.in_bounds(Point::new(x, y)) {
                    let idx = map_idx(x, y);
                    match self.titles[idx] {
                        TitleType::Floor => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                WHITE,
                                BLACK,
                                to_cp437('.')
                            )
                        }
                        TitleType::Wall => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                WHITE,
                                BLACK,
                                to_cp437('#')
                            )
                        }
                    }
                }
            }
        }
    }

    pub fn in_bounds (&self, Point{x,y}: Point) -> bool {
        x >= 0 && x < SCREEN_WIDTH
            && y >= 0 && y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile (&self, point: Point) -> bool {
        self.in_bounds(point)
            && self.titles[map_idx(point.x, point.y)] == TitleType::Floor
    }

    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        }else {
            Some(map_idx(point.x, point.y))
        }
    }
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}