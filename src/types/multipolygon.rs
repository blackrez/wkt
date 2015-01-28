// Copyright 2015 The GeoRust Developers
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

use tokenizer::PeekableTokens;
use types::FromTokens;
use types::polygon::Polygon;
use WktItem;


#[derive(Default)]
pub struct MultiPolygon {
    pub polygons: Vec<Polygon>
}

impl MultiPolygon {
    pub fn as_item(self) -> WktItem {
        WktItem::MultiPolygon(self)
    }
}

impl FromTokens for MultiPolygon {
    fn from_tokens(tokens: &mut PeekableTokens) -> Result<Self, &'static str> {
        let result = FromTokens::comma_many(<Polygon as FromTokens>::from_tokens_with_parens, tokens);
        result.map(|vec| MultiPolygon {polygons: vec})
    }
}


#[cfg(test)]
mod tests {
    use {Wkt, WktItem};

    #[test]
    fn basic_multipolygon() {
        let mut wkt = Wkt::from_str("MULTIPOLYGON (((8 4)), ((4 0)))").ok().unwrap();
        assert_eq!(1, wkt.items.len());
        let multipolygon = match wkt.items.pop().unwrap() {
            WktItem::MultiPolygon(multipolygon) => multipolygon,
            _ => unreachable!(),
        };
        assert_eq!(2, multipolygon.polygons.len());
    }
}