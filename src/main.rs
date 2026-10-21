use o_092::Hello ;

#[derive(Hello)]
struct Person {
    name: String,
    age: u32,
    email: String,
}

fn main() {
    Person::hello() ;
} /* Out:
hello from Person
*/
