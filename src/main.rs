use std::default::Default;
use mysql::*;
use mysql::prelude::*;
use mysql::Pool;
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

fn main() {
    test_enum();
    test_trait();
}

enum MyEnum {
    TypeA(TypeA),
    TypeB(TypeB),
    TypeC(TypeC),
}

struct TypeA;
impl TypeA {
    fn method_a(&self) {
        println!("TypeA method called");
    }
}

struct TypeB;
impl TypeB {
    fn method_b(&self) {
        println!("TypeB method called");
    }
}

struct TypeC;
impl TypeC {
    fn method_c(&self) {
        println!("TypeC method called");
    }
}

fn test_enum() {
    let vec: Vec<MyEnum> = vec![
        MyEnum::TypeA(TypeA),
        MyEnum::TypeB(TypeB),
        MyEnum::TypeC(TypeC),
    ];
    for item in vec {
        match item {
            MyEnum::TypeA(type_a) => type_a.method_a(),
            MyEnum::TypeB(type_b) => type_b.method_b(),
            MyEnum::TypeC(type_c) => type_c.method_c(),
        }
    }
}

trait MyTrait {
    fn my_method(&self);
}

struct Type_A;
impl MyTrait for Type_A {
    fn my_method(&self) {
        println!("TypeA method called");
    }
}

struct Type_B;
impl MyTrait for Type_B {
    fn my_method(&self) {
        println!("TypeB method called");
    }
}

struct Type_C;
impl MyTrait for Type_C {
    fn my_method(&self) {
        println!("TypeC method called");
    }
}

fn test_trait() {
    let vec: Vec<Box<dyn MyTrait>> = vec![
        Box::new(Type_A),
        Box::new(Type_B),
        Box::new(Type_C),
    ];

    for item in vec {
        item.my_method();
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Student {
    id: i32,
    name: String,
    age: i32,
    club: Club,
    classroom: Classroom,
    schedule: Schedule,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Club {
    id: i32,
    name: String,
    member_count: i32,
    students: [Student],
}


#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Classroom {
    id: u32,
    name: String,
    student_count: u32,
    students: [Student],
}


#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Schedule {
    id: u32,
    course_name: String,
    teacher: String,
    date: NaiveDate,
}

fn get_db_conn() {
    let url = "mysql://root:password@localhost:3306/database";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    return &conn
}

fn create_student() {
    let student = Student {
        id: 1,
        name: String::from("John Doe"),
        age: 18,
        club: Club {
            id: 0,
            name: (),
            member_count: 0,
            students: []
        },
        classroom: Classroom {
            id: 0,
            name: (),
            student_count: 0,
            students: []
        },
        schedule: Schedule {
            id: 0,
            course_name: (),
            teacher: (),
            date: ()
        }
    };

    conn.exec_drop(
        r"INSERT INTO students (id, name, age) VALUES (:id, :name, :age)",
        params! {
            "id" => student.id,
            "name" => &student.name,
            "age" => student.age,
        },
    ).unwrap();
}

fn create_club() {
    let club = Club {
        id: 1,
        name: "Coding Club".to_string(),
        member_count: 20,
        students: []
    };

    conn.exec_drop(
        r"INSERT INTO Club (id, name, member_count)
        VALUES (:id, :name, :member_count)",
        params! {
            "id" => club.id,
            "name" => &club.name,
            "member_count" => club.member_count,
        },
    ).unwrap();
}

fn creat_class_room() {
    let classroom = Classroom {
        id: 1,
        name: "Class 1A".to_string(),
        student_count: 30,
        students: []
    };

    conn.exec_drop(
        r"INSERT INTO classrooms (id, name, student_count) VALUES (:id, :name, :student_count)",
        params! {
            "id" => classroom.id,
            "name" => &classroom.name,
            "student_count" => classroom.student_count,
        },
    ).unwrap();
}

fn create_schedule() {
    let schedule = Schedule {
        id: 1,
        course_name: "Mathematics".to_string(),
        teacher: "John Doe".to_string(),
        date: NaiveDate::from_ymd(2020, 3, 14),
    };

    conn.exec_drop(
        r"INSERT INTO schedules (id, course_name, teacher, date)
            VALUES (:id, :course_name, :teacher, :date)",
        params! {
            "id" => schedule.id,
            "course_name" => &schedule.course_name,
            "teacher" => &schedule.teacher,
            "date" => schedule.date,
        }
    ).unwrap();
}

fn print_chart_list() {
    println!("start a-Z");
    module1::print_a_Z();
    println!("start A-z");
    module1::module2::print_A_z();
}

fn test_ownership() {
    let mut a1 = String::from("I love Rust");
    let a2 = &mut a1;
    *a2 = String::from("I love Rust!");
    let a3 = a2;
    foo(a3);
    // let a4 = &a2;
    // let a5 = a2;

    println!("{a3}");
    println!("{a1}");
}

fn foo(s: &mut String) {
    s.push_str(" too!")
}
