#[derive(Debug, Clone, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: boss::Boss,
    pub members: Vec<member::Member>,
    pub cities: Vec<(String, u8)>,
    pub wealth: u32,
}
impl Mob {
    pub fn recruit(&mut self, name: &str, age: u8) {
        let new_member = member::Member::new(name, member::Role::Associate, age);
        self.members.push(new_member);
    }
    pub fn attack(&mut self, defense: &mut Mob) {
        let mut power_attack = 0;
        let mut power_defense = 0;
        for member in self.members.iter() {
            match member.role {
                member::Role::Underboss => power_attack += 4,
                member::Role::Caporegime => power_attack += 3,
                member::Role::Soldier => power_attack += 2,
                member::Role::Associate => power_attack += 1,
            }
        }
        for member in defense.members.iter() {
            match member.role {
                member::Role::Underboss => power_defense += 4,
                member::Role::Caporegime => power_defense += 3,
                member::Role::Soldier => power_defense += 2,
                member::Role::Associate => power_defense += 1,
            }
        }
        let result = power_attack - power_defense;
        if result > 0 {
            defense.members.pop();
            if defense.members.len() == 0 {
                self.cities.extend(defense.cities.clone());
                self.wealth += defense.wealth.clone();
                defense.cities.clear();
                defense.wealth=0;
            }
        } else {
            self.members.pop();
            if self.members.len() == 0 {
                defense.cities.extend(self.cities.clone());
                defense.wealth += self.wealth.clone();
               self.cities.clear();
               self.wealth=0;
            }
        }
    }
    pub fn steal(&mut self, cible :&mut Mob , value :u32){
        let mut stolen = value;
        if stolen > cible.wealth{
            stolen=cible.wealth;
        }
        self.wealth+=stolen;
        cible.wealth-=stolen;
    }
    pub fn conquer_city(&mut self, mobs : Vec<Mob>, name : String, value :u8){
        let mut is_in = false;
        for mob in mobs.iter(){
            for city in mob.cities.iter(){
                if city.0 == name{
                    is_in=true;
                    break;
                }
            }
            if is_in{break;}
        }
        if !is_in{
            self.cities.push((name,value))
        }
    }
}
pub mod boss {
    #[derive(Debug, Clone, PartialEq)]
    pub struct Boss {
        pub name: String,
        pub age: u8,
    }
    impl Boss {
        pub fn new(name: &str, age: u8) -> Boss {
            Boss {
                name: name.to_string(),
                age,
            }
        }
    }
}
pub mod member {
    #[derive(Debug, Clone, PartialEq)]
    pub enum Role {
        Underboss,
        Caporegime,
        Soldier,
        Associate,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct Member {
        pub name: String,
        pub role: Role,
        pub age: u8,
    }
    impl Member {
        pub fn get_promotion(&mut self) {
            match self.role {
                Role::Associate => self.role = Role::Soldier,
                Role::Soldier => self.role = Role::Caporegime,
                Role::Caporegime => self.role = Role::Underboss,
                Role::Underboss => self.role = Role::Underboss,
            }
        }
        pub fn new(name: &str, role: Role, age: u8) -> Member {
            Member {
                name: name.to_string(),
                role,
                age,
            }
        }
    }
}