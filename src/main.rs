
#[macro_use]
mod data_img;
use data_img::*;

struct MainData {
	header: String,

	height: u16,
	width: u16,
	packed_field: PackedField,
	global_color_table: Vec<u8>,
	images: Vec<Image>,
}

struct PackedField {
	gct_size: u8,
	gct_flag: bool,
	
	color_res: u8,
	sort_flag: bool
}

impl MainData {
	
	pub fn new(filename: &str) -> Result<Self, &'static str> {
		let bytes = std::fs::read(filename).expect("No file");

		let header = String::from_utf8(bytes[0..=5].to_vec()).expect("Not u8 format");
		assert_eq!(header, "GIF89a");
		
		let width = to_u16(&bytes[6..=7]);
		let height = to_u16(&bytes[8..=9]);
		
		let field_byte: u8 = bytes[10];
		
		let packed_field: PackedField = PackedField {
			gct_size: field_byte & 0b00000111, gct_flag: field_byte & 0b00001000 != 0,
			color_res: (field_byte & 0b01110000) >> 4, sort_flag: field_byte & 0b10000000 != 0
		};
		
		let table_size: usize = 3*2_i32.pow(packed_field.gct_size as u32 + 1) as usize;
		let gct: Vec<u8> = bytes[13..13+table_size].to_vec();		

		let mut images: Vec<Image> = Vec::new();
		
		for i in 13+table_size..bytes.len() {
			let byte: u8 = bytes[i];
			if byte == 0x21 && bytes[i+1] == 0xF9 { // GCE
				let gce: Vec<u8> = bytes[i+2..=i+7].to_vec();

				let packed_field = bytes[i+17];
				let lct_flag: bool = packed_field & 0x80 != 0; // for later use
				
				if bytes[i+8] == 0x2C {
					let position = Vector2d::new(to_u16(&bytes[i+9..=i+10]) as i32, to_u16(&bytes[i+11..=i+12]) as i32);
					let resolution = Vector2d::new(to_u16(&bytes[i+13..=i+14]) as i32, to_u16(&bytes[i+15..=i+16]) as i32);
					
					println!("Position of image {}x{}", position.x, position.y);
					println!("Resolution of image {}x{}", resolution.x, resolution.y);

					let code_size = bytes[i+18];
					let block_len = bytes[i+19];
					let block = bytes[i+20..i+20+block_len as usize].to_vec();
					
					println!("Block size: {}, and Code size: {}", block_len, code_size);
					println!("{:X?}", block);
					
					
					
					let image: Image = Image { position, resolution };
					images.push(image);

				}
			}
		}
		
		Ok(Self { header, height, width, packed_field, global_color_table: gct, images })
	}
}

fn to_u16(array: &[u8]) -> u16 {
    ((array[0] as u16) <<  0) +
    ((array[1] as u16) <<  8)
}

fn main() -> std::process::ExitCode {
	let chunk = MainData::new("holymoly.gif");
	
	match chunk {
		Ok(c) => {
			println!("\n{}", c.header);
			println!("{}:{}", c.height, c.width);
		}
		Err(_) => {
			return std::process::ExitCode::FAILURE;
		}
	}
	
    std::process::ExitCode::SUCCESS
}
