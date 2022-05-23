use speed_transformation::km_per_hour_to_meters_per_second;

fn main() {
    let km_h = 72.0;
    let m_s = km_per_hour_to_meters_per_second(km_h);
    println!("{} km/h is equivalent to {} m/s", km_h, m_s);
}
