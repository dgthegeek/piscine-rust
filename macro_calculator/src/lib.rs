use json;
pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub proteins: f64,
    pub fats: f64,
    pub carbs: f64,
    pub nbr_of_portions: f64,
}
pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {
    let mut total_cals = 0.0;
    let mut total_carbs = 0.0;
    let mut total_proteins = 0.0;
    let mut total_fats = 0.0;
    for food in foods {
        let total_calories: f64 = food.calories[1]
            .trim_end_matches("kcal")
            .parse()
            .unwrap_or(0.0);
        total_cals += total_calories * food.nbr_of_portions;
        total_carbs += food.carbs * food.nbr_of_portions;
        total_proteins += food.proteins * food.nbr_of_portions;
        total_fats += food.fats * food.nbr_of_portions;
    }
    let result = json::object!{
        "cals": round(total_cals),
        "carbs": round(total_carbs),
        "proteins": round(total_proteins),
        "fats": round(total_fats),
    };
    result
}
fn round(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}