use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct DeployArgs {
    #[arg(long, default_value = "0x013dfbcd524fe71fb25b6367fcc0e2b3f432c61bbbf1d2984c0d45d37cc89f0f")]
    #[arg(help = "Specify the address of Kakarot")]
    pub kakarot_address: String,

    #[arg(long, default_value = "0x01b62d1ea4a35e5a49333ec6de69372b43073d988bf20703972e2737b13daac1")]
    #[arg(help = "Specify the proxy account class hash")]
    pub proxy_account_class_hash: String,

    #[arg(long, default_value = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266")]
    #[arg(help = "Specify the evm address to be deployed")]
    pub evm_address: String,
}
