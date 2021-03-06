use crate::ray::Ray;
use crate::vec3::Point3;
use crate::{utils, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    front: Vec3,
    right: Vec3,
    up: Vec3,
    lens_radius: f64,
}

impl Camera {
    /// Brief.
    ///
    /// Description.
    ///
    /// * `vfov` - Vertical field of view in degree
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan(); // assume focal length is 1.0
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let front = (lookat - lookfrom).normalize();
        let right = front.cross(vup).normalize();
        let up = right.cross(front);

        let origin = lookfrom;

        // focus plane distance from lens is no longer 1.0, scale up accordingly
        let horizontal = focus_dist * viewport_width * right;
        let vertical = focus_dist * viewport_height * up;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 + focus_dist * front;

        let lens_radius = aperture / 2.0;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            front,
            right,
            up,
            lens_radius,
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius * utils::rand_vec3_in_unit_disk();
        let offset = self.right * rd.x + self.up * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
        )
    }
}
