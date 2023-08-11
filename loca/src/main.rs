use loca::parse_loca;

fn main() {
    let path = "/run/media/minus/Kairos/Games/Modding/BG3/Out/Localization/English/Localization/English/english.loca";
    let data = std::fs::read(path).unwrap();
    let loca = parse_loca(&data).unwrap();

    println!("Loca: {loca:#?}");
}
