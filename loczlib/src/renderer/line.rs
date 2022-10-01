use super::fixedpoint;
use fixedpoint::Fixed;

pub struct YSampledLine {
    pub x: i32,
    pub start_y: i32,
    pub end_y: i32,

    area: Fixed<6>,
    pub base_x_step: i32,     // Base amount to step x by when moving 1 row down
    base_area_step: Fixed<6>, // Base amount to step area by when moving 1 row down
    darea_dx: Fixed<6>,
}

impl YSampledLine {
    pub fn new<'a>(from: &'a [f32; 4], to: &'a [f32; 4]) -> Self {
        let mut from_x = Fixed::<6>::from(from[0]);
        let mut from_y = Fixed::<6>::from(from[1]);
        let mut to_x = Fixed::<6>::from(to[0]);
        let mut to_y = Fixed::<6>::from(to[1]);

        if from_y > to_y || (from_y == to_y && from_x < to_x) {
            std::mem::swap(&mut from_x, &mut to_x);
            std::mem::swap(&mut from_y, &mut to_y);
        }

        // Area = (From->To) x (From->P) = (To_x-From_x)*(From_y-P_y)-(From_y-To_y)*(P_x-From_x) >= 0
        // NOTE: The y-coordinate deltas are reversed because a positive y delta actually points **down**
        // due to gfx coords
        let darea_dx = to_y - from_y;
        let darea_dy = from_x - to_x;

        let (mut base_x_step, mut base_area_step) = (Fixed::from(0), Fixed::from(0));
        if !darea_dx.is_zero() {
            (base_x_step, base_area_step) = darea_dy.div_rem_euclid(darea_dx);
        }
        base_x_step = -base_x_step;

        let y_0 = from_y.ceil();
        let end_y = to_y.floor();
        let area_y = (y_0 - from_y) * darea_dy;
        let mut x_0 = from_x.ceil();
        if !darea_dx.is_zero() {
            let (dx, _) = area_y.div_rem_euclid(darea_dx);
            x_0 = x_0 - dx;
        }
        let area_x = (x_0 - from_x) * darea_dx;

        Self {
            x: x_0.as_i32(),
            start_y: y_0.as_i32(),
            end_y: end_y.as_i32(),

            area: area_y + area_x - darea_dx,
            base_x_step: base_x_step.as_i32(),
            base_area_step,
            darea_dx,
        }
    }
    pub fn is_horizontal(&self) -> bool {
        self.end_y <= self.start_y
    }
    /// Steps the line down (in +y direction) by 1 unit. Returns a bool:
    /// * `true`: in addition to the base step, an additional step to the left was taken
    /// * `false`: only a base step was taken
    pub fn step(&mut self) -> bool {
        self.x += self.base_x_step;
        self.area = self.area + self.base_area_step;
        if !self.area.is_neg() {
            self.x -= 1;
            self.area = self.area - self.darea_dx;
            return true;
        }
        false
    }
}
