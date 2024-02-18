use crate::err::{LoongCtrlErr, LoongCtrlResult};
use crate::matrix::Matrix;
use crate::options::InnerCfg;
// use crate::utils::simple_rand;
use crate::{Direction, Point};
use rand::Rng;
use std::rc::Rc;
#[derive(Debug,Clone)]
pub struct Food {pub  positions:  Vec<Point>}

impl Food {
  fn generate(cfg: &InnerCfg, loong: &Loong, food: &Food) -> Point {
    let max_x = cfg.dimension_x;
    let max_y = cfg.dimension_y;

    if (max_x * max_y - 1) <= (loong.body.len() + food.positions.len()) as u16 {}

    let mut rng = rand::rngs::ThreadRng::default();
    let mut apple = loong.body[0];

    let occupied_points = [loong.body.clone(), food.positions.clone()].concat();

    while occupied_points.iter().any(|p| p == &apple) {
      let x = rng.gen_range(0, max_x);
      let y = rng.gen_range(0, max_y);
      apple = Point(x, y);
    }
   
   apple
  }

  fn clear_eaten(food: &mut Food, eaten: &Point) {
    if let Some(pos) = food.positions.iter().position(|p| p == eaten) {
      food.positions.remove(pos);
    }
  }

  fn new()->Self{
Food{positions:Vec::with_capacity(1)}
  }
  
}

#[derive(Debug,Clone)]
pub struct Loong {
  pub body: Vec<Point>,
}

impl Loong {
  
  pub fn new(center: Point, size: u16) -> LoongCtrlResult<Self> {
    let Point(center_x, center_y) = center;

    if center_x < size {
      return Err(LoongCtrlErr::InitLoongSizeIsBig);
    }

    let mut loong = Loong{body:Vec::with_capacity(usize::from(size))};
    for loong_part_ind in 0..size {
      loong.body.push(Point(center_x - loong_part_ind, center_y));
    }

    Ok(loong)
}
  fn create(center: Point, size: u16) -> LoongCtrlResult<Vec<Point>> {
    let Point(center_x, center_y) = center;

    if center_x < size {
      return Err(LoongCtrlErr::InitLoongSizeIsBig);
    }

    let mut loong = Vec::with_capacity(usize::from(size));
    for loong_part_ind in 0..size {
      loong.push(Point(center_x - loong_part_ind, center_y));
    }

    Ok(loong)
  }

  fn move_loong(
    cfg: &InnerCfg,
    loong: &mut Loong,
    direction: Direction,
  ) -> LoongCtrlResult<Point> {
    let last = if let Some(l) = loong.body.pop() {
      l
    } else {
      return Err(LoongCtrlErr::LoongIsZero);
    };

    let head = if let Some(f) = loong.body.first() {
      f
    } else {
      return Err(LoongCtrlErr::LoongIsZero);
    };

    let head_x = head.0;
    let head_y = head.1;

    let new_head = match direction {
      Direction::Right => (head_x as i32 + 1, head_y as i32),
      Direction::Top => (head_x as i32, head_y as i32 + 1),
      Direction::Bottom => (head_x as i32, head_y as i32 - 1),
      Direction::Left => (head_x as i32 - 1, head_y as i32),
    };

    let new_head = Loong::try_teleport_head_if_need(cfg, new_head)?;

    loong.body.insert(0, new_head);

    if Loong::is_ate_itself(loong) {
      return Err(LoongCtrlErr::LoongAteItself);
    }

    Ok(last)
  }

  fn is_ate_itself(loong: &mut Loong) -> bool {
    let head = &loong.body[0];
    for i in 1..loong.body.len() {
      let loong_part = &loong.body[i];
      if loong_part == head {
        return true;
      }
    }
    false
  }

  fn has_eaten(loong: Loong, food: &Food) -> Option<Point> {
    let first = &loong.body[0];
    for f in food.positions.iter() {
      if f == first {
        return Some(*f);
      }
    }
    None
  }

