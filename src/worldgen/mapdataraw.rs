//this will be the home of the struct that reads JSON files from Tiled
//and turns it into gamedata for our engine to use!
use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Layer {
    //the example used i32 but I need to see if it'll work with i32
    //so I don't need to cast any variables after the fact
    data: Vec<i32>,
    height: i32,
    id: i32,
    name: String,
    opacity: f64,
    #[serde(rename = "type")]
    type_: String,
    visible: bool,
    width: i32,
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Tileset {
    firstgid: i32,
    source: String,
}
///This struct holds the raw json data after its been deserialized. you will be able to export it to
///the actual mapscreens as well as in the future hopefully to spawn in monsters along with their AI
///or other light scripting
#[derive(Serialize, Deserialize, Debug)]
pub struct MapDataRaw {
    compressionlevel: i32,
    height: i32,
    infinite: bool,
    layers: Vec<Layer>,
    nextlayerid: i32,
    nextobjectid: i32,
    orientation: String,
    renderorder: String,
    tiledversion: String,
    tileheight: i32,
    tilesets: Vec<Tileset>,
    tilewidth: i32,
    #[serde(rename = "type")]
    type_: String,
    version: String,
    width: i32,
}

impl MapDataRaw {
    ///This method produces
    pub fn export_mapscreen(&self) -> MapScreen {
        //for now we only need to get the first layer of the map
        let first_layer = self.layers[0].clone();
        //filler mapscreen to write over later
        let mut mapscreen = MapScreen::new();
        //we need to manually track and increment an index for this
        let mut index: usize = 0;
        //only process the tiled map if its height and width is the same as the engine's mapscreens
        if first_layer.height == MAP_HEIGHT && first_layer.width == MAP_WIDTH {
            //iterate through and process the data into whatever the corresponding tiletypes are
            for tile_id in first_layer.data {
                // println!("{}", tile_id);
                mapscreen.tiles[index] = int_to_tile(tile_id);
                index += 1;
            }
        }
        mapscreen
    }
}

fn int_to_tile(id: i32) -> TileType {
    match id {
        36 => TileType::Wall,
        47 => TileType::Floor,
        61 => TileType::StairUp,
        63 => TileType::StairDown,
        _ => TileType::Floor,
    }
}
