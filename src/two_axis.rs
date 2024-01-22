use crate::common_interface::{TargetPosition, Axis};
use std::io::Error;
pub struct ArmAppearance2
{
    pub base_pos_x:f64,
    pub base_pos_y:f64,
    pub base_pos_z:f64,
    pub first_axis:Axis,
    pub second_axis:Axis,
}

impl ArmAppearance2 {
    pub fn new(
        x:f64,
        y:f64,
        z:f64,
        first_axis:Axis,
        second_axis:Axis,
    )->ArmAppearance2
    {
        ArmAppearance2{
            base_pos_x:x,
            base_pos_y:y,
            base_pos_z:z,
            first_axis:first_axis,
            second_axis:second_axis,
        }
    }

    pub fn get_first_axis_length(&self)->f64
    {
        self.first_axis.length
    }
    pub fn get_second_axis_length(&self)->f64
    {
        self.second_axis.length
    }
    pub fn get_first_axis_direction(&self)->char
    {
        self.first_axis.rotation_direction
    }
    pub fn get_second_axis_direction(&self)->char
    {
        self.second_axis.rotation_direction
    }
}

pub struct TargetAngle
{
    pub theta_0:f64,
    pub theta_1:f64,
    pub theta_2:f64,
}

pub fn create_target_angle(
    target_position:TargetPosition,
    arm:ArmAppearance2
)->Result<TargetAngle, Error>
{
    let x = target_position.get_x();
    let y = target_position.get_y();
    let z = target_position.get_z();

    let theta0 = (z / x).atan();

    let l1 = arm.get_first_axis_length();
    let l2 = arm.get_second_axis_length();

    let xy = x.powi(2) + y.powi(2);

    match (l1-l2).powi(2) <= xy && xy <= (l1 + l2).powi(2) {
        true=>{
            let theta2 = ((xy - l1.powi(2) - l2.powi(2)) / (2.0 * l1 * l2)).acos();

            let x2 = arm.get_second_axis_length() * theta2.cos();
            let y2 = arm.get_second_axis_length() * theta2.sin();

            let x1 = target_position.x - x2;
            let y1 = target_position.y - y2;

            let theta1 = (y1 / x1).atan().to_degrees();

            Ok(TargetAngle{theta_0:theta0, theta_1:theta1, theta_2:theta2})
        },
        false=>{
            Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "TargetPosition is position that arm cannot reach"))
        }
    }
}