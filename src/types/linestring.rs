// Copyright 2014-2015 The GeoRust Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use types::coord::Coord;
use Geometry;


#[derive(Default)]
pub struct LineString(pub Vec<Coord>);

impl LineString {
    pub fn as_item(self) -> Geometry {
        Geometry::LineString(self)
    }
}


#[cfg(test)]
mod tests {
    use {Wkt, Geometry};
    use super::LineString;

    #[test]
    fn basic_linestring() {
        let mut wkt = Wkt::from_str("LINESTRING (10 -20, -0 -0.5)").ok().unwrap();
        assert_eq!(1, wkt.items.len());
        let coords = match wkt.items.pop().unwrap() {
            Geometry::LineString(LineString(coords)) => coords,
            _ => unreachable!(),
        };
        assert_eq!(2, coords.len());

        assert_eq!(10.0, coords[0].x);
        assert_eq!(-20.0, coords[0].y);
        assert_eq!(None, coords[0].z);
        assert_eq!(None, coords[0].m);

        assert_eq!(0.0, coords[1].x);
        assert_eq!(-0.5, coords[1].y);
        assert_eq!(None, coords[1].z);
        assert_eq!(None, coords[1].m);
    }
}
