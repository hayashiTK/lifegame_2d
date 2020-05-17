
fn main() {
    //let mut line = [0; 10];
    let mut line = [false; 10];
    line[1] = true;
    line[3] = true;
    print_line(&line);
}

fn print_line(line: &[bool]) {
    for cell in line {
      if *cell == true {
        print!("■");
      } else {
        print!("□");
      }
    }
    println!("");
}
