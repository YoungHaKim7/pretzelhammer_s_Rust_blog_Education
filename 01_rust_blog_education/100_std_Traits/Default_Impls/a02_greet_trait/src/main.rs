trait Greet {
    fn greet(&self, name: &str) -> String;
    fn greet_loudly(&self, name: &str) -> String {
        self.greet(name) + "! (trait) auto"
    }
}

struct Hello;
struct Hola;

impl Greet for Hello {
    fn greet(&self, name: &str) -> String {
        format!("(impl + trait)Hello, {}!", name)
    }
}

impl Greet for Hola {
    fn greet(&self, name: &str) -> String {
        format!("(trait)Hola, {}!", name)
    }

    fn greet_loudly(&self, name: &str) -> String {
        let mut greeting = self.greet(name);
        greeting.insert(0, 'i');
        greeting + "! hola !!!!"
    }
}

fn main() {
    println!("{}", Hello.greet("John"));
    println!("{}", Hello.greet_loudly("John greet_loudly"));
    println!("{}", Hola.greet("Hola John"));
    println!("{}", Hola.greet_loudly("(greet_loudly) John"));
}
