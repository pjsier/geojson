use crate::json::JsonValue;
use crate::{util, Error};

/// Positions
///
/// [GeoJSON Format Specification § 3.1.1](https://tools.ietf.org/html/rfc7946#section-3.1.1)
pub trait Position: Sized + serde::Serialize {
    fn from_json_value(json: &JsonValue) -> Result<Self, Error>;
    fn from_x_y(x: f64, y: f64) -> Self;
    fn x(&self) -> f64;
    fn y(&self) -> f64;
}
// TODO: should this derivce serialize unconditionally?

impl Position for Vec<f64> {
    fn from_json_value(json: &JsonValue) -> Result<Self, Error> {
        let coords_array = util::expect_array(json)?;
        let mut coords = Vec::with_capacity(coords_array.len());
        for position in coords_array {
            coords.push(util::expect_f64(position)?);
        }
        Ok(coords)
    }

    fn from_x_y(x: f64, y: f64) -> Self {
        vec![x, y]
    }

    fn x(&self) -> f64 {
        self[0]
    }

    fn y(&self) -> f64 {
        self[1]
    }
}

impl Position for (f64, f64) {
    fn from_json_value(json: &JsonValue) -> Result<Self, Error> {
        let coords_array = util::expect_array(json)?;
        if coords_array.len() != 2 {
            unimplemented!()
        }
        Ok((
            util::expect_f64(&coords_array[0])?,
            util::expect_f64(&coords_array[1])?,
        ))
    }

    fn from_x_y(x: f64, y: f64) -> Self {
        (x, y)
    }

    fn x(&self) -> f64 {
        self.0
    }

    fn y(&self) -> f64 {
        self.1
    }
}

impl Position for (f64, f64, f64) {
    fn from_json_value(json: &JsonValue) -> Result<Self, Error> {
        let coords_array = util::expect_array(json)?;
        if coords_array.len() != 3 {
            unimplemented!()
        }
        Ok((
            util::expect_f64(&coords_array[0])?,
            util::expect_f64(&coords_array[1])?,
            util::expect_f64(&coords_array[2])?,
        ))
    }

    fn from_x_y(_x: f64, _y: f64) -> Self {
        unimplemented!()
    }

    fn x(&self) -> f64 {
        self.0
    }

    fn y(&self) -> f64 {
        self.1
    }
}