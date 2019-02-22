use crate::hitable::*;
use crate::ray::Ray;

pub struct HitableList {
    list: Vec<Box<Hitable>>
}

impl HitableList {
    pub fn new() -> HitableList {
        HitableList {list: Vec::new()}
    }
    pub fn push(&mut self, v: Box<Hitable>) {
        self.list.push(v);
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut ret: Option<HitRecord> = None;
        for v in self.list.iter() {
            if let Some(rec) = v.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                ret = Some(rec);
            }
        }
        ret
    }
}