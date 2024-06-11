#[macro_export]
macro_rules! color_v_trait {
    ($T:ty) => {
        
        impl ThemeColorValue for $T {
            fn v(target: u32) -> Vec4 {
                hex_to_vec4(match target {
                    25 => Self::_25,
                    50 => Self::_50,
                    100 => Self::_100,
                    200 => Self::_200,
                    300 => Self::_300,
                    400 => Self::_400,
                    500 => Self::_500,
                    600 => Self::_600,
                    700 => Self::_700,
                    800 => Self::_800,
                    900 => Self::_900,
                    _ => panic!("invalid target"),
                })
            }
        
            fn get(&self) -> Vec4 {
                self.0
            }
        }
    };
}