use core::fmt::Display;
use std::num::ParseFloatError;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError<'a> {
    ParseFloatError(std::num::ParseFloatError),
    LineMalformed(&'a str),
}

impl From<ParseFloatError> for ParseError<'_> {
    fn from(value: ParseFloatError) -> Self {
        Self::ParseFloatError(value)
    }
}

impl Display for ParseError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LineMalformed(v) => write!(f, "Line is malformed: {v}"),
            Self::ParseFloatError(e) => write!(f, "Unable to parse into a float: {e}"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct ExtendedMatrix<const D: usize> {
	left: [[f64; D]; D],
	right: [f64; D],
}

impl<const D: usize> ExtendedMatrix<D> {
	
	/// Solves the given square matrix in place by Gaussian Elimination. Returns Some(()),
	/// if the matrix was solvable (i. e. its rows are linearly independent) and None
	/// otherwise. If it was solvable, its left part is transformed into reduced echolon form,
	/// and its right part represents the solution vector. No columns are swapped,
	/// but due to the transformation, the determinant will likely be changed.
	fn solve(&mut self) -> Option<()> {
		// Gaussian Elimination
        let dim = D;
		for row in 0..dim {
			let (r_max, val) = self.left.iter().enumerate().skip(row).map(|(idx, r)| (idx, r[row])).max_by_key(|(_idx, val)| val.abs() as usize).unwrap();
			if val.is_subnormal() {
				// This column is already 0 for all rows. This means, the rows aren't linearly independent,
				// so there is no unique solution.
				return None;
			} else {
				if row != r_max {
                    self.left.swap(row, r_max);
                    self.right.swap(row, r_max);
				}
				let pivot = self.left[row];
				self.left.iter_mut().enumerate().skip(row+1).for_each(|(r_idx, r)| {
					let factor = r[row]/pivot[row];
					r[row] = 0.0;
					r.iter_mut().enumerate().skip(row+1).for_each(|(c_idx, val)| *val -= pivot[c_idx]*factor);
					self.right[r_idx] -= self.right[row]*factor;
				});
			}
		}
		for row in (0..dim).rev() {
			let was = self.right[row];
			let sub: f64 = self.left[row].iter().enumerate().skip(row+1).map(|(col, val)| val*self.right[col]).sum();
			self.right[row] = (was-sub)/self.left[row][row];
            self.left[row].fill(0.0);
            self.left[row][row] = 1.0;
		}
		Some(())
	}
}

#[derive(Clone, Copy, Debug)]
struct Path {
    px: f64,
    py: f64,
    pz: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

impl<'a> TryFrom<&'a str> for Path {
    type Error = ParseError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let components: Vec<_> = value.split_whitespace().map(|c| c.split_once([',', '@']).unwrap_or((c, "")).0).collect();
        if components.len() != 7 {
            return Err(Self::Error::LineMalformed(value));
        }
        let px = components[0].parse::<f64>()?;
        let py = components[1].parse::<f64>()?;
        let pz = components[2].parse::<f64>()?;
        let vx = components[4].parse::<f64>()?;
        let vy = components[5].parse::<f64>()?;
        let vz = components[6].parse::<f64>()?;

        Ok(Path{ px, py, pz, vx, vy, vz, })
    }
}

impl Path {
    fn m_y(&self) -> f64 {
        self.vy/self.vx
    }

    fn n_y(&self) -> f64 {
        self.py - self.px * self.vy / self.vx
    }

    fn at(&self, t: f64) -> (f64, f64, f64) {
        (self.px+t*self.vx, self.py+t*self.vy, self.pz+t*self.vz)
    }

    fn horizontal_intersection(&self, other: &Self) -> (f64, f64, bool) {
        let x = (other.n_y() - self.n_y()) / (self.m_y() - other.m_y());
        let y = self.m_y() * x + self.n_y();
        let future = match (self.vx > 0.0, other.vx > 0.0) {
            (true, true) => x > self.px && x > other.px,
            (true, false) => x > self.px && x <= other.px,
            (false, true) => x <= self.px && x > other.px,
            (false, false) => x <= self.px && x <= other.px,
        };
        (x, y, future)
    }

}

pub fn run(input: &str, min: f64, max: f64) -> Result<(usize, usize), ParseError> {
    let paths: Vec<_> = input.lines().map(Path::try_from).collect::<Result<Vec<_>, _>>()?;
    let intersections: Vec<_> = paths.iter().enumerate().flat_map(|(idx, p1)| paths.iter().skip(idx+1).map(|p2| p1.horizontal_intersection(p2)).collect::<Vec<_>>()).collect();
    let first = intersections.iter().filter(|&(x, y, future)| *future && (min..=max).contains(x) && (min..=max).contains(y)).count();
    let stone_path = find_stone_path(&paths);
    let second = (stone_path.px + stone_path.py + stone_path.pz) as usize;
    Ok((first, second))
}

fn find_stone_path(paths: &[Path]) -> Path {
    // Since the rock r must hit any hail j at time t_j, we know that their x coordinates must be equal:
    // p_xj+v_xj*t_j = p_xr+v_xr*t_j
    //// p_xj-p_xr = t_j(v_xr-v_xj)
    // (p_xj-p_xr)/(v_xr-v_xj) = t_j
    //
    // Since the same holds true on the y axis, we can equate:
    // p_yj+v_yj(p_xj-p_xr)/(v_xr-v_xj) = p_yr+v_yr(p_xj-p_xr)/(v_xr-v_xj)
    // (v_xr-v_xj)p_yj + (p_xj-p_xr)v_yj - (v_xr-v_xj)p_yr - (p_xj-p_xr)v_yr = 0
    //
    // Now, this must be true for any hail k as well (since the equation no longer depends on t_j), so:
    // 0 = (v_xr-v_xk)p_yk + (p_xk-p_xr)v_yk - (v_xr-v_xk)p_yr - (p_xk-p_xr)v_yr
    // (v_xr-v_xj)p_yj + (p_xj-p_xr)v_yj - (v_xr-v_xj)p_yr - (p_xj-p_xr)v_yr = (v_xr-v_xk)p_yk + (p_xk-p_xr)v_yk - (v_xr-v_xk)p_yr - (p_xk-p_xr)v_yr
    //
    // Wich simplifies as
    //// v_yr(p_xk-p_xj) + p_yr(v_xj-v_xk) + v_xr(p_yj-p_yk) + p_xr(v_yk-v_yj) = p_xk*v_yk + p_yj*v_xj - p_yk*v_xk - p_xj*v_yj
    // v_xr(p_yj-p_yk) + v_yr(p_xk-p_xj) + p_xr(v_yk-v_yj) + p_yr(v_xj-v_xk) = p_xk*v_yk + p_yj*v_xj - p_yk*v_xk - p_xj*v_yj
    //
    // Now we have a linear equation with four unknowns (v_xr, v_yr, p_xr, and p_yr), which only depends on the
    // parameters of j and k. So we can pick any 4 (linearly independent) combination of 2 hails and solve.

    let (p_x0, p_y0, v_x0, v_y0) = {
        let hail = paths[0];
        (hail.px, hail.py, hail.vx, hail.vy)
    };
    let (p_x1, p_y1, v_x1, v_y1) = {
        let hail = paths[1];
        (hail.px, hail.py, hail.vx, hail.vy)
    };
    let (p_x4, p_y4, v_x4, v_y4) = {
        let hail = paths[2];
        (hail.px, hail.py, hail.vx, hail.vy)
    };
    let (p_x2, p_y2, v_x2, v_y2) = {
        let hail = paths[3];
        (hail.px, hail.py, hail.vx, hail.vy)
    };
    let (p_x3, p_y3, v_x3, v_y3) = {
        let hail = paths[4];
        (hail.px, hail.py, hail.vx, hail.vy)
    };

    let mut matrix = ExtendedMatrix{
        left: [
            [p_y0-p_y1, p_x1-p_x0, v_y1-v_y0, v_x0-v_x1],
            [p_y0-p_y2, p_x2-p_x0, v_y2-v_y0, v_x0-v_x2],
            [p_y0-p_y3, p_x3-p_x0, v_y3-v_y0, v_x0-v_x3],
            [p_y0-p_y4, p_x4-p_x0, v_y4-v_y0, v_x0-v_x4],
        ],
        right: [
            p_x1*v_y1 + p_y0*v_x0 - p_y1*v_x1 - p_x0*v_y0,
            p_x2*v_y2 + p_y0*v_x0 - p_y2*v_x2 - p_x0*v_y0,
            p_x3*v_y3 + p_y0*v_x0 - p_y3*v_x3 - p_x0*v_y0,
            p_x4*v_y4 + p_y0*v_x0 - p_y4*v_x4 - p_x0*v_y0,
        ],
    };
    if matrix.solve().is_none() {
        panic!("Hails aren't linearly independent");
    }
    let (v_xr, v_yr, p_xr, p_yr) = (matrix.right[0].round(), matrix.right[1].round(), matrix.right[2].round(), matrix.right[3].round());

    let t_0 = (p_x0-p_xr)/(v_xr-v_x0);
    let t_1 = (p_x1-p_xr)/(v_xr-v_x1);
    let z_0 = paths[0].at(t_0).2;
    let z_1 = paths[1].at(t_1).2;

    let v_zr = (z_1-z_0)/(t_1-t_0);
    let p_zr = z_0 - t_0*v_zr;

    Path{ px: p_xr, py: p_yr, pz: p_zr, vx: v_xr, vy: v_yr, vz: v_zr }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn read_file(name: &str) -> String {
        read_to_string(name).expect(&format!("Unable to read file: {name}")[..]).trim().to_string()
    }

    #[test]
    fn matrix_solver() {
        let mut m = ExtendedMatrix{ left: [
            [1.0, 3.0, -2.0, 4.0],
            [3.0, 5.0, 6.0, 0.0],
            [2.0, 4.0, 3.0, 2.0],
            [1.0, 2.0, -2.0, 2.0],
        ], right: [5.0, 7.0, 8.0, 1.0], };
        let expected = ExtendedMatrix{ left: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ], right: [5.0, -4.0, 2.0, 4.0], };
        assert!(m.solve().is_some());
        assert_eq!(m.left, expected.left);
        expected.right.iter().enumerate().for_each(|(idx, e)| assert!((e - m.right[idx]).abs() < 0.0001));
    }

    #[test]
    fn test_sample() {
        let sample_input = read_file("tests/sample_input");
        assert_eq!(run(&sample_input, 7.0, 27.0), Ok((2, 47)));
    }

    #[test]
    fn test_challenge() {
        let challenge_input = read_file("tests/challenge_input");
        assert_eq!(run(&challenge_input, 200_000_000_000_000.0, 400_000_000_000_000.0), Ok((17906, 571093786416929)));
    }
}
