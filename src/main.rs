use crate::traffic_light::TrafficLightTime;
use crate::traffic_light::TrafficLight;
use crate::area::{*};

mod traffic_light;
mod summary;
mod area;

fn main() {
    // 第一题
    let red = TrafficLight::Red;
    println!("{}", red.time());

    let red = traffic_light::TrafficLight::Yellow;
    println!("{}", red.time());

    let red = traffic_light::TrafficLight::Green;
    println!("{}", red.time());


    // 第二题
    if let Some(res) = summary::summary(&[4294967295, 0]) {
        println!("{}", res);
    };

    if let Some(res) = summary::summary(&[4294967295, 20]) {
        println!("{}", res);
    };

    // 第三题
    let c = Circle{r:2.0};
    print_area(&c);

    let s = Square{s:3.0};
    print_area(&s);

}

