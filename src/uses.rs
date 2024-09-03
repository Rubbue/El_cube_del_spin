
#[derive(Clone, Copy)]
pub struct Point {
    pub x:f64,
    pub y:f64,
    pub z:f64
}

impl Point {

    pub fn sub(&self,p:&Point) -> Point{
        return Point {x:self.x-p.x,y:self.y-p.y,z:self.z-p.z};
    }
    
    pub fn cross(&self,p:&Point) -> Point{
        return Point {x:(self.y*p.z-self.z*p.y),y:-(self.x*p.z-self.z*p.x),z:(self.x*p.y-self.y*p.x)};
    }

    pub fn dot(&self,p:&Point) -> f64{
        return self.x*p.x + self.y*p.y + self.z*p.z;
    }

    pub fn scale(&mut self,scalar:f64) {

        self.x = self.x*scalar;
        self.y = self.y*scalar;
        self.z = self.z*scalar;

    }
}

pub struct Side {
    pub corners:[Point;4], //tl,tr,br,bl
}

impl Side {

    pub fn normalize(&mut self,cube_size:usize) {
        
        for i in self.corners.iter_mut() {

            let norm_len:f64 = 1.0/i.dot(i).sqrt();

            i.scale(norm_len);

            i.scale((cube_size as f64)*(3 as f64).sqrt()/2.0)

        }

        
    }
    
}