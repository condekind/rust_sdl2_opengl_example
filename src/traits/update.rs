
/*
pub trait Updatable {
    fn update<T>(&mut self, args: T);
}

pub enum UpdateArgs<A, > {
    One(i32),
    Two(i32, f64),
    // more variants for possible combinations of types
}

pub struct MyStruct {
    pub value: i32,
}

impl Updatable for MyStruct {
    fn update(&mut self, args: UpdateArgs) {
        match args {
            UpdateArgs::One(v) => {
                // handle one argument
                self.value = v;
            }
            UpdateArgs::Two(v1, v2) => {
                // handle two arguments
                self.value = (v1 as f64 * v2) as i32;
            }
            // same for other variants
        }
    }
}


*/