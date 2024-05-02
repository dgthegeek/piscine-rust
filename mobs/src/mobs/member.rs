pub enum Role { 
    Underboss,
    Caporegime,
    Soldier,
    Associate,
}
pub struct Member {
    name: String,
    role: Role,
    age: u8,
}
impl Member {
    pub fn get_promotion(&mut self){
        match(self.role){
            Associate => self.role=Soldier,
            Soldier => self.role=Caporegime,
            Caporegime => self.role=Underboss
        }
    }
    pub fn new(name : String,role:Role,age:u8)->Member{
        Member{name,role,age}
    }
}