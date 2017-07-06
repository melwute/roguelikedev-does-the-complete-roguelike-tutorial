
extern crate rand;

use rand::Rng;

use std::cmp;

use game::rect::*;
use game::tile::*;




pub struct Map {
    tiles: Vec<Tile>,
    width:i32,
    height:i32,
}

//TODO Maybe one day we can implement an iterator over the map
//that will give the (x,y) coord of the tile and the tile itself
impl Map {
    //We use i32's for the map's width / height because
    //easier intergration with libtcod
    //less wonky math when dealing with negatives
    
    pub fn new(width:i32, height:i32, default_tile:Tile) -> Self {
        assert!(width > 0, "width must be greater than 0!");
        assert!(height > 0, "height must be greater than 0!");

        Map {
            tiles: vec![default_tile; (height * width) as usize],
            width:width,
            height:height,
        }
    }

    fn index_at(&self, x:i32, y:i32) -> usize {
        return (y * self.width() + x) as usize;

    }
    pub fn at(&self, x:i32, y:i32) -> &Tile {
        &self.tiles[self.index_at(x,y)]
    }

    pub fn set(&mut self, x:i32, y:i32, tile:Tile){
        let index = self.index_at(x,y);
        self.tiles[index] = tile;
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }


    fn create_room(&mut self, room: Rect, ) {
        for x in (room.x1 + 1) .. room.x2 {
            for y in (room.y1 + 1) .. room.y2 {
                self.set(x,y,Tile::empty());
            }
        }
    }

    fn create_v_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
            self.set(x,y, Tile::empty());
        }
    }
    fn create_h_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
            self.set(x,y, Tile::empty());
        }
    }


    pub fn create_random(width:i32, height:i32) -> (Self, (i32,i32)){
        const ROOM_MAX_SIZE: i32 = 10;
        const ROOM_MIN_SIZE: i32 = 6;
        const MAX_ROOMS: i32 = 40;

        //set everything to a wall first.
        let mut map = Map::new(width,height, Tile::wall());

        //our starting position will be in the first valid room's center.
        let mut starting_position = (0, 0);

        //Then "carve" the empty rooms out.
        let mut rooms = vec![];

        for _ in 0..MAX_ROOMS {
            // random width and height
            let w = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
            let h = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
            // random position without going out of the boundaries of the map
            let x = rand::thread_rng().gen_range(0, map.width() - w);
            let y = rand::thread_rng().gen_range(0, map.height() - h);
            let new_room = Rect::new(x, y, w, h);

            // run through the other rooms and see if they intersect with this one
            let failed = rooms.iter().any(|other_room| new_room.intersects_with(other_room));

            // this means there are no intersections, so this room is valid
            if !failed {
                // "carve" it to the map's tiles
                map.create_room(new_room);

                //TODO just for the hell of it make it so the player spawns randomly in the first room.
                let (new_x, new_y) = new_room.center();
                
                if rooms.is_empty() {
                    //First room since there isnt any other rooms
                    starting_position = (new_x, new_y);
                }else{
                    //Non first room. 
                    // all rooms after the first:
                    // connect it to the previous room with a tunnel
                    // center coordinates of the previous room
                    let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                    // draw a coin (random bool value -- either true or false)
                    if rand::random() {
                        // first move horizontally, then vertically
                        map.create_h_tunnel(prev_x, new_x, prev_y);
                        map.create_v_tunnel(prev_y, new_y, new_x);
                    } else {
                        // first move vertically, then horizontally
                        map.create_v_tunnel(prev_y, new_y, prev_x);
                        map.create_h_tunnel(prev_x, new_x, new_y);
                    }
                }

                rooms.push(new_room);
            }   
        }
        (map, starting_position)
    }

    

}