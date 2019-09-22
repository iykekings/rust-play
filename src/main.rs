use std::io::stdin;
use std::process::exit;

fn main() {
    let mut gp = 0_u32;
    let mut units = 0_u32;

    for i in 1..=10 {
        let mut raw_str = String::new();
        println!("Input unit load for course {}", i);
        stdin().read_line(&mut raw_str).unwrap();
        let unit = raw_str.trim().parse::<u32>().unwrap();
        units += unit;

        println!("Input grade for course {}", i);
        let mut grade = String::new();
        stdin().read_line(&mut grade).unwrap();
        let grade = grade.trim();

        match grade.to_uppercase().as_ref() {
            "A" => gp += unit * 5,
            "B" => gp += unit * 4,
            "C" => gp += unit * 3,
            "D" => gp += unit * 2,
            "E" => gp += unit * 1,
            "F" => gp += unit * 0,
            _ => {
                println!("error");
                exit(0)
            }
        }
    }
    println!("Your GPA is: {:.2}", gp as f32 / units as f32);
}
