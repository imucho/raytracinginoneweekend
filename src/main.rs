mod vec3;
mod ray;
mod hitable;
mod hittable_list;
mod sphere;

use std::f32;
use vec3::Vec3;
use ray::Ray;
use hitable::*;
use hittable_list::HitableList;
use sphere::Sphere;

fn color(r: &Ray, world: &Hitable) -> Vec3 {
    if let Some(rec) = world.hit(r, 0.0, f32::MAX) {
        return 0.5 * Vec3::new(rec.normal.x()+1.0, rec.normal.y()+1.0, rec.normal.z()+1.0);
    } else {
        let unit_direction = Vec3::unit_vector(r.direction);
        let t = 0.5*(unit_direction.y() + 1.0);
        return (1.0-t)*Vec3::new(1.0, 1.0, 1.0) + t*Vec3::new(0.5, 0.7, 1.0);
    }
}

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255",nx,ny);
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let mut world = HitableList::new();
    world.push(Box::new(Sphere {center: Vec3::new(0.0,0.0,-1.0), radius: 0.5}));
    world.push(Box::new(Sphere {center: Vec3::new(0.0,-100.5,-1.0), radius: 100.0}));
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical);
            let col = color(&r, &world);
            let ir = (255.99*col[0]) as i32;
            let ig = (255.99*col[1]) as i32;
            let ib = (255.99*col[2]) as i32;
            println!("{} {} {}",ir,ig,ib);
        }
    }
}