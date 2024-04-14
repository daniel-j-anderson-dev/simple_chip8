/// original keypad
/// ```md
/// =================
/// | 1 | 2 | 3 | C |
/// | 4 | 5 | 6 | D |
/// | 7 | 8 | 9 | E |
/// | A | 0 | B | F |
/// =================
/// ```
///
/// QWERTY keyboard mapping
///
/// ```md
/// =================
/// | 1 | 2 | 3 | 4 |
/// | Q | W | E | R |
/// | A | S | D | F |
/// | Z | X | C | V |
/// =================
/// ```
#[derive(Debug)]
pub struct Keypad([[bool; 4]; 4]);

impl Default for Keypad {
    fn default() -> Self {
        return Self([[false; 4]; 4]);
    }
}
