use std::sync::Arc;

use crate::material::Material;
use crate::Point3;
use crate::Ray;
use crate::Vec3;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: &Point3, normal: &Vec3, t: f64, material: &Arc<dyn Material>) -> Self {
        Self {
            p: p.clone(),
            normal: normal.clone(),
            t,
            front_face: false,
            material: material.clone(),
        }
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction().dot(*outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hittable: Sync + Send {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
