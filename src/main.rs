use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "RGB ({r}, {g}, {b}) 0x{r:02X}{g:02X}{b:02X}",
            r = self.red,
            g = self.green,
            b = self.blue
        )
    }
}

fn main() {
    debug_print();
    display_color();
}

fn debug_print() {
    // used.
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // FIXME ^ Add the missing argument: "James" ✅

    // Create a structure named `Structure` which contains an `i32`.
    #[derive(Debug)]
    struct Structure(i32);

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    println!("This struct `{:?}` won't print...", Structure(3));
    // FIXME ^ Comment out this line. ✅
}

fn display_color() {
    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ]
    .iter()
    {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{}", *color);
    }
}
