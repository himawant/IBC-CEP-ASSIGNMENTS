struct Student {
    name: String,
    email: String,
    phone: String,
    id: u32,
}
fn main() {
    let students: Vec<Student> = vec![
        Student {
            name: String::from("Ali"),
            email: String::from("ali@example.com"),
            phone: String::from("1235997890"),
            id: 1,
        },
        Student {
            name: String::from("Bobesh"),
            email: String::from("bobesh@example.com"),
            phone: String::from("123456780"),
            id: 2,
        },
        Student {
            name: String::from("Boblu"),
            email: String::from("boblu@example.com"),
            phone: String::from("123456"),
            id: 3,
        },
        Student {
            name: String::from("maesh"),
            email: String::from("maesh@example.com"),
            phone: String::from("16780"),
            id: 4,
        },
        Student {
            name: String::from("dhon"),
            email: String::from("dhon@example.com"),
            phone: String::from("56780"),
            id: 5,
        },
        Student {
            name: String::from("givaaash"),
            email: String::from("givaash@example.com"),
            phone: String::from("1200067"),
            id: 6,
        },

    ];

    fn main() {
        let students: Vec<Student> = vec![
    
        ];
    
        // Example: Accessing a student by index
        let index = 1;
        if let Some(student) = students.get(index) {
            println!("Student at index {} is {:?}", index, student);
        } else {
            println!("Student at index {} does not exist.", index);
        }
    
        let index = 10; // Example invalid index
if let Some(student) = students.get(index) {
    println!("Student at index {} is {:?}", index, student);
} else {
    println!("Student at index {} does not exist.", index);
}

    }
    
}

