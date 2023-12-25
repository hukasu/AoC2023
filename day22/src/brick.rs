#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Brick {
    pub base_x: usize,
    pub base_y: usize,
    pub base_z: usize,
    pub base_w: usize,
    // Depth
    pub base_d: usize,
    pub height: usize,
}

impl Brick {
    pub fn apply_gravity(&self) -> Brick {
        Brick {
            base_x: self.base_x,
            base_y: self.base_y,
            base_z: self.base_z - 1,
            base_w: self.base_w,
            base_d: self.base_d,
            height: self.height,
        }
    }

    pub fn on_ground(&self) -> bool {
        self.base_z == 1
    }

    pub fn collision_detection(&self, other: &Brick) -> bool {
        self.base_z + self.height > other.base_z
            && self.base_z < other.base_z + other.height
            && self.base_x + self.base_w > other.base_x
            && self.base_x < other.base_x + other.base_w
            && self.base_y + self.base_d > other.base_y
            && self.base_y < other.base_y + other.base_d
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let (lposx, lposy, lposz) = (self.base_x, self.base_y, self.base_z);
        let (rposx, rposy, rposz) = (other.base_x, other.base_y, other.base_z);
        (lposz, lposx, lposy).cmp(&(rposz, rposx, rposy))
    }
}
