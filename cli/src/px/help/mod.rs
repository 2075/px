use super::config;

const PX_ART: &str = "
	_____________  ___
	\\_____   \\   \\/  /
	 |    ___/\\     / 
	 |   |    /     \\ 
	 |___|   /___/\\  \\
	                \\_/

";

pub fn help() {

	println!( "{}", PX_ART );
	config::version();
	println!("
	usage:
	px <command> <params>

		run <ref> — execute a binary fetched from chain

		pkg {{ ls | add | rm }} <ref> — mutate your local registry
		pkg {{ upgrade }} {{ force }} <ref> — upgrade local package, * will upgrade all local

		web3 {{ accounts }} — list accounts from configured web3 node

		ipfs {{ ls }} — list file hashes from configured ipfs node

		cfg ls — list config
		cfg get <key> — list config value by key arg
		cfg set <node> <key> <value> — set value for key in node

		update — fetch and install the latest px
		help — show this
	");

}


