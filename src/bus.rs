pub struct Bus {
    pub ram: [u8; 65536] 
}

impl Bus {

    pub fn Read(&self, addr: u16) -> u8 {
        if addr >= 0x0000 && addr <= 0xFFFF {
            self.ram[addr as usize]
        } else {
            0x00
        }
    }

    pub fn Write(&mut self, addr: u16, val: u8) {
            if addr >= 0x0000 && addr <= 0xFFFF {
                self.ram[addr as usize] = val;

        }
        
    }

    pub fn EraseAll(&mut self) {
        for i in 0..self.ram.len() {
            self.ram[i] = 0x00;
        }
    }
}

