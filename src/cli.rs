use crate::blockchain::Blockchain;
use crate::errors::Result;
pub struct Cli {
    bc: Blockchain,
}

impl Cli {
    pub fn new() -> Result<Cli> {
        Ok(Cli {
            bc: Blockchain::new()?,
        })
    }
        pub fn run(&mut self) -> Result<()> {
            let matches = Command::new("ESGIUM")
                .version("O.1")
                .author("Sisek and Seongo")
                .about("This is a school project about creating a blockchain in rust")
                .subcommand(Command::new("printchain").about("print all the chain blocks"))
                .subcommand(Command::new("addblock").about("Add a block in the blockchain").arg(arg!(<DATA>" The blockchain data ")),
                )
            .get_matches();
        if let Some(ref matches) = matches.subcommand_matches("addblock") {
            self.addblock(String::from(c))?;
        } else {
            println!("Not printing testing lists...");
        }

    if let Some(_) = matches.subcommand_matches("printchain") {
        self.printchain();
    }
    Ok(())
    }
}

fn addblock(&mut self, data: String) -> Result<()> {
    self.bc.add_block(data)
}

fn printchain(&mut self) {
    for b in &mut self.bc.iter() {
        println!("block: {:#?}", b);
    }
}