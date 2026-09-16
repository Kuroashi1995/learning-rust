#[derive(Debug)]
enum Permissions {
    Read,
    Write,
    Delete,
}

#[derive(Debug)]
struct ManagerStruct {
    name: String,
    permissions: [Permissions; 2],
}

struct AdminStruct {
    name: String,
    permissions: [Permissions; 3],
}

enum User {
    Visitor(String), // We only need the name of the visitor, so there is no need for a struct
    Manager(ManagerStruct),
    Admin(AdminStruct),
}

fn main() {
    let visitor = User::Visitor(String::from("John Visitor"));
    let manager = User::Manager(ManagerStruct { name: String::from("John Manager"), permissions: [Permissions::Read, Permissions::Write] });
    let admin = User::Admin(AdminStruct { name: String::from("John Admin"), permissions: [Permissions::Write, Permissions::Read, Permissions::Delete]});

    salute(visitor);
    salute(manager);
    salute(admin);

}

fn salute(user: User) {
    match user {
        User::Visitor(name) => {
            println!("Hello {name:?}");
        },
        User::Manager(manager) => {
            println!("Hello, mr. {manager:?}, your permissions are R & W");
        },
        User::Admin(_) => {
            println!("Hello mr. Stark, full access granted");
        }
    }
}
