// Exercise 3.6: 17 or older, or 13 or older with a parent's permission.
fn can_see_movie(age: i32, permission: bool) -> bool {
    age >= 17 || (age >= 13 && permission)
}

fn main() {
    println!("John who is 18, can see the movie: {}", can_see_movie(18, true)); // John who is 18, can see the movie: true
    println!("  age  permission  can_see_movie");
    for age in [12, 13, 16, 17, 18] {
        for permission in [false, true] {
            println!("  {age:>3}  {permission:<10}  {}", can_see_movie(age, permission));
        }
    }
    // 12: false false; 13: false true; 16: false true; 17: true true; 18: true true
}
