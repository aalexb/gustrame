use cgmath::num_traits::clamp;
use glam::{vec2, Vec2};

use super::{gameobject::{GameObject, Ball}, Direction};

pub fn AABB(one:&GameObject,two:&GameObject)->bool {
    let collision_x = one.position().x+one.size().x>=two.position().x &&
        two.position().x+two.size().x>=one.position().x;
    let collision_y = one.position().y+one.size().y>=two.position().y &&
        two.position().y+two.size().y>=one.position().y;
    collision_x &&collision_y
}

pub fn check_collision(one:&Ball,two:&GameObject)->bool {
    let center = one.obj.position()+one.radius;
    let aabb_half_extents=vec2(two.size().x/2.0,two.size().y/2.0);
    let aabb_center = vec2(two.position().x+aabb_half_extents.x,
        two.position().y+aabb_half_extents.y);
    let difference=center-aabb_center;
    let clamped = difference.clamp(-aabb_half_extents, aabb_half_extents);
    let closest = aabb_center+clamped;
    let difference = closest-center;
    difference.dot(difference)<one.radius
}

pub fn vector_direction(target:Vec2)->Direction {
    let compass = [vec2(0.0, 1.0),vec2(1.0, 0.0),
                    vec2(0.0,-1.0),vec2(-1.0, 0.0)];
    let mut max = 0.0;
    let mut best_match = -1;
    for i in 0..4 {
        let dot_product=target.normalize().dot(compass[i]);
        if dot_product>max {
            max=dot_product;
            best_match=i as i32;
        }
    }
    match best_match {
        0=>Direction::UP,
        1=>Direction::RIGHT,
        2=>Direction::DOWN,
        _=>Direction::LEFT
    }
}