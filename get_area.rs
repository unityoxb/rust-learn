#[derive(Debug)]

// 三角形
pub struct Triangle{
    bottom: f32,
    height: f32,
}

// 长方形
pub struct Rectangle{
    width: f32, 
    height: f32,
}

// 圆形
pub struct Round{
    radius: f32,
}

// 实现一个trait接口
pub trait Area {
    // 实现一个计算面积的方法
    fn getarea(&self) -> f32;
} 

impl Area for Triangle {
    fn getarea(&self) -> f32 {
        self.bottom * self.height * 0.5
    }
}

impl Area for Rectangle {
    fn getarea(&self) -> f32 {
        self.width * self.height
    }
}

impl Area for Round {
    fn getarea(&self) -> f32 {
        self.radius * self.radius * std::f32::consts::PI
    }
}



fn main() {
    let tri = Triangle{
        bottom: 10.5,
        height: 4.5,
    };
    let rec = Rectangle{
        width: 20.3,
        height: 5.8,
    };
    let round = Round{
        radius: 32.5,
    };

    println!("三角形面积:{}", tri.getarea());
    println!("长方形面积:{}", rec.getarea());
    println!("圆形面积:{}", round.getarea());

}
