use crate::common_interface::{Axis, TargetPosition};

pub struct ArmAppearance3
{
    pub base_pos_x:f64,
    pub base_pos_y:f64,
    pub base_pos_z:f64,
    pub first_axis:Axis,
    pub second_axis:Axis,
    pub third_axis:Axis,
}

impl ArmAppearance3 {
    pub fn new(
        x:f64,
        y:f64,
        z:f64,
        first_axis:Axis,
        second_axis:Axis,
        third_axis:Axis,
    )->ArmAppearance3
    {
        ArmAppearance3{
            base_pos_x:x,
            base_pos_y:y,
            base_pos_z:z,
            first_axis:first_axis,
            second_axis:second_axis,
            third_axis:third_axis
        }
    }

    pub fn get_length_first_axis_(self)->f64
    {
        self.first_axis.length
    }
    pub fn get_length_second_axis(self)->f64
    {
        self.second_axis.length
    }
    pub fn get_length_third_axis(self)->f64
    {
        self.third_axis.length
    }
    pub fn get_direction_first_axis(self)->char
    {
        self.first_axis.rotation_direction
    }
    pub fn get_direction_second_axis(self)->char
    {
        self.second_axis.rotation_direction
    }
    pub fn get_direction_third_axis(self)->char
    {
        self.third_axis.rotation_direction
    }
    pub fn get_current_angle_first_axis(self)->f64
    {
        self.first_axis.current_angle
    }
    pub fn get_current_angle_second_axis(self)->f64
    {
        self.second_axis.current_angle
    }
    pub fn get_current_angle_third_axis(self)->f64
    {
        self.third_axis.current_angle
    }
}

