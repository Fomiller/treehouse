use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }

    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the treehouse, {}", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the treehouse, {}.", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}.", self.name);
                }
            }
            VisitorAction::Probation => println!("{} is now a probationary member.", self.name),
            VisitorAction::Refuse => println!("Do not allow {} in", self.name),
        }
    }
}

fn main() {
    let mut visitor_list = vec![
        Visitor::new("forrest", VisitorAction::Accept, 28),
        Visitor::new(
            "chuck",
            VisitorAction::AcceptWithNote {
                note: String::from("chuck is a cool cat."),
            },
            21,
        ),
        Visitor::new("bob", VisitorAction::Refuse, 30),
    ];
    loop {
        println!("Hello, what is your name? (leave emptyand press ENTER to quit)");
        let name = what_is_your_name();

        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} is not on the visitor list.", name);
                    let mut new_visitor_age = String::new();
                    println!("What is your age {}?", name);
                    stdin()
                        .read_line(&mut new_visitor_age)
                        .expect("failed to read line.");

                    visitor_list.push(Visitor::new(
                        &name,
                        VisitorAction::Probation,
                        new_visitor_age.trim().parse::<i8>().unwrap(),
                    ));
                }
            }
        }
    }
    println!("The final list of visitors:");
    println!("{:#?}", visitor_list);
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}
