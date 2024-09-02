pub trait Plane {
    fn fly(&self);
}

pub trait Bird {
    fn fly(&self);
}

pub struct Object;

impl Object {
    pub fn fly(&self) {
        println!("Do nothing");
    }
}

impl Plane for Object {
    fn fly(&self) {
        println!("Flying as Plane");
    }
}

impl Bird for Object {
    fn fly(&self) {
        println!("Flying as Bird");
    }
}

// Associated functions that are not methods (Don't have self parameter).
pub trait Pizza {
    fn pizza_name() -> String;
}

pub struct Pepperoni;

impl Pepperoni {
    pub fn pizza_name() -> String {
        String::from("Pepperoni")
    }
}

impl Pizza for Pepperoni {
    fn pizza_name() -> String {
        String::from("Pizza")
    }
}
