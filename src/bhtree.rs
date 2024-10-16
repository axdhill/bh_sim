// Following guidance of
// https://www.cs.princeton.edu/courses/archive/fall03/cs126/assignments/barnes-hut.html
use macroquad::prelude::*;

#[allow(dead_code)]
#[derive(Debug)]
struct Quad {
    pos: Vec2,
    size: f32,
}

#[allow(dead_code)]
impl Quad {
    pub fn length(&self) -> f32 {
        return self.size;
    }
    pub fn contains(&self, x: Vec2) -> bool {
        let disp = self.pos - x;
        let len = self.size;
        return (disp.x >= 0.0) & (disp.x < len) & (disp.y >= 0.0) & (disp.y < len);
    }

    pub fn nw(&self) -> Quad {
        let new_size = self.size / 2.0;
        return Quad {
            size: new_size,
            pos: vec2(self.pos.x, self.pos.y + new_size),
        };
    }

    pub fn ne(&self) -> Quad {
        let new_size = self.size / 2.0;
        return Quad {
            size: new_size,
            pos: vec2(self.pos.x + new_size, self.pos.y + new_size),
        };
    }
    pub fn se(&self) -> Quad {
        let new_size = self.size / 2.0;
        return Quad {
            size: new_size,
            pos: vec2(self.pos.x, self.pos.y),
        };
    }
    pub fn sw(&self) -> Quad {
        let new_size = self.size / 2.0;
        return Quad {
            size: new_size,
            pos: vec2(self.pos.x + new_size, self.pos.y),
        };
    }
}
#[derive(Debug)]
#[allow(dead_code)]
struct Body {
    pos: Vec2,
    mass: f32,
}

#[allow(dead_code)]
impl Body {
    pub fn inside(&self, q: &Quad) -> bool {
        return q.contains(self.pos);
    }
}

#[allow(dead_code)]
fn sum_bodies(a: Body, b: Body) -> Body {
    return Body {
        mass: a.mass + b.mass,
        pos: a.pos * a.mass + b.pos * b.mass,
    };
}

#[derive(Debug)]
#[allow(dead_code)]
struct BHTree {
    body: Option<Box<Body>>,
    quad: Quad,
    nw: Option<Box<BHTree>>,
    ne: Option<Box<BHTree>>,
    sw: Option<Box<BHTree>>,
    se: Option<Box<BHTree>>,
}

#[allow(dead_code)]
impl BHTree {
    pub fn new(q: Quad) -> BHTree {
        BHTree {
            quad: q,
            body: None,
            nw: None,
            ne: None,
            sw: None,
            se: None,
        }
    }
    pub fn insert(&mut self, b: Body) {
        self.body = Some(Box::new(b));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bhtree() {
        let quad = Quad {
            pos: vec2(0.0, 0.0),
            size: 10.0,
        };
        let _bhtree = BHTree::new(quad);
    }
}
