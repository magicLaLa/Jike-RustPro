use blockchain_rust_part_1::Blockchain;

fn main() {
  tracing_subscriber::fmt().init();

  let mut bc = Blockchain::new();

  bc.mine_block("test -> Bob 2 btc");
  bc.mine_block("test -> Brucce 2 btc");

  bc.blocks_info();
}