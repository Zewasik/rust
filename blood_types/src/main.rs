use blood_types::*;

fn main() {
    let blood_type: BloodType = "A-".parse().unwrap();
    let mut temp = blood_type.recipients();
    temp.sort();
    println!("recipients of O+ {:?}", temp);
    println!("donors of O+ {:?}", blood_type.donors());
    let another_blood_type: BloodType = "A+".parse().unwrap();
    println!(
        "donors of O+ can receive from {:?} {:?}",
        &another_blood_type,
        blood_type.can_receive_from(&another_blood_type)
    );
}
