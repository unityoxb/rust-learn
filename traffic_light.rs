fn main() {
    let light = TrafficLight::Green.traffic_light_time();
    println!("此灯要亮{}秒", light);
}

// 定义 Trait 接口
pub trait OutputTime{
    // 定义一个模式匹配方法，返回各种信号灯的时间
    fn traffic_light_time(&self) -> u8;
}


// 定义信号灯的枚举
pub enum TrafficLight {
    Red,
    Green, 
    Yellow,
}


impl OutputTime for TrafficLight {
    // 实现trait方法
    fn traffic_light_time(&self) -> u8 {
        match self {
            TrafficLight::Red => 40,
            TrafficLight::Green => 70,
            TrafficLight::Yellow => 10,
        }
    }
}
