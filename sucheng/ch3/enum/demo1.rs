#[derive(Debug)]
enum Sex {
    Male,
    Female,
}

#[derive(Debug)]
enum SexType {
    Male(String),
    Female(String),
}

#[derive(Debug)]
enum SexTypeMutli {
    Male(String, i32),
    Female(String, i32),
}

#[derive(Debug)]
struct User {
    id: i32,
    sex: Sex,
    sex_multi: SexTypeMutli,
}

fn Check(u : User) {
    match u.sex {
        Sex::Male => {
            println!("{}", "man");
        },
        Sex::Female => {
            println!("{}", "lady");
        }
    };
}

fn check_if_let(u: User) {
    if let Sex::Female = u.sex {
        println!("lady");
    }
}

fn check_multi(u: User) {
    if let SexTypeMutli::Male(s, i) = u.sex_multi {
        println!("{},{}",s, i);
    }
}

fn main() {
    println!("{:?}", Sex::Male);
    println!("{:?}", SexType::Male(String::from("man")));
    println!("{:?}", SexTypeMutli::Male(String::from("man"), 0));

    let u = User {
        id : 101,
        sex: Sex::Male,
        sex_multi: SexTypeMutli::Male(String::from("man"), 0),
    };
    println!("{:?}", u);

    Check(u);

    let u = User {
        id : 101,
        sex: Sex::Female,
        sex_multi: SexTypeMutli::Male(String::from("man"), 0),
    };
    check_if_let(u);

    let u = User {
        id : 101,
        sex: Sex::Female,
        sex_multi: SexTypeMutli::Male(String::from("man"), 1),
    };
    check_multi(u);
}