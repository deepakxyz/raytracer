#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3{
    e: [f32;3]
}

impl Vec3{
    pub fn new(x:f32, y:f32, z:f32)-> Self{
        Vec3{
            e:[x, y, z]
        }
    }

    pub fn length(self)->f32{
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()

    }

    pub fn unit_vector(v:Vec3)->Vec3{
        v / v.length()
    }
}

impl std::ops::Add for Vec3{
    type Output = Self;

    fn add(self, rhs:Self)->Self{
        Self{
            e:[self.e[0] + rhs.e[0], self.e[1] + rhs.e[1], self.e[2] + rhs.e[2]]
        }
    }
}

impl std::ops::Mul<f32> for Vec3{
    type Output = Self;

    fn mul(self, rhs: f32)->Self{
        Self{
            e:[self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs]
        }
    }
}

impl std::ops::Div<f32> for Vec3{
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output{
        Self{
            e:[self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs]
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_vec3_add(){
        assert_eq!(Vec3::new(1.0, 1.0, 1.0) + Vec3::new(2.0, 2.0, 3.0), 
                   Vec3::new(3.0, 3.0, 4.0));
    }

    #[test]
    fn test_vec3_mult(){
        assert_eq!(Vec3::new(1.0, 1.0, 1.0) * 3.0,
                    Vec3::new(3.0, 3.0, 3.0));
    }

    #[test]
    fn test_vec3_div(){
        assert_eq!(Vec3::new(2.0, 8.0, 4.0) / 2.0, Vec3::new(1.0, 4.0, 2.0));
    }

}
