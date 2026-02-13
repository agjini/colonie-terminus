use crate::gameplay::tilemap::asset::Ground;
use bevy::math::U16Vec2;
use bevy::prelude::ops::*;
use bevy::prelude::*;
use noiz::prelude::*;
use std::f32::consts::PI;

#[derive(Clone)]
pub struct TilemapGenerator {
    pub noise: Noise<MixCellGradients<OrthoGrid, Smoothstep, QuickGradients>>,
    pub seed: u32,
    planet_width: f32,
    planet_height: f32,
}

impl TilemapGenerator {
    pub fn new(seed: u32, planet_width: f32, planet_height: f32) -> Self {
        let mut noise = Noise::<MixCellGradients<OrthoGrid, Smoothstep, QuickGradients>>::default();
        noise.set_seed(seed);
        noise.set_frequency(2.);
        Self {
            noise,
            seed,
            planet_width,
            planet_height,
        }
    }

    fn wrapping_noise(&self, pos: IVec2, scale: f32, offset: f32) -> f64 {
        let x = pos.x as f32;
        let y = pos.y as f32;

        let nx = cos(2.0 * PI * x / self.planet_width) * scale;
        let ny = sin(2.0 * PI * x / self.planet_width) * scale;
        let nz = cos(2.0 * PI * y / self.planet_height) * scale;
        let nw = sin(2.0 * PI * y / self.planet_height) * scale;

        self.noise.sample(Vec4::new(
            nx + offset,
            ny + offset,
            nz + offset,
            nw + offset,
        ))
    }

    pub fn ground_type(&self, pos: IVec2) -> Ground {
        if self.wrapping_noise(pos, 1.0, 0.0) > 0.0 {
            Ground::DirtRed
        } else {
            Ground::DirtBrown
        }
    }

    pub fn has_variant(&self, pos: IVec2) -> bool {
        self.wrapping_noise(pos, 80.0, 500.0) > 0.4
    }

    pub fn tile_variant(&self, pos: IVec2, from: U16Vec2, to: U16Vec2) -> U16Vec2 {
        let h = hash(self.seed, pos.x, pos.y);
        let range_x = (to.x - from.x + 1) as u32;
        let range_y = (to.y - from.y + 1) as u32;
        let x = from.x + (h % range_x) as u16;
        let y = from.y + ((h / range_x) % range_y) as u16;
        U16Vec2::new(x, y)
    }
}

fn hash(seed: u32, x: i32, y: i32) -> u32 {
    let mut h = seed;
    h = h.wrapping_mul(0x9E3779B9).wrapping_add(x as u32);
    h ^= h >> 16;
    h = h.wrapping_mul(0x85EBCA6B).wrapping_add(y as u32);
    h ^= h >> 13;
    h = h.wrapping_mul(0xC2B2AE35);
    h ^= h >> 16;
    h
}