  fn try_teleport_head_if_need(
    cfg: &InnerCfg,
    new_head_unnormalized: (i32, i32),
  ) -> LoongCtrlResult<Point> {
    let (new_head_x, new_head_y) = new_head_unnormalized;
    let can_teleport = cfg.walking_through_the_walls;
    let err =
      || -> LoongCtrlResult<Point> { Err(LoongCtrlErr::LoongHitTheWall) };
    if new_head_unnormalized.0 < 0 {
      if !can_teleport {
        return err();
      }
      return Ok(Point(cfg.dimension_x - 1, new_head_y as u16));
    } else if new_head_y < 0 {
      if !can_teleport {
        return err();
      }
      return Ok(Point(new_head_x as u16, cfg.dimension_y - 1));
    } else if new_head_x > (cfg.dimension_x as i32 - 1) {
      if !can_teleport {
        return err();
      }
      return Ok(Point(0, new_head_y as u16));
    } else if new_head_unnormalized.1 > (cfg.dimension_y as i32 - 1) {
      if !can_teleport {
        return err();
      }
      return Ok(Point(new_head_x as u16, 0));
    }

    Ok(Point(new_head_x as u16, new_head_y as u16))
  }
}

pub(crate) struct Board {
  cfg: Rc<InnerCfg>,
  dim_x: u16,
  dim_y: u16,

  pub(crate) loong: Loong,
  pub(crate) food: Food,
}
impl Board {
  pub(crate) fn new(cfg: Rc<InnerCfg>) -> LoongCtrlResult<Self> {
    let InnerCfg {
      dimension_x,
      dimension_y,
      initial_loong_size,
      ..
    } = *cfg;

    let center = Board::center_of(dimension_x, dimension_y);
    let loong = Loong::new(center, initial_loong_size)?;

    let mut board = Board {
      cfg,
      dim_x: dimension_x,
      dim_y: dimension_y,
      loong,
      food:Food::new() ,
    };

    if board.cfg.auto_gen_food {
      board.generate_food();
    }

    Ok(board)
  }

  pub fn restart(&mut self) -> LoongCtrlResult<()> {
    self.loong = Loong::new(
      Board::center_of(self.cfg.dimension_x, self.cfg.dimension_y),
      self.cfg.initial_loong_size,
    )?;
    self.food = Food::new();
    if self.cfg.auto_gen_food {
      self.generate_food();
    }
    Ok(())
  }

  pub(crate) fn move_loong(
    &mut self,
    direction: Direction,
  ) -> LoongCtrlResult<bool> {
    let removed_last =
      Loong::move_loong(&self.cfg, &mut self.loong, direction)?;

    let eaten = Loong::has_eaten(self.loong.clone(), &self.food.clone());

    if let Some(e) = eaten {
      self.loong.body.push(removed_last);
      Food::clear_eaten(&mut self.food, &e);

      if self.cfg.auto_gen_food {
        self.generate_food();
      }
      return Ok(true);
    }
    return Ok(false);
  }

  pub(crate) fn generate_food(&mut self) {
    self
      .food.positions
      .push(
        Food::generate(&self.cfg, &self.loong, &self.food
        ));
  }

  fn center_of(dim_x: u16, dim_y: u16) -> Point {
    let center_x = dim_x / 2;
    let center_y = dim_y / 2;
    Point(center_x, center_y)
  }

  pub(crate) fn clone_loong(&self) -> Loong {
    Loong{body: self.loong.body.clone()}
  }
  pub(crate) fn clone_food(&self) -> Vec<Point> {
    self.food.positions.clone()
  }

  pub(crate) fn get_matrix(&self) -> Matrix {
    let mut m = Matrix::new(self.dim_x, self.dim_y);
    m.add_loong(&self.loong.body);
    m.add_food(&self.food);
    m
  }
}

 