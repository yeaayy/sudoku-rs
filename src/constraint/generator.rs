use super::{GroupGenerator, NoDuplicateGenerator};

pub struct GGVertical {
  board_width: usize,
  board_height: usize,
  width: usize,
  height: usize,
  dx: usize,
  dy: usize,
}

impl GGVertical {
  pub fn new(nd: &NoDuplicateGenerator, width: usize, height: usize, dx: usize, dy: usize) -> GGVertical {
    GGVertical {
      board_width: nd.width,
      board_height: nd.height,
      width,
      height,
      dx,
      dy,
    }
  }
}

impl GroupGenerator for GGVertical {
  fn get_board(&self) -> (usize, usize) {
    (self.board_width, self.board_height)
  }

  fn get_size(&self) -> (usize, usize) {
    (self.width, self.height)
  }

  fn get_member(&self, group: usize, member: usize) -> usize {
    group + self.dx + (member + self.dy) * self.board_width
  }

}

pub struct GGHorizontal {
  board_width: usize,
  board_height: usize,
  width: usize,
  height: usize,
  dx: usize,
  dy: usize,
}

impl GGHorizontal {
  pub fn new(nd: &NoDuplicateGenerator, width: usize, height: usize, dx: usize, dy: usize) -> GGHorizontal {
    GGHorizontal {
      board_width: nd.width,
      board_height: nd.height,
      width,
      height,
      dx,
      dy,
    }
  }
}

impl GroupGenerator for GGHorizontal {
  fn get_board(&self) -> (usize, usize) {
    (self.board_width, self.board_height)
  }


  fn get_size(&self) -> (usize, usize) {
    (self.height, self.width)
  }

  fn get_member(&self, group: usize, member: usize) -> usize {
    member + self.dx + (group + self.dy) * self.board_width
  }
}

pub struct GGBlock {
  board_width: usize,
  board_height: usize,
  width: usize,
  height: usize,
  block_width: usize,
  block_height: usize,
  dx: usize,
  dy: usize,
}

impl GGBlock {
  pub fn new(nd: &NoDuplicateGenerator, width: usize, height: usize, block_width: usize, block_height: usize, dx: usize, dy: usize) -> GGBlock {
    GGBlock {
      board_width: nd.width,
      board_height: nd.height,
      width,
      height,
      block_width,
      block_height,
      dx,
      dy,
    }
  }
}

impl GroupGenerator for GGBlock {
  fn get_board(&self) -> (usize, usize) {
    (self.board_width, self.board_height)
  }

  fn get_size(&self) -> (usize, usize) {
    (self.width * self.height, self.block_width * self.block_height)
  }

  fn get_member(&self, group: usize, member: usize) -> usize {
    group % self.width * self.block_width + member % self.block_width + self.dx
    +(group / self.width * self.block_height + member / self.block_width + self.dy) * self.board_width
  }

}
