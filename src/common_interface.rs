pub struct ArmFingerTipPosition
{
    pub pos_x:f64,
    pub pos_y:f64,
    pub pos_z:f64,
}

pub struct ArmBasePosition
{
    pub pos_x:f64,
    pub pos_y:f64,
    pub pos_z:f64,
}

impl ArmBasePosition {
    pub fn new(x:f64, y:f64, z:f64)->ArmBasePosition
    {
        ArmBasePosition{
            pos_x:x,
            pos_y:y,
            pos_z:z
        }
    }
}

impl ArmFingerTipPosition {
    pub fn new(x:f64, y:f64, z:f64)->ArmFingerTipPosition
    {
        ArmFingerTipPosition{
            pos_x:x,
            pos_y:y,
            pos_z:z
        }
    }
}