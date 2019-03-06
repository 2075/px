# dependency: license type contract standard

## create a public contract to reference license conditions globally

### license types 
0 unlicensed
1 mit
2 apache
3 gpl

### semver version string
0.0.0-alpha

### text resources array
iso code,	ipfs address text resource


# px package contract

## contract payload

{
	"name":    "unique_package-name",
	"version": "0.0.0",
	"license": "0xabcdefabcdef",
	"address": "0x00000123456789abcdef",
	"creator": "0x00112233445566778899",
	"owner":   "0xaabbccddeeff00000000",
	"pkg":     "QmWGeRAEgtsHW3ec7U4qW2CyVy7eA2mFRVbk1nb24jFyks"
}

## package config on ipfs

{
	"uri": "0x000000123456789abcdef",
	"version": "0.0.0",
	"name": "unique_package-name",
	"description": "a package description file",
	"standard": "ERC-721",
	"license": "UNLICENSED",
	"author": {
		"name": "john doe",
		"organisation": "unknown, co.",
		"uia": "0x00112233445566778899"
	},
	/*
		ipfs address for package
	 */
	"pkg": "QmWGeRAEgtsHW3ec7U4qW2CyVy7eA2mFRVbk1nb24jFyks",

}

# registry

## registry dapp

- show existing packages based on filters, like npm, gem, cargo
- register a package by deploying a contract with the owner account ( through metamask )
- 