use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hitable::{HitRecord,Hitable};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32
}

impl Sphere {
    fn make_hit_record(&self, t: f32, r: &Ray) -> HitRecord {
        let p = r.point_at_parameter(t);
        let normal = (p - self.center) / self.radius;
        HitRecord { t: t, p: p, normal: normal }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.dot(r.direction);
        let b = oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius*self.radius;
        let discriminant = b*b - a*c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && t_min < temp {
                return Some(self.make_hit_record(temp, r));
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && t_min < temp {
                return Some(self.make_hit_record(temp, r));
            }
        }
        None
    }
}