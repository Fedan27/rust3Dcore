/// Точка
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Вектор
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Прямая
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line {
    pub origin: Point3,
    pub direction: Vector3,
}

/// Круг
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    pub center: Point3,
    pub normal: Vector3,
    pub radius: f32,
}

/// Элипс
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ellipse {
    pub center: Point3,
    pub normal: Vector3,
    pub major_axis: Vector3, // Направление и длина большей полуоси
    pub minor_radius: f32,   // Длина меньшей полуоси
}

/// Плоскость
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plane {
    pub origin: Point3,
    pub normal: Vector3,
}

/// Цилиндр
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cylinder {
    pub axis_origin: Point3,
    pub axis_direction: Vector3,
    pub radius: f32,
}

/// Конус
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cone {
    pub apex: Point3,
    pub axis_direction: Vector3,
    pub half_angle: f32, // Угол от оси
}

/// Сфера
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
}

/// Тор
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Torus {
    pub axis_origin: Point3,
    pub axis_direction: Vector3, // Ось вращения
    pub major_radius: f32,       // Радиус до центра образующей окружности
    pub minor_radius: f32,       // Радиус образующей окружности
}

/// Дуга
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Arc {
    pub base: Circle,
    pub reference_dir: Vector3, // От этого направления отсчитываются углы (перпендикулярен base.normal)
    pub start_angle: f32,
    pub end_angle: f32,
}

/// Дуга элипса
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EllipticalArc {
    pub base: Ellipse,
    pub start_angle: f32, // Отсчитываются от base.major_axis
    pub end_angle: f32,
}

/// Поверхность сферы
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SphericalSurface {
    pub base: Sphere,

    pub polar_axis: Vector3, // Ось для полярных координат
    pub theta_start: f32,    // theta ∈ [0, PI]
    pub theta_end: f32,

    pub reference_dir: Vector3, // Нулевой меридиан (перпендикулярен polar_axis)
    pub phi_start: f32,         // phi ∈ [0, 2*PI]
    pub phi_end: f32,
}

/// Поверхность конуса
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConeSurface {
    pub base: Cone,

    pub reference_dir: Vector3, // Направление для отсчёта угла
    pub phi_start: f32,         // phi ∈ [0, 2*PI]
    pub phi_end: f32,

    pub h_start: f32, // Ограничения по высоте по base.axis_direction
    pub h_end: f32,
}

/// Поверхность тора
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TorusSurface {
    pub base: Torus,

    pub reference_dir: Vector3, // Направление для отсчёта major-угла
    pub phi_start: f32,         // phi ∈ [0, 2*PI]
    pub phi_end: f32,

    pub theta_start: f32, // theta (minor-угол) отсчитываем от base.axis_direction
    pub theta_end: f32,   // theta ∈ [0, 2*PI]
}

/// Габаритный отрезок
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Segment {
    pub base: Line,
    pub start_offset: f32, // расстояние от origin вдоль direction
    pub end_offset: f32,
}

/// Габаритный прямоугольник
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlaneRect {
    pub base: Plane,
    pub reference_dir: Vector3, // Направление оси u (перпендикулярен base.normal)
    pub u_min: f32,             // Отсчитываются от base.origin
    pub u_max: f32,
    pub v_min: f32, // Направление v избыточно
    pub v_max: f32,
}

/// Габаритный куб
pub struct Box {
    pub center: Point3,
    pub axis_w: Vector3,
    pub axis_u: Vector3, // v = w × u
    pub half_u: f32,
    pub half_v: f32,
    pub half_w: f32,
}

fn main() {}
