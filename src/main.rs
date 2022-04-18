extern crate num;
use macroquad::prelude::*;

const DAMPBALL : f32 = 0.4;
const DAMPWALL : f32 = 0.9;

fn distance_between(x1:f32,y1:f32,x2:f32,y2:f32) -> f32{
    f32::sqrt(num::pow(x1-x2,2) + num::pow(y1-y2,2))
}


struct Ball{
    x:f32,
    y:f32,
    velx:f32,
    vely:f32,
    accelaration:f32,
    collided:bool,
}

impl Ball{
    fn wall_collison_x(&mut self) {
        let left = screen_width()/6.0+15.0;
        let right = screen_width()*5.0/6.0-15.0;
        if self.x<= left {self.velx =  self.velx.abs() * DAMPWALL}
        if self.x>=right {self.velx = - self.velx.abs() * DAMPWALL}
    }
    fn wall_collison_y(&mut self) {
        let top = screen_height()/4.0+15.0;
        let bottom = screen_height()*3.0/4.0-15.0;
        if self.y<= top {self.vely =  self.vely.abs() * DAMPWALL;}
        if self.y>=bottom {self.vely = - self.vely.abs() * DAMPWALL;}
    }
    fn collison_detection(&self,dice: &Dice) -> bool{
        if distance_between(self.x,self.y,dice.x,dice.y)<30.0 { return true; }
        false
    }
    pub fn update_vars(&mut self,dice: &Dice){
        if Ball::collison_detection(self,dice) && !self.collided {
            let theta = ((self.y - dice.y) / (self.x-dice.x)).atan();
            self.collided = true;
            self.accelaration = dice.accelaration;
            if dice.velx == 0.0 {
                self.velx = -self.velx * DAMPBALL;
                self.velx = self.velx * theta.cos() * theta.cos() + self.vely * theta.cos() * theta.sin();
            }
            else {self.velx = dice.velx * theta.cos() * theta.cos() + dice.vely * theta.cos() * theta.sin();}
            if dice.vely == 0.0 {
                self.vely = -self.vely * DAMPBALL;
                self.vely = self.vely * theta.sin() * theta.sin() + self.velx * theta.cos() * theta.sin();
            }
            else {self.vely = dice.vely * theta.sin() * theta.sin() + dice.velx * theta.cos() * theta.sin();}
        } else if !Ball::collison_detection(self,dice) {self.collided = false;}
        self.velx = self.velx.clamp(-1000.0,1000.0);
        self.vely = self.vely.clamp(-1000.0,1000.0);
        Ball::wall_collison_x(self);  
        Ball::wall_collison_y(self);  
        let time = get_frame_time();
        let distancex = self.velx * time;
        self.x = self.x + distancex;
        let distancey = self.vely * time;
        self.y = self.y + distancey;
    }
}

struct Dice{
    x:f32,
    y:f32,
    velx:f32,
    vely:f32,
    accelaration:f32,
}

impl Dice{
    pub fn fix_pos(num:f32,lower:f32,higher:f32) -> f32{
        let mid = (lower+higher)/2.0;
        if num>=lower && num<=mid {
            return lower;
        }

        if num>mid && num<=higher {return higher;}
        num
    }
    pub fn handle_collision(&self,ball: &Ball,x: f32,y: f32) -> (f32,f32) {
        if ball.collided {
            let y = self.y;
            let x = self.x;
            let distance = 30.0;
            let theta = ((y - ball.y) / (x-ball.x)).atan();
            let disx = (distance * theta.cos()).abs();
            let disy = (distance * theta.sin()).abs();
            let y = Dice::fix_pos(y,ball.y-disy,ball.y+disy);
            let x = Dice::fix_pos(x,ball.x-disx,ball.x+disx);
            return (x,y)
        }
        (x,y)
    }
    pub fn update_vars(&mut self,ball : &Ball){
        let time = get_frame_time();
        let (x,y) = mouse_position();
        let x = x.clamp(screen_width()/6.0+15.0,screen_width()/2.0-15.0);
        let y = y.clamp(screen_height()/4.0+15.0,screen_height()*3.0/4.0-15.0);
        let (x,y) = Dice::handle_collision(self,ball,x,y);
        if !ball.collided {
            self.velx = (x-self.x)/time;
            self.vely = (y- self.y)/time;
        }
        self.x = x;
        self.y = y;
    }
}

#[macroquad::main("demo game")]
async fn main() {
    let (x,y) = mouse_position();
    let mut dice = Dice{
        x:x,
        y:y,
        velx:0.0,
        vely:0.0,
        accelaration:0.0,
    };
    let mut ball = Ball{
        x:screen_width()/2.0,
        y:screen_height()/2.0,
        velx:0.0,
        vely:0.0,
        accelaration : 0.0,
        collided : false,
    };
    let mut temp = 1;
    loop {
        clear_background(RED);
        dice.update_vars(&ball);
        ball.update_vars(&dice);
        draw_rectangle_lines(screen_width()/6.0,screen_height()/4.0,screen_width()/1.5,screen_height()/2.0,5.0,YELLOW);
        draw_rectangle_lines(screen_width()/6.0-10.0,screen_height()/4.0-10.0,screen_width()/1.5+20.0,screen_height()/2.0+20.0,5.0,YELLOW);
        draw_line(screen_width()/2.0-1.25,screen_height()/4.0,screen_width()/2.0-1.25,screen_height()*3.0/4.0,2.5,YELLOW);
        draw_circle(dice.x,dice.y,15.0,YELLOW);
        draw_circle(ball.x,ball.y,15.0,GREEN);
        if temp == 60 {
            println!("{} {} {}",ball.x,ball.y,ball.velx);
            println!("{} {} {}",dice.x,dice.y,dice.velx);
            println!("{}",ball.collided);
            temp = 0;
        }
        temp+=1;
        next_frame().await
    }
}
