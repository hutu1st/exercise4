pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

pub trait TrafficLightTime {
    fn time(&self) -> u8;
}

impl TrafficLightTime for TrafficLight {
     fn time(&self) -> u8 {
        match self {
            TrafficLight::Red => 10,
            TrafficLight::Green => 20,
            TrafficLight::Yellow => 30,
        }
    }
}
