mod database {
    #[derive(Debug)]
    pub struct Person {
        id: u32,
        email: String,
        name: String,
        age: u8,
    }

    #[derive(Debug)]
    pub struct Db {
        file: String,
        persons: Vec<Person>,
    }

    impl Person {
        pub fn new(email: String, name: String, age: u8) -> Person {
            Person {
                id: 0,
                email: email,
                name: name,
                age: age,
            }
        }

        pub fn get_email(&self) -> &String {
            &self.email
        }

        pub fn get_name(&self) -> &String {
            &self.name
        }

        pub fn get_age(&self) -> u8 {
            self.age
        }
    }

    impl Db {
        pub fn new(file: String) -> Db {
            Db {
                file: file,
                persons: Vec::new(),
            }
        }

        pub fn load(file: String) -> Db {
            Db::new(file)
        }

        pub fn add(&mut self, person: Person) {
            let mut new_person = Person::new(person.email, person.name, person.age);
            new_person.id = (self.persons.len() as u32) + 1;

            self.persons.push(new_person);
        }

        pub fn remove(&mut self, id: u32) -> bool {
            let mut index = 0;
            let mut found = false;
            for person in &self.persons {
                if person.id == id {
                    found = true;
                    break;
                }
                index += 1;
            }

            if found {
                self.persons.remove(index);
            }

            found
        }

        pub fn save(&self) {}
    }
}
