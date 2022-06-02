use json::object;

pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {
    let mut cal_sum = 0.0;
    let mut carb_sum = 0.0;
    let mut protein_sum = 0.0;
    let mut fat_sum = 0.0;

    for food in foods {
        cal_sum +=
            food.calories[1].replace("kcal", "").parse::<f64>().unwrap() * food.nbr_of_portions;
        carb_sum += food.carbs * food.nbr_of_portions;
        protein_sum += food.proteins * food.nbr_of_portions;
        fat_sum += food.fats * food.nbr_of_portions;
    }

    object! {
        "cals": (cal_sum * 100.0).round() / 100.0,
        "carbs": (carb_sum * 100.0).round() / 100.0,
        "proteins": (protein_sum * 100.0).round() / 100.0,
        "fats": (fat_sum * 100.0).round() / 100.0,
    }
}
