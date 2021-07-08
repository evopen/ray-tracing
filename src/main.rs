// based on ray tracing in one weekend 3.2.3

mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod utils;
mod vec3;

use std::rc::Rc;

use color::Color;
use hittable::HitRecord;
use ray::Ray;
use vec3::{Point3, Vec3};

use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> Option<f64> {
    let oc = r.origin() - *center;
    let a = r.direction().length_squared();
    let half_b = oc.dot(r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        None
    } else {
        Some((-half_b - discriminant.sqrt()) / a)
    }
}

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::default();
    if world.hit(r, 0.0, std::f64::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::splat(1.0));
    }

    let unit_direction = r.direction().normalize();
    let t = (unit_direction.y + 1.0) * 0.5;
    Color::splat(1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(&Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(&Vec3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - Vec3::new(0.0, 0.0, focal_length) - horizontal / 2.0 - vertical / 2.0;

    // Render
    let mut image_buffer = image::RgbImage::new(image_width, image_height);

    for y in (0..image_height).rev() {
        println!("\rScanlines remaining: {}", y);
        let v = y as f64 / (image_height - 1) as f64;
        for x in 0..image_width {
            let u = x as f64 / (image_width - 1) as f64;
            let r = Ray::new(
                &origin,
                &(lower_left_corner + u * horizontal + v * vertical - origin),
            );
            let pixel_color = ray_color(&r, &world);
            color::write_color(&mut &mut image_buffer, x, y, &pixel_color);
        }
    }
    println!("Done");
    image_buffer.save("result.png").unwrap();
}
