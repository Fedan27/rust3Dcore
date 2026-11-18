// Точка
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// Вектор
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// Прямая
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line {
    pub origin: Point3,
    pub direction: Vector3,
}

// Круг
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    pub center: Point3,
    pub normal: Vector3,
    pub radius: f32,
}

// Элипс
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ellipse {
    pub center: Point3,
    pub normal: Vector3,
    pub major_axis: Vector3, // Направление и длина большей полуоси
    pub minor_radius: f32,   // Длина меньшей полуоси
}

// Плоскость
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plane {
    pub origin: Point3,
    pub normal: Vector3,
}

// Цилиндр
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cylinder {
    pub axis_origin: Point3,
    pub axis_direction: Vector3,
    pub radius: f32,
}

// Конус
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cone {
    pub apex: Point3,
    pub axis_direction: Vector3,
    pub half_angle: f32, // Угол от оси
}

// Сфера
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
}

// Тор
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Torus {
    pub axis_origin: Point3,
    pub axis_direction: Vector3, // Ось вращения
    pub major_radius: f32,       // Радиус до центра образующей окружности
    pub minor_radius: f32,       // Радиус образующей окружности
}

fn main() {}
