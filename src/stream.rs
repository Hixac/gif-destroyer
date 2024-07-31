pub mod stream {

	pub struct Stream {
		pos: usize,
		chunk: Vec<u8>
	}

	impl Stream {
		pub fn new(filename: &str) -> Self {
			let chunk = std::fs::read(filename).expect("No file found");
			
			Self { pos: 0, chunk }
		}

		pub fn is_end(&self) -> bool {
			self.pos >= self.chunk.len()
		}
		
		pub fn read(&mut self, size: usize) -> Option<Vec<u8>> {
			if self.pos + size >= self.chunk.len() { return None; }
			let buf = self.chunk[self.pos..self.pos+size].to_vec();
			self.pos += size;
			Some(buf)
		}
		
		fn to_u16(&self, array: &[u8]) -> u16 {
			((array[0] as u16) <<  0) +
			((array[1] as u16) <<  8)
		}

		pub fn bread(&mut self) -> Option<u8> {
			let byte = self.read(1);
			match byte {
				Some(b) => return Some(b[0]),
				None => return None
			}
		}
		
		pub fn uread(&mut self) -> Option<u32> {
			let num = self.read(2);
			match num {
				Some(n) => return Some(self.to_u16(&n) as u32),
				None => return None
			}
			
		}

	}
	
}
