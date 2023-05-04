use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub struct Company {
    people: HashMap<String, Vec<String>>,
}

impl Company {
    pub fn new() -> Self {
        Self {
            people: HashMap::new(),
        }
    }

    pub fn list_all(&self) {
        for (key, val) in self.people.iter() {
            println!("Department: {} has: {:?}", *key, *val)
        }
    }

    pub fn list(&mut self, dep: &str) {
        let people_list = self.people.entry(dep.to_string());

        match people_list {
            Entry::Occupied(vec) => {
                println!("Department: {}, has: {:?}", dep, vec.get())
            }
            _ => println!("That department does not exist!"),
        }
    }

    pub fn adjust(&mut self, s: &str) -> Option<()> {
        let operation = Operation::new(s)?;

        match operation.op.to_lowercase().as_str() {
            "add" => {
                self.add(&operation.name, &operation.department);
            }
            "remove" => {
                self.remove(&operation.name, &operation.department);
            }
            _ => {}
        }

        Some(())
    }

    fn add(&mut self, person: &str, dep: &str) {
        self.people
            .entry(dep.to_string())
            .or_insert(Vec::new())
            .push(person.to_string());
    }

    fn remove(&mut self, person: &str, dep: &str) {
        let entry = self.people.entry(dep.to_string());

        match entry {
            Entry::Occupied(vec) => {
                let vec = vec.into_mut();

                if let Some(pos) = vec.iter().position(|x| *x == person) {
                    vec.remove(pos);
                }
            }
            _ => {}
        }
    }
}

struct Operation {
    op: String,
    name: String,
    department: String,
}

impl Operation {
    pub fn new(s: &str) -> Option<Self> {
        let words: Vec<&str> = s.split_whitespace().collect();

        if words.len() < 4 {
            return None;
        }

        let op = String::from(words[0]);
        let name = String::from(words[1]);
        let department = String::from(words[3]);

        Some(Self {
            op,
            name,
            department,
        })
    }
}

#[cfg(test)]
mod test_operation {
    use super::Operation;

    #[test]
    fn new_operation_valid() {
        let operation = Operation::new("add phil to engineering").unwrap();

        assert_eq!(operation.op, "add");
        assert_eq!(operation.name, "phil");
        assert_eq!(operation.department, "engineering");
    }

    #[test]
    fn new_operation_invalid() {
        let operation = Operation::new("bad");
        assert!(operation.is_none());
    }
}

#[cfg(test)]
mod test_company {
    use super::Company;

    #[test]
    fn test_adjust_add() {
        let mut company = Company::new();
        let res = company.adjust("add phil to engineering").unwrap();

        assert_eq!(res, ());
    }

    #[test]
    fn test_adjust_remove() {
        let mut company = Company::new();
        let res = company.adjust("remove phil from engineering").unwrap();

        assert_eq!(res, ());
    }

    #[test]
    fn test_adjust_invalid() {
        let mut company = Company::new();
        assert!(company.adjust("bad").is_none());
    }
}
