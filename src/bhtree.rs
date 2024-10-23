// Following guidance of
// https://www.cs.princeton.edu/courses/archive/fall03/cs126/assignments/barnes-hut.html
use macroquad::prelude::*;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
struct Quad {
    pos: Vec2,
    size: f32,
}

#[allow(dead_code)]
impl Quad {
    pub fn length(&self) -> f32 {
        self.size
    }
    pub fn contains(&self, x: &Vec2) -> bool {
        let disp = self.pos - x;
        let len = self.size;
        (disp.x >= 0.0) & (disp.x < len) & (disp.y >= 0.0) & (disp.y < len)
    }

    pub fn nw(&self) -> Quad {
        let new_size = self.size / 2.0;
        Quad {
            size: new_size,
            pos: vec2(self.pos.x, self.pos.y + new_size),
        }
    }

    pub fn ne(&self) -> Quad {
        let new_size = self.size / 2.0;
        Quad {
            size: new_size,
            pos: vec2(self.pos.x + new_size, self.pos.y + new_size),
        }
    }
    pub fn se(&self) -> Quad {
        let new_size = self.size / 2.0;
        Quad {
            size: new_size,
            pos: vec2(self.pos.x, self.pos.y),
        }
    }
    pub fn sw(&self) -> Quad {
        let new_size = self.size / 2.0;
        Quad {
            size: new_size,
            pos: vec2(self.pos.x + new_size, self.pos.y),
        }
    }

    pub fn dispatch_into(&self, x: &Vec2) -> Quad {
        if self.se().contains(x) {
            return self.se();
        } else if self.sw().contains(x) {
            return self.sw();
        } else if self.nw().contains(x) {
            return self.nw();
        } else {
            return self.ne();
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
struct Body {
    pos: Vec2,
    mass: f32,
}

#[allow(dead_code)]
impl Body {
    pub fn inside(&self, q: &Quad) -> bool {
        q.contains(&self.pos)
    }
}

fn sum_bodies(a: &Body, b: &Body) -> Body {
    let total_mass = a.mass + b.mass;
    Body {
        mass: total_mass,
        pos: (a.pos * a.mass + b.pos * b.mass) / total_mass,
    }
}

#[derive(Debug, PartialEq)]
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
            quad: q.clone(),
            body: None,
            nw: None,
            ne: None,
            sw: None,
            se: None,
        }
    }

    fn find_subtree(&self, x: &Vec2) -> Option<Box<BHTree>> {
        let this_quad = self.quad;
        if self.quad.ne().contains(x) {
            return Some(Box::new(1.0));
        } else {
            return None;
        }
    }

    pub fn is_external(&self) -> bool {
        return (self.nw == None) & (self.ne == None) & (self.sw == None) & (self.se == None);
    }

    pub fn is_internal(&self) -> bool {
        return !self.is_external();
    }

    pub fn insert(&mut self, b: Body) {
        match &self.body {
            Some(body) => {
                self.body = Some(Box::new(sum_bodies(&body, &b)));
                let mut tree = self.find_subtree(&b.pos);
                tree.insert(b);
            }
            None => self.body = Some(Box::new(b)),
        }
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

    #[test]
    fn test_sum_bodies() {
        let a = Body {
            pos: vec2(0.5, 0.5),
            mass: 1.0,
        };
        let b = Body {
            pos: vec2(-0.5, -0.5),
            mass: 1.0,
        };

        let summed_body = sum_bodies(&a, &b);
        assert_eq!(
            summed_body,
            Body {
                pos: vec2(0.0, 0.0),
                mass: 2.0
            }
        );
    }

    #[test]
    fn test_sum_bodies_2() {
        let a = Body {
            pos: vec2(0.5, 0.5),
            mass: 1.0,
        };
        let b = Body {
            pos: vec2(-0.5, 0.5),
            mass: 1.0,
        };
        let c = Body {
            pos: vec2(-0.5, -0.5),
            mass: 1.0,
        };
        let d = Body {
            pos: vec2(0.5, -0.5),
            mass: 1.0,
        };

        let mut summed_body = sum_bodies(&a, &b);
        summed_body = sum_bodies(&summed_body, &c);
        summed_body = sum_bodies(&summed_body, &d);

        assert_eq!(
            summed_body,
            Body {
                pos: vec2(0.0, 0.0),
                mass: 4.0
            }
        );
    }

    #[test]
    fn test_basic_quad() {
        // Test the case where there is a BHTree with four bodies
        // one in each quadrant

        let body_ne = Body {
            pos: vec2(0.5, 0.5),
            mass: 1.0,
        };
        let body_nw = Body {
            pos: vec2(-0.5, 0.5),
            mass: 1.0,
        };
        let body_sw = Body {
            pos: vec2(-0.5, -0.5),
            mass: 1.0,
        };
        let body_se = Body {
            pos: vec2(0.5, -0.5),
            mass: 1.0,
        };

        let quad = Quad {
            pos: vec2(0.0, 0.0),
            size: 1.0,
        };
        let mut bhtree = BHTree::new(quad);

        bhtree.insert(body_ne.clone());
        bhtree.insert(body_nw.clone());
        bhtree.insert(body_sw.clone());
        bhtree.insert(body_se.clone());

        println!("{:?}\n", bhtree);

        // Set up target tree
        let spanning_quad = Quad {
            pos: vec2(-0.5, -0.5),
            size: 1.0,
        };
        let mut sample_bh_tree = BHTree::new(spanning_quad);
        sample_bh_tree.body = Some(Box::new(Body {
            pos: vec2(0.0, 0.0),
            mass: 4.0,
        }));

        let sw_quad = BHTree {
            quad: Quad {
                pos: vec2(-0.5, -0.5),
                size: 0.5,
            },
            body: Some(Box::new(body_sw)),
            ne: None,
            se: None,
            sw: None,
            nw: None,
        };

        let nw_quad = BHTree {
            quad: Quad {
                pos: vec2(-0.5, 0.0),
                size: 0.5,
            },
            body: Some(Box::new(body_nw)),
            ne: None,
            se: None,
            sw: None,
            nw: None,
        };
        let se_quad = BHTree {
            quad: Quad {
                pos: vec2(0.0, -0.5),
                size: 0.5,
            },
            body: Some(Box::new(body_se)),
            ne: None,
            se: None,
            sw: None,
            nw: None,
        };
        let ne_quad = BHTree {
            quad: Quad {
                pos: vec2(0.0, 0.0),
                size: 0.5,
            },
            body: Some(Box::new(body_ne)),
            ne: None,
            se: None,
            sw: None,
            nw: None,
        };

        sample_bh_tree.ne = Some(Box::new(ne_quad));
        sample_bh_tree.nw = Some(Box::new(nw_quad));
        sample_bh_tree.sw = Some(Box::new(sw_quad));
        sample_bh_tree.se = Some(Box::new(se_quad));

        println!("{:?}", sample_bh_tree);

        assert_eq!(bhtree, sample_bh_tree);
    }
}
