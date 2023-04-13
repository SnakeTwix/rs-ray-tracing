use crate::material::{Lambertian, Material};
use crate::ray::Ray;
use crate::vec3::{Color, Point3, Vec3};
use std::rc::Rc;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            material: Rc::new(Lambertian {
                albedo: Color::new(0., 0., 0.),
            }),
            p: Point3::new(0., 0., 0.),
            normal: Point3::new(0., 0., 0.),
            t: 0.,
            front_face: true,
        }
    }
}

impl HitRecord {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction.dot(&outward_normal) < 0.;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with(object: Rc<dyn Hittable>) -> Self {
        let mut hittable_list = Self::new();
        hittable_list.add(object);
        hittable_list
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if object.hit(ray, t_min, closest_so_far, &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                *rec = temp_record.clone()
            }
        }

        hit_anything
    }
}
