pub mod graphic_vec {
    use std::ops::{Add, Sub};

    pub struct Vec3 {
        color: Vec<f64>,
    }

    impl Add for Vec3 {
        type Output = Self;
        fn add(self, other: Self) -> Self {
            Vec3 {
                color: vec![
                    self.get_val(0) + other.get_val(0),
                    self.get_val(1) + other.get_val(1),
                    self.get_val(2) + other.get_val(2),
                ],
            }
        }
    }

    impl Sub for Vec3 {
        type Output = Self;
        fn sub(self) -> Self {
            Vec3 {
                color: vec![-self.get_val(0), -self.get_val(1), -self.get_val(2)],
            }
        }
    }

    impl Vec3 {
        pub fn empty_vec3() -> Self {
            Self {
                color: vec![0.0; 3],
            }
        }

        pub fn valued_vec3(value1: f64, value2: f64, value3: f64) -> Self {
            Self {
                color: vec![value1, value2, value3],
            }
        }

        pub fn get_x(&self) -> f64 {
            self.color.get(0).cloned().unwrap_or(0.0)
        }

        pub fn get_y(&self) -> f64 {
            self.color.get(1).cloned().unwrap_or(0.0)
        }

        pub fn get_z(&self) -> f64 {
            self.color.get(2).cloned().unwrap_or(0.0)
        }

        fn negate_value(&self, index: usize) -> f64 {
            -self.color.get(index).cloned().unwrap_or(0.0)
        }
        pub fn operator_negative(&self) -> Self {
            Self {
                color: vec![
                    self.negate_value(0),
                    self.negate_value(1),
                    self.negate_value(2),
                ],
            }
        }

        pub fn get_val(&self, index: usize) -> &f64 {
            self.color.get(index).unwrap_or(&0.0)
        }

        // fn operation_add_helper(&self, index: usize, value: f64) -> f64 {
        //     match operation {
        //         '+' => self.color.get(index).cloned().unwrap_or(0.0) + value,
        //         '*' => self.color.get(index).cloned().unwrap_or(0.0) * value,
        //         _ => 0.0,
        //     }
        // }
        pub fn operator_add(&mut self, vec3: &Self) -> &Self {
            self.color = vec![
                self.get_val(0) + vec3.get_x(),
                self.get_val(1) + vec3.get_y(),
                self.get_val(2) + vec3.get_z(),
            ];
            self
        }

        pub fn operator_multiply(&mut self, val: f64) -> &Self {
            for ele in self.color.iter_mut() {
                *ele *= val;
            }
            self
        }

        pub fn operator_divide(&mut self, val: f64) -> &Self {
            for ele in self.color.iter_mut() {
                *ele *= 1.0 / val;
            }
            self
        }

        pub fn length(&self) -> f64 {
            let mut sum = 0.0;
            for ele in self.color.iter() {
                sum += ele * ele;
            }
            sum.sqrt()
        }
    }

    type Point3 = Vec3;
}
