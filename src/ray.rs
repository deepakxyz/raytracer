use crate::vec3::Vec3;

pub struct Ray{
    a:Vec3, // origin
    b:Vec3, // direction
}

impl Ray{
    pub fn new(a: Vec3, b:Vec3)-> Ray{
        Ray{
            a,b
        }
    }

    pub fn origin(self) -> Vec3{
        self.a
    }

    pub fn direction(self) -> Vec3{
        self.b
    }
    
    // used to move along the point or extend the vector
    pub fn point_at_paramter(self, t:f32)->Vec3{
        self.a + self.b * t
    }
    
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_ray_origin(){

    }

    #[test]
    fn test_ray_direction(){

    }

    #[test]
    fn test_ray_point_at_paramter(){

    }
}
