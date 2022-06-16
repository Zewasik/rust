#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Object {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThrowObject {
    pub init_position: Object,
    pub init_velocity: Object,
    pub actual_position: Object,
    pub actual_velocity: Object,
    pub time: f32,
}

impl ThrowObject {
    pub fn new(init_position: Object, init_velocity: Object) -> ThrowObject {
        ThrowObject {
            init_position,
            init_velocity,
            actual_position: init_position,
            actual_velocity: init_velocity,
            time: 0.0,
        }
    }
}

impl Iterator for ThrowObject {
    type Item = ThrowObject;

    fn next(&mut self) -> Option<Self::Item> {
        let round_2_digits = |f: f32| (f * 100.0).round() / 100.0;

        self.time += 1.0;
        self.actual_position.y = round_2_digits(
            self.init_position.y
                + self.init_velocity.y * self.time
                + 0.5 * (-9.8 * self.time * self.time),
        );
        self.actual_position.x =
            round_2_digits(self.init_position.x + self.init_velocity.x * self.time);
        self.actual_velocity.y = round_2_digits(self.init_velocity.y + (-9.8 * self.time));

        if self.actual_position.y >= 0.0 {
            Some(self.clone())
        } else {
            None
        }
    }
}
