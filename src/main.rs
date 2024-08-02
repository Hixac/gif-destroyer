#![allow(warnings)] // don't forget to remove it then

#[macro_use]
mod data_img;
pub use data_img::*;

mod stream;
pub use stream::stream::*;

struct MainData {
	pub header: String,
	
	pub height: u32,
	pub width: u32,
	pub packed_field: PackedField,
	pub global_color_table: Vec<Vector3d>,
	pub images: Vec<Image>,

	stream: Stream,
}

struct PackedField {
	pub gct_size: u8,
	pub gct_flag: bool,
	
	pub color_res: u8,
	pub sort_flag: bool
}

impl MainData {
	
	pub fn new(filename: &str) -> Result<Self, &'static str> {
		let mut stream = Stream::new(filename);

		let header = String::from_utf8(stream.read(6).expect("")).expect("Not u8 format");
		assert_eq!(header, "GIF89a");
		
		let width = stream.uread().expect("");
		let height = stream.uread().expect("");
		
		let field_byte: u8 = stream.bread().expect("");
		
		let packed_field: PackedField = PackedField {
			gct_size: field_byte & 0b00000111, gct_flag: field_byte & 0b00001000 != 0,
			color_res: (field_byte & 0b01110000) >> 4, sort_flag: field_byte & 0b10000000 != 0
		};
		
		let table_size: usize = 2_i32.pow(packed_field.gct_size as u32 + 1) as usize;
		let bci = stream.bread().expect("");
		let par = stream.bread().expect("");
		
		let mut gct: Vec<Vector3d> = Vec::new();
		for _ in 0..table_size {
			let color = vector3d!(stream.read(3).expect(""));
			//print!("({:X}, {:X}, {:X}) ", color.x, color.y, color.z);
			gct.push(color);
		}
		
		let net_ext = stream.read(19);
		
		let mut images: Vec<Image> = Vec::new();

		loop {
			let format = stream.bread().unwrap();

			println!("format {:X}", format);
			
			if format == 0x21 {
				let Some(gce) = stream.read(7) else { break; };
			} else if format == 0x2C {
				let Some(image_descriptor) = stream.read(9) else { break; };

				let Some(code_size) = stream.bread() else { break; };
				
				let mut block_size = stream.bread().unwrap();
				let mut block: Vec<u8> = Vec::new();
				while block_size != 0 {
					block.append(&mut stream.read(block_size as usize).unwrap());
					block_size = stream.bread().unwrap();
				}
				lzw_decode(&block, code_size, gct.len());
				
			} else if format == 0x3B {
				break;
			}
		}
		
		Ok(Self { header, height, width, packed_field, global_color_table: gct, images, stream })
	}

	
}

fn lzw_decode(block: &Vec<u8>, code_size: u8, reserved: usize) -> Vec<i32> {
	
	let clear = 1 << code_size;
	let end = clear + 1;
	let mut min_code = 1 + code_size as usize;

	let mut offset: usize = 0;

	fn read(bin: &Vec<u8>, offset: &mut usize, size: usize) -> usize {
		let mut code = 0;
		for i in (0..size) {
			if (bin[*offset >> 3] & (1 << (*offset & 7))) != 0 {
				code |= 1 << i;
			}
			*offset += 1;
		}

		code
	}

	let mut code_table: Vec<Vec<i32>> = Vec::new();
	for i in (0..reserved as i32) {
		code_table.push([i].to_vec());
	}
	code_table.push([].to_vec());
	code_table.push([-1].to_vec());

	let mut index_stream: Vec<i32> = Vec::new();

	let mut code = 0;

	loop {
		let prev_code = code;
		code = read(&block, &mut offset, min_code);

		if code == clear {
			code_table.truncate(reserved + 2);
			min_code = 1 + code_size as usize;
			continue;
		}
		if code == end {
			break;
		}

		if code < code_table.len() {
			if prev_code != clear {
				let mut code_info = code_table[prev_code].clone();
				code_info.push(code_table[code][0].clone());
				code_table.push(code_info);
			}
		} else {
			if code != code_table.len() { assert!(false, "Fuck... {}, {}, {}", code, code_table.len(), min_code); }

			let mut code_info = code_table[prev_code].clone();
			code_info.push(code_table[prev_code][0].clone());
			code_table.push(code_info);
		}
		index_stream.append(&mut code_table[code].clone());
		
		if code_table.len() == 1 << min_code && min_code < 12 {
			min_code += 1;
		}
	}

	index_stream
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
