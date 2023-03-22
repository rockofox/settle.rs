use std::sync::Arc;

use hecs::World;
use hexes::hex::{create_hex, DiceValue, HexCoords};
use resources::resource::Resource;

mod hexes;
mod resources;

fn main() {
    let mut world = World::new();
    let wood = Arc::from(Resource {
        name: "Wood".into(),
    });
    world.spawn(create_hex((0, 0), 2, wood.clone()));
    world.spawn(create_hex((1, 0), 3, wood.clone()));
    world.spawn(create_hex((0, 1), 4, wood.clone()));
    world.spawn(create_hex((1, 1), 2, wood));

    let rolled_value = 2;
    println!("Rolled a {}", rolled_value);
    for (_id, (pos, value, resource)) in world.query::<(&HexCoords, &DiceValue, &Arc<Resource>)>().iter()
    {
        if *value == rolled_value {
            println!("Got {} from {:?}", resource.name, pos);
        }
    }
}
