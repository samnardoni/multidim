//! # Example
//! ```
//! let mut m = Matrix::from_elem(3,2, 300u);
//! println!("{}", m[(0,1)]);	
//! m[(0,1)] = 5;
//! println!("{}", m[(0,1)]);	
//! ```

use std::default::Default;
use std::iter::repeat;

pub struct Matrix<T> {
	elem: Vec<T>,
	rows: uint,
	cols: uint,
}

impl<T: Clone> Matrix<T> {
	pub fn from_elem(rows: uint, cols: uint, e: T) -> Matrix<T> {
		Matrix { 
			elem: repeat(e).take(rows*cols).collect(),
			rows: rows,
			cols: cols,
		}
	}
}

impl<T: Default + Clone> Matrix<T> {
	pub fn default(rows: uint, cols: uint) -> Matrix<T> {
		Matrix::from_elem(rows, cols, Default::default())
	}
}

impl<T> Matrix<T> {
	pub fn rows(&self) -> uint { self.rows }
	pub fn cols(&self) -> uint { self.cols }
}

impl<T> Index<(uint, uint), T> for Matrix<T> {
	fn index(&self, &(row, col): &(uint, uint)) -> &T {
		&self.elem[row*self.cols + col]
	}
}

impl<T> IndexMut<(uint, uint), T> for Matrix<T> {
	fn index_mut(&mut self, &(row, col): &(uint, uint)) -> &mut T {
		&mut self.elem[row*self.cols + col]
	}
}
