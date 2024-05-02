#[derive(Debug, PartialEq, Clone, Eq, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(self, first: u8, second: u8) -> Color {
        let mut new_color = Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        };

        if first == self.r {
            new_color.r = second;
        } else if first == self.g {
            new_color.g = second;
        } else if first == self.b {
            new_color.b = second;
        } else if first == self.a {
            new_color.a = second;
        }

        if second == self.r {
            new_color.r = first;
        } else if second == self.g {
            new_color.g = first;
        } else if second == self.b {
            new_color.b = first;
        } else if second == self.a {
            new_color.a = first;
        }

        new_color
    }
}