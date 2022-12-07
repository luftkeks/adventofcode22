use std::{
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
    rc::Rc,
};

struct Node {
    value: i32,
    name: String,
    dirs: Vec<Rc<Node>>,
    parent: Option<Rc<Node>>,
}

impl Node {
    fn is_empty(&self) -> bool {
        self.dirs.is_empty()
    }

    fn sum_of_content(&self) -> i32 {
        if self.is_empty() {
            return self.value;
        }

        self.dirs.iter().map(|p| p.sum_of_content()).sum()
    }
}

// das ist 1:1 aus ChatGPT geklaut
impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Berechnen Sie hier die Hash-Werte für die Felder des struct und fügen Sie sie dem
        // state-Hasher hinzu. Beispiel:
        state.write_i32(self.value);
        state.write(self.name.as_bytes());
        // ... und so weiter für alle Felder des struct.
    }
}

pub fn day7() {
    let input = std::fs::read_to_string("input6.txt").unwrap();

    let first_Node: Node = Node {
        value: 0,
        dirs: vec![],
        name: String::from("/"),
        parent: None,
    };
    let mut now = Rc::new(first_Node);
    for line in input.lines() {
        let (_, nameRaw) = line.trim().split_at(4);
        let name = String::from(nameRaw);
        if line.starts_with("$") {
            if line.contains("cd") {}
        } else if line.starts_with("dir") {
            let mut new = Node {
                value: 0,
                dirs: vec![],
                name: name,
                parent: Some(Rc::clone(&now)),
            };
            let new2 = Rc::new(new);
            now.dirs.push(Rc::clone(&new2));
        } else {
            let (number, name) = line.trim().split_once(" ").unwrap();
            let new = Node {
                value: number.parse::<i32>().unwrap(),
                dirs: vec![],
                name: String::from(name),
                parent: Some(now),
            };
            now.dirs.push(Rc::new(new));
        }
    }

    //This might never be finished because of: KEIN BOCK
}
