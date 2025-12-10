use std::{thread, time};
use std::io;
fn main() {
    let ball_layer1 = " /-----\\".to_string();
    let ball_layer2 = "| ◕   ◕ |".to_string();
    let ball_layer3 = "|  ‿‿‿  |".to_string();
    let ball_layer4 = " \\_____/".to_string();
    let mut delay_set: u64 = 10;
    loop{
    let mut set_speed = String::new();
    println!("Pick a speed: Fast-[1], Slow-[2], Medium-[3], MegaFast-[4]");
    io::stdin()
        .read_line(&mut set_speed)
        .expect("Failed to read line");
    let set_speed: u32 = match set_speed.trim().parse(){
        Ok(1) => {delay_set = 10; break},
        Ok(2) => {delay_set = 100; break},
        Ok(3) => {delay_set = 50; break},
        Ok(4) => {delay_set = 3; break},
        Ok(num) => continue,
        Err(_) => continue,
    };
    }
    loop{
        draw_frame(&ball_layer1, &ball_layer2, &ball_layer3, &ball_layer4, delay_set);
    }
}
fn draw_frame(l1:&String, l2:&String, l3:&String, l4:&String, delay_milli:u64){
    let delay_time = time::Duration::from_millis(delay_milli);
    //10 fast, 50, normal, 100 slow)
    let now = time::Instant::now();
    for frame_number in 1..=100{
        let cl1 = l1.clone();
        let cl2 = l2.clone();
        let cl3 = l3.clone();
        let cl4 = l4.clone();
        let returned_frame1: String = prepend_whitespace(cl1, frame_number);
        let returned_frame2: String = prepend_whitespace(cl2, frame_number);
        let returned_frame3: String = prepend_whitespace(cl3, frame_number);
        let returned_frame4: String = prepend_whitespace(cl4, frame_number);
        println!("{returned_frame1}");
        println!("{returned_frame2}");
        println!("{returned_frame3}");
        println!("{returned_frame4}");
        print!("{}[2J", 27 as char);
        thread::sleep(delay_time);    
    }
    for frame_number in (1..=100).rev(){
        let cl1 = l1.clone();
        let cl2 = l2.clone();
        let cl3 = l3.clone();
        let cl4 = l4.clone();
        let returned_frame1: String = prepend_whitespace(cl1, frame_number);
        let returned_frame2: String = prepend_whitespace(cl2, frame_number);
        let returned_frame3: String = prepend_whitespace(cl3, frame_number);
        let returned_frame4: String = prepend_whitespace(cl4, frame_number);
        println!("{returned_frame1}");
        println!("{returned_frame2}");
        println!("{returned_frame3}");
        println!("{returned_frame4}");
        thread::sleep(delay_time);
        print!("{}[2J", 27 as char);
    }
}
fn prepend_whitespace(frame:String, frame_num:i32) -> String{
    let mut returned_frame1: String  = "".to_string();
    let mut whitespace_string: String = "".to_string();
    for whitesspace_amount in 1..=frame_num{
       whitespace_string.push_str(" ");
    }
    returned_frame1 = whitespace_string;
    returned_frame1.push_str(&frame);
    return returned_frame1
}


