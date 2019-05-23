pub trait ExtString {
    fn reverse(&self) -> String;

    fn pad_left(&self, pad_len: usize, c: char) -> String;

    fn pad_right(&self, pad_len: usize, c: char) -> String;

    fn pad_left_str(&self, pad_len: usize, s: &str) -> String;

    fn pad_right_str(&self, pad_len: usize, s: &str) -> String;
}

impl ExtString for String {
    fn reverse(&self) -> String {
        let mut n = String::with_capacity(self.len());
        for c in self.chars().rev() {
            n.push(c);
        }
        n
    }

    fn pad_left(&self, pad_len: usize, c: char) -> String {
        let count = self.chars().count();
        if pad_len <= count {
            return self.clone();
        }
        let repeat = pad_len - count;
        let mut pad = String::new();
        for _ in 0..repeat {
            pad.push(c);
        }
        pad.push_str(self);
        pad
    }

    fn pad_right(&self, pad_len: usize, c: char) -> String {
        let count = self.chars().count();
        if pad_len <= count {
            return self.clone();
        }
        let repeat = pad_len - count;
        let mut pad = String::new();
        pad.push_str(self);
        for _ in 0..repeat {
            pad.push(c);
        }
        pad
    }

    fn pad_left_str(&self, pad_len: usize, s: &str) -> String {
        let count = self.chars().count();
        if pad_len <= count {
            return self.clone();
        }
        
        let repeat = pad_len - count;
        let repeat_len = s.chars().count();
        let mut pad = String::new();
        for index in 0..repeat {    
            pad.push(s.chars().nth(index % repeat_len).unwrap());
        }
        pad.push_str(self);
        pad 
    }
    
    fn pad_right_str(&self, pad_len: usize, s: &str) -> String {
        let count = self.chars().count();
        if pad_len <= count {
            return self.clone();
        }
        
        let repeat = pad_len - count;
        let repeat_len = s.chars().count();
        let mut pad = String::new();
        pad.push_str(self);
        for index in 0..repeat {    
            pad.push(s.chars().nth(index % repeat_len).unwrap());
        }
        pad 
    }
}

#[cfg(test)]
mod tests {
    use crate::ExtString;

    #[test]
    fn test_reverse() {
        let original = String::from("123456789");
        assert_eq!(original.reverse(), "987654321");
    }

    #[test]
    fn test_pad_left() {
        let s = "12345";
        let space = ' ';
        assert_eq!("12345", String::from(s).pad_left(3, space));
        assert_eq!("     12345", String::from(s).pad_left(10, space));
    }

    #[test]
    fn test_pad_right() {
        let s = "12345";
        let space = ' ';
        assert_eq!("12345", String::from(s).pad_right(3, space));
        assert_eq!("12345     ", String::from(s).pad_right(10, space));
    }
    
    #[test]
    fn test_pad_left_str() {
        let s = "12345";
        let padding = "qwerty";
        assert_eq!("qwerty12345", String::from(s).pad_left_str(11, padding));
        assert_eq!("qwertyqwe12345", String::from(s).pad_left_str(14, padding));
    }
    
    #[test]
    fn test_pad_right_str() {
        let s = "12345";
        let padding = "qwerty";
        assert_eq!("12345qwerty", String::from(s).pad_right_str(11, padding));
        assert_eq!("12345qwertyqwe", String::from(s).pad_right_str(14, padding));
    }
}
