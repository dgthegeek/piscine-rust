// pub use mobs::*;
// pub use boss;
// pub mod mobs {
//     pub struct Mob {
//         pub name: String,
//         pub boss: boss::Boss,
//         pub members: Vec<member::Member>,
//         pub cities: Vec<(String, u8)>,
//         pub wealth: u32,
//     }
//     impl Mob {
//         pub fn recruit(&mut self, name: String, age: u8) {
//             let new_member = member::Member::new(name, member::Role::Associate);
//             self.members.push(new_member);
//         }
//     }
// }