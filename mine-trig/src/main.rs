use std::io;
use std::io::Write;

#[inline(always)]
fn n_or_s (e1: f32, e2:f32, e3: f32 ){

    let tan9h1:f32 = (90.0 - e2).to_radians().tan();
    let tan9h2:f32 = (90.0 - e3).to_radians().tan();

    let x:f32 = -(e1/ (tan9h1 - tan9h2));

    println!("O valor de X é: {:.0}", x);

    let z:f32 = (e1 * tan9h1) / (tan9h1 - tan9h2);
    println!("O valor de Z é: {:.0}", z);

}
#[inline(always)]
fn w_or_e (e1: f32, e2:f32, e3: f32 ){

    let tanh1:f32 = (e2).to_radians().tan();
    let tanh2:f32 = (e3).to_radians().tan();

    let x: f32 = (e1 * tanh1) / (tanh1 - tanh2);

    println!("O valor de X é: {:.0}", x);

    let z: f32 = - (e1/ (tanh1 - tanh2));
    println!("O valor de Z é: {:.0}", z);

}

fn ln() -> f32{
    let mut x:String = String::new();
    print!("What is the second angle?\n--> ");
    io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut x).expect("erFlnL");
    x.trim().parse().expect("erFlnC")
}

fn main() {
    let mut b: String = String::new();
    print!("What is the first angle?\n--> ");
    io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut b).expect("rl1");
    let c1:f32= b.trim().parse().expect("cnv1");
    b.clear();
    print!("Digite:\n[1]North\n[2]South\n[3]West\n[4]East\nWhich direction appeared for this angle?\n--> ");
    io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut b).expect("rl2");
    let d:u8 = b.trim().parse().expect("cnv2");
    match d {
        1 => {
            println!("Go to (0,-350)");
            n_or_s(-350.0,c1,ln());
        },
        2 => {
            println!("Go to (0,350)");
            n_or_s(350.0,c1,ln());
        },
        3 => {
            println!("Go to (-350,0)");
            w_or_e(-350.0,c1,ln());
        },
        4 => {
            println!("Go to (350,0)");
            w_or_e(350.0,c1,ln());
        },
        _ => {
            println!("Invalid entry");
        },
    }
    
}