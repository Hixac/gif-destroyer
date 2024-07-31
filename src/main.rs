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
			let Some(gce) = stream.read(8) else { break; };
			let Some(image_descriptor) = stream.read(10) else { break; };

			let Some(code_size) = stream.bread() else { break; };
			let Some(block_size) = stream.bread() else { break; };

			let Some(block) = stream.read(1 + block_size as usize) else { break; };

			lzw_decode(block, code_size + 1, gct.len() - 1);
			
		}
		
		Ok(Self { header, height, width, packed_field, global_color_table: gct, images, stream })
	}

	
}

// https://habr.com/ru/articles/127083/ - useful
fn lzw_decode(block: Vec<u8>, code_size: u8, reserved: usize) -> Vec<usize> {
	
	let mut min_code = code_size as usize;

	let mut offset: usize = 0;
	
	let mut block_str = String::new();
	for i in block {
		block_str.push_str(&((i & 1 != 0) as u8).to_string());
		block_str.push_str(&((i & 2 != 0) as u8).to_string());
		block_str.push_str(&((i & 4 != 0) as u8).to_string());
		block_str.push_str(&((i & 8 != 0) as u8).to_string());
		block_str.push_str(&((i & 16 != 0) as u8).to_string());
		block_str.push_str(&((i & 32 != 0) as u8).to_string());
		block_str.push_str(&((i & 64 != 0) as u8).to_string());
		block_str.push_str(&((i & 128 != 0) as u8).to_string());
	}

	//println!("end {}, clear {}, reserved {}", end, clear, reserve);

	#[derive(PartialEq)]
	enum Code {
		Val(Vec<usize>),
		Clear,
		EOI,
	}

	impl Code {
		fn unwrap(&self) -> Vec<usize> {
			if let Code::Val(v) = self {
				v.clone()
			} else {
				panic!("Failed to unwrap");
			}
		}
	}
	
	impl Clone for Code {
		fn clone(&self) -> Self {
			match self {
				Code::Val(v) => Code::Val(v.clone()),
				Code::Clear => Code::Clear,
				Code::EOI => Code::EOI
			}
		}
	}
	
	fn read(bin_str: &String, offset: &mut usize, bits: usize) -> usize {
		let val: String = bin_str[*offset..*offset+bits]
			.chars()
			.rev()
			.collect();
		
		*offset += bits;

		println!("{}", val);
		
		usize::from_str_radix(&val, 2).unwrap()
	}
	
	let mut code_table: Vec<Code> = (0..reserved)
		.into_iter()
		.map(|i| -> Code {
			Code::Val(vec![i])
		}).collect();
	
	code_table.push(Code::Clear);
	code_table.push(Code::EOI);
	
	let mut index_stream: Vec<usize> = Vec::new();
	
	let mut code = read(&block_str, &mut offset, min_code);	
	code_table.truncate(reserved);

	code = read(&block_str, &mut offset, min_code);
	index_stream.append(&mut code_table[code].clone().unwrap());

	let mut prev_code_info: Vec<usize> = code_table[code].unwrap();
	
	loop {
		if offset + min_code > block_str.len() {
			let diff = block_str.len() - offset;
			code = read(&block_str, &mut offset, diff);
			println!("{}, {}", offset, min_code);
			println!("end code - {}, {}, {}", code, diff, block_str.len());
			break;
		}
		
		if code_table.len() > 2_usize.pow(min_code as u32) {
			min_code += 1;

			//println!("////////////////////// {} /////////////////////////", min_code);
		}

		//println!("{}", code_table.len());
		
		code = read(&block_str, &mut offset, min_code);
		let mut code_info = code_table.get(code).cloned();
		
		//println!("{}", code);
		//println!("{:?}", prev_code_info);
		//println!("index stream - {:?}", index_stream);
		
		match code_info {
			Some(info) => {
				//println!("Some");
				
				index_stream.append(&mut info.clone().unwrap());
				
				let k = info.unwrap()[0];
				prev_code_info.push(k);
				code_table.push(Code::Val(prev_code_info));

				prev_code_info = info.unwrap();
			},
			None => {
				//println!("None");
				
				let k = prev_code_info[0];
				prev_code_info.push(k);
				
				index_stream.append(&mut prev_code_info.clone());

				code_table.push(Code::Val(prev_code_info.clone()));
			}
		}

	}

	println!("//////////// index stream len -  {} /////////////", index_stream.len());
	
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
