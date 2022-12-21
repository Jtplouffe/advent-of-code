use encrypted_file::EncryptedFile;

mod encrypted_file;

fn main() {
    let input = include_str!("./input");

    let mut encrypted_file_1 = EncryptedFile::from(input);
    encrypted_file_1.decrypt(1);
    println!("Part 1: {}", encrypted_file_1.grove_coordinates());

    let mut encrypted_file_2 = EncryptedFile::from(input);
    encrypted_file_2.apply_key(811589153);
    encrypted_file_2.decrypt(10);
    println!("Part 2: {}", encrypted_file_2.grove_coordinates());
}
