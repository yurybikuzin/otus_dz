pub const VEC3_LEN: usize = 3;

#[derive(Debug, PartialEq)]
pub struct Vec3([i32; VEC3_LEN]);

impl Vec3 {
    pub fn default_vec3() -> Self {
        Self([0; 3])
    }

    pub fn scalar_sum(&self, b: Vec3) -> i32 {
        let mut c = 0;
        for i in 0..VEC3_LEN {
            c += self.0[i] + b.0[i];
        }
        c
    }

    pub fn vector_sum(&self, b: Vec3) -> Vec3 {
        let mut c = Vec3::default_vec3();
        for i in 0..3 {
            c.0[i] = self.0[i] + b.0[i];
        }
        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec3() {
        assert_eq!(Vec3::default_vec3(), Vec3([0; 3]));
        assert_eq!(Vec3::scalar_sum(&Vec3::default_vec3(), Vec3([2; 3])), 6);
        assert_eq!(Vec3::scalar_sum(&Vec3([5; 3]), Vec3([2; 3])), 21);
        assert_eq!(Vec3::vector_sum(&Vec3([5; 3]), Vec3([2; 3])), Vec3([7; 3]));
    }
}
