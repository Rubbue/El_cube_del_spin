use uses::{Point, Side};
use std::{f32::consts::PI, thread::sleep, time::Duration};

mod uses;

const WIDTH:usize = 600;
const HEIGHT:usize = 80;
const CUBE_SIDE:usize = 50;
const X_DEGREE:f64 = (PI as f64)*2.0/360.0;
const Y_DEGREE:f64 = (PI as f64)*0.2/360.0;
const Z_DEGREE:f64 = (PI as f64)*0.5/360.0;

const BACKGROUND:char = ' ';

const COLOR:[char;6] = ['#','£','*','.','?','&'];


fn rotate_x(p:&mut Side) {

   for i in p.corners.iter_mut() {
      i.y = i.y * X_DEGREE.cos() + i.z * X_DEGREE.sin();
      i.z = i.z * X_DEGREE.cos() - i.y * X_DEGREE.sin();
       
   }
}

fn rotate_y(p:&mut Side) {

   for i in p.corners.iter_mut() {
      i.x = i.x * Y_DEGREE.cos() - i.z * Y_DEGREE.sin();
      i.z = i.x * Y_DEGREE.sin() + i.z * Y_DEGREE.cos()
       
   }
}

fn rotate_z(p:&mut Side) {

   for i in p.corners.iter_mut() {
      i.x = i.x * Z_DEGREE.cos() + i.y * Z_DEGREE.sin();
      i.y = i.y * Z_DEGREE.cos() - i.x * Z_DEGREE.sin()
       
   }
}

fn get_z(s:&Side, x:f64,y:f64) -> f64 {
   let v_1:Point = s.corners[1].sub(s.corners[0]);
   let v_2:Point = s.corners[2].sub(s.corners[0]);

   let norm:Point = v_1.cross(v_2);

   let d:f64 = -norm.dot(s.corners[0]);

   return -(norm.x*x+norm.y*y+d)/norm.z;


}

fn to_buffer(side:&Side,buf:&mut [char],z_pos:&mut [f64],paint:char) {

   for i  in 0..HEIGHT  {
      for j in 0..WIDTH {
         
         let mut inside = false;
         let mut p1 = side.corners[0].clone();

         let x:f64 = (j as i32 - WIDTH as i32/2) as f64;
         
         let y:f64 = (-(i as i32) + HEIGHT as i32/2) as f64;

         for i in 1..5 {

            let p2 = side.corners[i%4].clone();
   
            if y > (p1.y).min(p2.y){

               if y <= p1.y.max(p2.y){

                   if x <= p1.x.max(p2.x){

                       let x_intersection:f64 = ((y)- p1.y) * (p2.x - p1.x) / (p2.y - p1.y) + p1.x;
                       
                       if (p1.x-p2.x).abs()<0.2 || x <= x_intersection {
                           
                           inside = !inside;
                           
                           }
                        }
                     }
                  }

            p1 = p2;
         }

         let z = get_z(side, x, y);

         if inside && z_pos[i*WIDTH+j]>z{

            buf[i*WIDTH+j] = paint;
            z_pos[i*WIDTH+j] = z;
            
         }

      }
      
   }
   
}

fn main() {
   let mut draw_buffer:[char;WIDTH*HEIGHT] = [BACKGROUND;WIDTH*HEIGHT];
   let mut z_pos:[f64;WIDTH*HEIGHT] = [(CUBE_SIDE as f64);WIDTH*HEIGHT];

   let o_p:f64 = (CUBE_SIDE as f64)/2.0;

   //Front,Back;Left,Right;Top,Bottom
   let side1:Side = Side { corners: [Point { x: -o_p, y:  o_p, z: -o_p,},Point { x:  o_p, y:  o_p, z: -o_p },
                                         Point { x:  o_p, y: -o_p, z: -o_p },Point { x: -o_p, y: -o_p, z: -o_p }] };  //#

   let side2:Side = Side { corners: [Point { x: -o_p, y:  o_p, z:  o_p,},Point { x:  o_p, y:  o_p, z:  o_p },
                                         Point { x:  o_p, y: -o_p, z:  o_p },Point { x: -o_p, y: -o_p, z:  o_p }] }; //¤

   let side3:Side = Side { corners: [Point { x: -o_p, y:  o_p, z:  o_p },Point { x: -o_p, y:  o_p, z: -o_p },
                                         Point { x: -o_p, y: -o_p, z: -o_p },Point { x: -o_p, y: -o_p, z:  o_p }] }; //%

   let side4:Side = Side { corners: [Point { x:  o_p, y:  o_p, z: -o_p,},Point { x:  o_p, y:  o_p, z:  o_p },
                                         Point { x:  o_p, y: -o_p, z:  o_p },Point { x:  o_p, y: -o_p, z: -o_p }] }; //&

   let side5:Side = Side { corners: [Point { x: -o_p, y:  o_p, z:  o_p,},Point { x:  o_p, y:  o_p, z:  o_p },
                                         Point { x:  o_p, y:  o_p, z: -o_p },Point { x: -o_p, y:  o_p, z: -o_p }] }; //@

   let side6:Side = Side { corners: [Point { x: -o_p, y: -o_p, z: -o_p,},Point { x:  o_p, y: -o_p, z: -o_p },
                                         Point { x:  o_p, y: -o_p, z:  o_p },Point { x: -o_p, y: -o_p, z:  o_p }] }; //$

   let mut sides:[Side;6] = [side1,side2,side3,side4,side5,side6];

   let mut forever = 1;

   while forever>0{

      for i in 0..6{
         rotate_x(&mut sides[i]);
         rotate_y(&mut sides[i]);
         rotate_z(&mut sides[i]);

         to_buffer(&sides[i], &mut draw_buffer, &mut z_pos, COLOR[i]);
      }

       
      for i in 0..HEIGHT{
         for j in 0..WIDTH {
            print!("{}",draw_buffer[i*WIDTH+j])
         }
         print!("\n")
      }
      

      print!("\n");
      print!("{}",forever);
      print!("\n");

      draw_buffer = [BACKGROUND;WIDTH*HEIGHT];
      z_pos = [(2.0*CUBE_SIDE as f64);WIDTH*HEIGHT];

      sleep(Duration::from_millis(20));

      forever += 1;
   }

   
     

   }