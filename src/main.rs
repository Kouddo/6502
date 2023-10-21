mod bus;

use bus::Bus;



fn main() {
    println!("Hello, world!");

    let mut mainBus = Bus {
        ram: [0; 65536],
    };

    mainBus.EraseAll();

    mainBus.Write(0x0001, 0x11);
    
    let val = mainBus.Read(0x0001);

    println!("{}", val);

}
