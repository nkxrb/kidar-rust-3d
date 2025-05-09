use nalgebra::{Point3, Vector3, UnitQuaternion, Unit};

/// 调整相机俯仰角（Pitch）
/// - `eye`: 相机位置（`Point3<f32>`）
/// - `target`: 当前目标点（`Point3<f32>`）
/// - `up`: 当前相机的上向量（通常 `Vector3::y()`）
/// - `angle_degrees`: 旋转角度（正 = 抬头，负 = 低头）
/// - 返回: 新的目标点 `target`（`Point3<f32>`）
pub fn adjust_camera_pitch(
    eye: Vector3<f32>,
    target: Vector3<f32>,
    up: Vector3<f32>,
    angle_degrees: f32,
) -> Vector3<f32> {
    let forward = target - eye;
    let forward_norm = forward.normalize();
    let right = forward_norm.cross(&up).normalize();

    // 使用四元数旋转
    let rotation = UnitQuaternion::from_axis_angle(&Unit::new_normalize(right), angle_degrees.to_radians());
    let new_forward = rotation * forward;

    eye + new_forward
}