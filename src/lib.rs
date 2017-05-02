#[macro_use]
extern crate helix;

use helix::{UncheckedValue, ToRust};

ruby! {
    reopen class RubyString {
        def camelcase(&self) -> String {
            let mut buffer = String::new();
            let native_str = self.to_string();
            let mut under = false;
            for c in native_str.chars() {
                match c {
                    '_' => under = true,
                    _ => {
                        if under {
                            buffer.push_str(&c.to_uppercase().to_string());
                            under = false;
                        } else {
                            buffer.push(c)
                        }
                    },
                }
            }
            buffer
        }

        def snakecase(&self) -> String {
            let mut buffer = String::new();
            let rust_str = self.to_string();
            let mut rust_chars = rust_str.chars().peekable();

            let mut last_alpha = false;
            let mut last_upper = false;

            while let Some(c) = rust_chars.next() {
                if c.is_uppercase() {
                    if let Some(ch) = rust_chars.peek() {
                        if !last_upper && last_alpha {
                            buffer.push('_');
                            if !ch.is_uppercase() {
                                buffer.push_str(&c.to_lowercase().to_string());
                                continue
                            }
                        }
                    }
                    last_upper = true;
                } else { last_upper = false; }
                buffer.push(c);
                last_alpha = c.is_alphanumeric();
            }
            buffer
        }
    }
}

impl ToString for RubyString {
    fn to_string(&self) -> String {
        let checked = self.helix.to_checked().unwrap();
        checked.to_rust()
    }
}
