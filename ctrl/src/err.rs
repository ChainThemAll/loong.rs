use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoongCtrlErr {
  #[error("row index ({0}) is out of bounds")]
  RowIndexOutOfBounds(u16),
  #[error("column index ({0}) is out of bounds")]
  ColumnIndexOutOfBounds(u16),
  #[error("the loong ate itself")]
  LoongAteItself,
  #[error("the loong hit the wall")]
  LoongHitTheWall,
  #[error("initial loong size is more than possible")]
  InitLoongSizeIsBig,
  #[error("something is really wrong. your loong size is zero")]
  LoongIsZero,
}

pub type LoongCtrlResult<T> = Result<T, LoongCtrlErr>;
