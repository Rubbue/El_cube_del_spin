
#[derive(Clone, Copy)]
pub struct Point {
    pub x:f64,
    pub y:f64,
    pub z:f64
}

impl Point {

    pub fn sub(&self,p:Point) -> Point{
        return Point {x:self.x-p.x,y:self.y-p.y,z:self.z-p.z};
    }
    
    pub fn cross(&self,p:Point) -> Point{
        return Point {x:(self.y*p.z-self.z*p.y),y:-(self.x*p.z-self.z*p.x),z:(self.x*p.y-self.y*p.x)};
    }

    pub fn dot(&self,p:Point) -> f64{
        return self.x*p.x + self.y*p.y + self.z*p.z;
    }
}

pub struct Side {
    pub corners:[Point;4] //tl,tr,br,bl
}
