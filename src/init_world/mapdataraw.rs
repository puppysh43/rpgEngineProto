//this will be the home of the struct that reads JSON files from Tiled
//and turns it into gamedata for our engine to use!
use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Layer {
    //the example used i64 but I need to see if it'll work with i32
    //so I don't need to cast any variables after the fact
    data: Vec<i64>,
    height: i64,
    id: i64,
    name: String,
    opacity: f64,
    #[serder(rename = "type")]
    type_: String,
    visible: bool,
    width: i64,
    x: i64,
    y: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Tileset {
    firstgid: i64,
    source: String,
}
///This struct holds the raw json data after its been deserialized. you will be able to export it to
///the actual mapscreens as well as in the future hopefully to spawn in monsters along with their AI
///or other light scripting
#[derive(Serialize, Deserialize, Debug)]
struct MapDataRaw {
    compressionlevel: i64,
    height: i64,
    infinite: bool,
    layers: Vec<Layer>,
    nextlayerid: i64,
    nextobjectid: i64,
    orientation: String,
    renderorder: String,
    tiledversion: String,
    tileheight: i64,
    tilesets: Vec<Tileset>,
    tilewidth: i64,
    #[serde(rename = "type")]
    type_: String,
    version: String,
    width: i64,
}

impl MapDataRaw {
    fn export_mapscreen(&self) -> MapScreen {
        let first_layer = self.layers[0].clone().unwrap();
        let mapscreen = MapScreen::new();
        let mut index: usize = 0;
        if first_layer.height == MAP_HEIGHT && first_layer.width == MAP_WIDTH {
            // for tileID in first_layer.data {
            //will need to get the integer from the data vec and somehow convert it into a tiletype enum and then
            //add it into the mapscreen variable, with an index variable to keep track of how far we are into
            //the mapscreen.
            // }
        }
    }
}
