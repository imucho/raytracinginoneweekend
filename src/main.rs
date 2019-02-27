extern crate rand;

mod vec3;
mod ray;
mod hitable;
mod hittable_list;
mod sphere;
mod camera;

use std::f32;
use vec3::Vec3;
use ray::Ray;
use hitable::*;
use hittable_list::HitableList;
use sphere::Sphere;
use camera::Camera;
use rand::Rng;

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p;
    loop {
        p = 2.0*Vec3::new(rng.gen_range(0.0,1.0), rng.gen_range(0.0,1.0), rng.gen_range(0.0,1.0)) - Vec3::new(1.0,1.0,1.0);
        if p.squared_length() >= 1.0 {
            break;
        }
    }
    return p;
}

fn color(r: &Ray, world: &Hitable) -> Vec3 {
    if let Some(rec) = world.hit(r, 0.0, f32::MAX) {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        return 0.5 * color(&Ray::new(rec.p, target-rec.p), world);
    } else {
        let unit_direction = Vec3::unit_vector(r.direction);
        let t = 0.5*(unit_direction.y() + 1.0);
        return (1.0-t)*Vec3::new(1.0, 1.0, 1.0) + t*Vec3::new(0.5, 0.7, 1.0);
    }
}

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    println!("P3\n{} {}\n255",nx,ny);
    let mut world = HitableList::new();
    world.push(Box::new(Sphere {center: Vec3::new(0.0,0.0,-1.0), radius: 0.5}));
    world.push(Box::new(Sphere {center: Vec3::new(0.0,-100.5,-1.0), radius: 100.0}));
    let mut rng = rand::thread_rng();
    let cam = Camera::new();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (i as f32 + rng.gen_range(0.0,1.0)) / nx as f32;
                let v = (j as f32 + rng.gen_range(0.0,1.0)) / ny as f32;
                let r = cam.get_ray(u, v);
                col += color(&r, &world);
            }
            col /= ns as f32;
            col = Vec3::new(col[0].sqrt(), col[1].sqrt(), col[2].sqrt());
            let ir = (255.99*col[0]) as i32;
            let ig = (255.99*col[1]) as i32;
            let ib = (255.99*col[2]) as i32;
            println!("{} {} {}",ir,ig,ib);
        }
    }
}