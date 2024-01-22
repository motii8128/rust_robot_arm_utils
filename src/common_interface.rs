pub struct TargetPosition
{
    pub x:f64,
    pub y:f64,
    pub z:f64
}

impl TargetPosition {
    pub fn new(x:f64, y:f64, z:f64)->TargetPosition
    {
        TargetPosition{
            x:x,
            y:y,
            z:z
        }
    }
    pub fn get_x(&self)->f64
    {
        self.x
    }
    pub fn get_y(&self)->f64
    {
        self.y
    }
    pub fn get_z(&self)->f64
    {
        self.z
    }
}

pub struct Axis
{
    pub length:f64,
    pub current_angle:f64,
    pub rotation_direction:char
}

impl Axis {
    pub fn new(length:f64, current_angle:f64, rotation_direction:char)->Axis
    {
        Axis{
            length:length,
            current_angle:current_angle,
            rotation_direction:rotation_direction
        }
    }
}
