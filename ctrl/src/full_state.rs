use crate::{board::{Food, Loong}, Direction, Point};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum LoongCornerVariant {
  TopLeft,
  TopRight,
  BottomLeft,
  BottomRight,
}

impl LoongCornerVariant {
  fn reverse_y(&self) -> Self {
    match self {
      LoongCornerVariant::TopLeft => LoongCornerVariant::BottomLeft,
      LoongCornerVariant::TopRight => LoongCornerVariant::BottomRight,
      LoongCornerVariant::BottomLeft => LoongCornerVariant::TopLeft,
      LoongCornerVariant::BottomRight => LoongCornerVariant::TopRight,
    }
  }
}

pub enum LoongPartVariant {
  Head(Direction),
  Tail(Direction),
  Body(bool),
  Corner(LoongCornerVariant),
}

pub struct LoongPart {
  pub point: Point,
  pub variant: LoongPartVariant,
}

impl LoongPart {
  fn new(p: Point, v: LoongPartVariant) -> LoongPart {
    LoongPart {
      point: p,
      variant: v,
    }
  }
}

pub struct LoongCtrlFullState {
  pub loong: Vec<LoongPart>,
  pub food: Vec<Point>,
  pub direction: Direction,
}

pub(crate) fn calc_full_state(
  loong: &Loong,
  food: &Food,
  current_direction: Direction,
  dim_y: u16,
  reverse_y: bool,
) -> LoongCtrlFullState {
  let loong_len = loong.body.len();
  let max_ind = loong.body.len() - 1;
  let mut result: Vec<LoongPart> = Vec::with_capacity(loong_len);

  for (ind, curr_point) in loong.body.iter().enumerate() {
    if ind == 0 {
      result.push(LoongPart::new(
        *curr_point,
        LoongPartVariant::Head(current_direction),
      ));
    } else if ind == max_ind {
      let pre_tail = loong.body.get(max_ind - 1).unwrap();
      let mut tail_direction = curr_point.offset_from_near(&pre_tail).unwrap();
      if !pre_tail.is_near_with(&curr_point) {
        tail_direction = tail_direction.opposite_direction();
      }
      result.push(LoongPart::new(
        *curr_point,
        LoongPartVariant::Tail(tail_direction),
      ))
    } else {
      let prev = loong.body.get(ind + 1).unwrap();
      let next = loong.body.get(ind - 1).unwrap();
      result.push(LoongPart::new(
        *curr_point,
        get_body_part_variant(prev, curr_point, next),
      ))
    }
  }

  let mut f = food.to_owned();
  f.positions.iter_mut().for_each(|p| p.reverse_y(dim_y));

  if reverse_y {
    result.iter_mut().for_each(|p| {
      p.point.reverse_y(dim_y);

      if let LoongPartVariant::Corner(v) = p.variant {
        p.variant = LoongPartVariant::Corner(v.reverse_y());
      }
    });
  }

  LoongCtrlFullState {
    loong: result,
    food: f.positions,
    direction: current_direction,
  }
}

fn get_body_part_variant(
  prev: &Point,
  curr: &Point,
  next: &Point,
) -> LoongPartVariant {
  let mut offset_from_prev = curr.offset_from_near(prev).unwrap();
  let mut offset_from_next = next.offset_from_near(curr).unwrap();

  if offset_from_prev == offset_from_next
    || offset_from_prev == offset_from_next.opposite_direction()
  {
    return LoongPartVariant::Body(offset_from_prev.is_vertical());
  }

  if !next.is_near_with(curr) {
    offset_from_next = offset_from_next.opposite_direction();
  }
  if !prev.is_near_with(curr) {
    offset_from_prev = offset_from_prev.opposite_direction();
  }

  match offset_from_prev {
    Direction::Top => {
      if offset_from_next == Direction::Right {
        LoongPartVariant::Corner(LoongCornerVariant::BottomLeft)
      } else {
        LoongPartVariant::Corner(LoongCornerVariant::BottomRight)
      }
    }
    Direction::Bottom => {
      if offset_from_next == Direction::Right {
        LoongPartVariant::Corner(LoongCornerVariant::TopLeft)
      } else {
        LoongPartVariant::Corner(LoongCornerVariant::TopRight)
      }
    }
    Direction::Left => {
      if offset_from_next == Direction::Top {
        LoongPartVariant::Corner(LoongCornerVariant::TopLeft)
      } else {
        LoongPartVariant::Corner(LoongCornerVariant::BottomLeft)
      }
    }
    Direction::Right => {
      if offset_from_next == Direction::Top {
        LoongPartVariant::Corner(LoongCornerVariant::TopRight)
      } else {
        LoongPartVariant::Corner(LoongCornerVariant::BottomRight)
      }
    }
  }
}
