use ls::pak;

fn main() {
    let path =
        "../../../../../SteamLib/steamapps/common/Baldurs Gate 3/Data/Localization/English.pak";
    let file = std::fs::File::open(path).unwrap();
    let file = std::io::BufReader::new(file);

    let package = pak::read_package(file, &path, false).unwrap();

    println!("Package: {:#?}", package);
}
