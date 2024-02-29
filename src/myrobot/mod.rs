use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::runner::{Robot, Runnable};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::World;

pub struct MyRobot{
    pub(crate) robot : Robot,
}
impl MyRobot{
    pub fn new()->Self{
        Self{
            robot: Robot::default(),
        }
    }
}

impl Runnable for MyRobot{
    fn process_tick(&mut self, world: &mut World) {
        stupid_actions(self, world)
    }
    fn handle_event(&mut self, event: Event) {
        println!("{}", event);
    }
    fn get_energy(&self) -> &Energy {
        &self.robot.energy
    }
    fn get_energy_mut(&mut self) -> &mut Energy {
        &mut self.robot.energy
    }
    fn get_coordinate(&self) -> &Coordinate {
        &self.robot.coordinate
    }
    fn get_coordinate_mut(&mut self) -> &mut Coordinate{
        &mut self.robot.coordinate
    }
    fn get_backpack(&self) -> &BackPack {
        &self.robot.backpack
    }
    fn get_backpack_mut(&mut self) -> &mut BackPack {
        &mut self.robot.backpack
    }
}

fn stupid_actions(myr: &mut MyRobot, world : &mut World){

}