//! ExtString is an attempt to bring string functions from other programming languages to the Rust std String struct


/// The trait that adds functionality to the String struct.
pub trait ExtString {
    /// Reverses order of characters
    fn reverse(&self) -> String;

    /// Pads the left side of a string by repeating the same character until 'pad_len' is reached.
    /// If pad_len is shorter or equal to the character length, a simple cloned string will be returned.
    fn pad_left(&self, pad_len: usize, c: char) -> String;
    /// Pads the right side of a string by repeating the same character until 'pad_len' is reached.
    /// If pad_len is shorter or equal to the character length, a simple cloned string will be returned.
    fn pad_right(&self, pad_len: usize, c: char) -> String;
    /// Pads the left side of a string by repeating the same string slice until 'pad_len' is reached.
    /// If pad_len is shorter or equal to the character length, a simple cloned string will be returned.
    fn pad_left_str(&self, pad_len: usize, s: &str) -> String;
    /// Pads the right side of a string by repeating the same string slice until 'pad_len' is reached.
    /// If pad_len is shorter or equal to the character length, a simple cloned string will be returned.
    fn pad_right_str(&self, pad_len: usize, s: &str) -> String;
    /// Checks that all characters in a string are numeric characters.
    fn is_numeric(&self) -> bool;
    /// Checks that all characters in a string are alphabetic characters.
    fn is_alphabetic(&self) -> bool;
}

impl ExtString for String {
    /// Reverses order of characters
    fn reverse(&self) -> String {
        self.chars().rev().collect::<String>()
    }

    /// Pads the left side of a string by repeating the same character until 'pad_len' is reached.
    /// If pad_len is shorter or equal to the character length, a simple cloned string will be returned.
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

    /// Pads the right side of a string by repeating the same character until 'pad_len' is reached.
    /// If pad_len is shorter or equal to the character length, a simple cloned string will be returned.
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
        if pad_len <= count || s.is_empty() {
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
        if pad_len <= count || s.is_empty()  {
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
    
    /// Checks that all characters in a string are numeric characters.
    fn is_numeric(&self) -> bool {
      
        for c in self.chars() {
            if !c.is_numeric() {
                return false
            }
        }
        true  
    }
    
    /// Checks that all characters in a string are alphabetic characters.
    fn is_alphabetic(&self) -> bool {
      
        for c in self.chars() {
            if !c.is_alphabetic() {
                return false
            }
        }
        true  
    }
}

#[cfg(test)]
mod tests {
    use crate::ExtString;

    #[test]
    fn test_reverse() {
        let original = String::from("123456789");
        assert_eq!(original.reverse(), "987654321");
        let chinese = String::from("汉字漢字");
        assert_eq!(chinese.reverse(), "字漢字汉");
        let mangled = String::from("גבאabc1汉字漢字");
        assert_eq!(mangled.reverse(), "字漢字汉1cbaאבג");
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
    
    #[test]
    fn test_is_numeric() {
        assert!(String::from("123456").is_numeric());
        assert!(String::from("000100").is_numeric());
        assert!(!String::from("123v56").is_numeric());
        assert!(!String::from("-123v56").is_numeric());
    }
    
    #[test]
    fn test_is_alphabetic() {
        assert!(String::from("abcאבג").is_alphabetic());
        assert!(String::from("literal").is_alphabetic());
        assert!(!String::from("v1234").is_alphabetic());
        assert!(!String::from("6v7777").is_alphabetic());
    }
}
