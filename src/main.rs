use goblin::elf::Elf;
use std::fs;
use std::process;

fn main() {
    // 1. Load the target ELF file into a byte buffer
    let path = "test/test";
    let buffer = fs::read(path).expect("Failed to read file");

    // 2. Parse the buffer
    let elf = Elf::parse(&buffer).expect("Failed to parse ELF");

    // 3. Find the .text section
    let text_section = elf.section_headers.iter().find(|header| {
        // Get the name from the section header string table
        elf.shdr_strtab.get_at(header.sh_name) == Some(".text")
    });

    match text_section {
        Some(header) => {
            // 4. Use the offset and size to slice the original buffer
            let start = header.sh_offset as usize;
            let end = start + header.sh_size as usize;
            let text_binary = &buffer[start..end];

            println!("Successfully extracted .text section!");
            println!("Size: {} bytes", text_binary.len());

            // Optional: Write the raw binary to a file
            // fs::write("text_section.bin", text_binary).unwrap();
        }
        None => {
            eprintln!("Could not find .text section in the ELF file.");
            process::exit(1);
        }
    }
}