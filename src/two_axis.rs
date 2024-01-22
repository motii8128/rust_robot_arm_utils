use crate::common_interface::{TargetPosition, ArmAppearance2};

pub fn get_target_angle2(
    target_position:TargetPosition,
    arm:ArmAppearance2
)->f64
{
    let x = target_position.x;
    let y = target_position.y;

    let l1 = arm.first_axis.length;
    let l2 = arm.second_axis.length;

    let xy = x.powi(2) + y.powi(2);

    ((xy - l1.powi(2) - l2.powi(2)) / (2.0 * l1 * l2)).acos()
}