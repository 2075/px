# PX
## The web3 package manager

PX ("Packs") is a smart contract and ipfs based universal package registry and manager for binaries and buildable packages as a secure replacement for existing package managers like npm and foremost as an extension for the growing ecosystem of the Web3.

* This is currently experimental, do not expect anything to work.

## Requirements
- rust
- docker
- ipfs
- ganache
- node

## Todo / in progress
- rust interface to ipfs, ethereum, substrate
- lightweight identity provider implementing erc725
- px registry contract
- px package contract

# Why

Often external packages are used to extend functionality of applications during engineering a product. While the usage of external packages  enhances overall development velocity and experience, usage of those packages carries a plethora of risk, which especially when aiming at privacy and security focussed software -- which we should all aim for -- should definitely be mitigated.

PX offers unified api headers for binaries and packages which define a common interface for registration, upgrading, auditing and execution. This common interface is proposed as an EIP to result in a standard contract which can be extended to reflect individual api endpoints.

During development engineering can be sped up in a secure fashion, while the resulting package descriptors enable smaller distributables for the dApp, as it can dynamically load external, audited packages during runtime.

# Workflow


## Register your identity

The PX Dapp encourages strong relation between developing entities and registered artefacts to encourage quality and enable independent rating of the published code in respect to developer credibility, security and efficiency. furthermore it enables additional licensing and payment options for published artefacts to increase moentization options for developers.

## Register your package / binary

The PX Dapp enables creation of unique identifier contracts containing

	- package identifier payload
	- versioning information
	- creator, ownership relation
	- license terms

the package identifier payload contains information to resolve artefacts stored on ipfs.

these artefacts are compressed archives containing

	- package header

	- binary uri
	containing all required dependencies for the respective architecture

	- source uri to build the binary
	building a package might involve package resolution during the build process which may require additional packages. these should be resolved automagically during the build.




# Workflow

`DApp <- dapp.json <- px(n).json`

## dapp.json
{
	"id": "publickey"
	"name": "my-new-dapp"
}





Dapp < dapp header contract
Package < package header contract

* Registration cost paid by creator, can be sponsored by an operator acting on behalf of creator
* Execution cost distributed between Validators and Creators
* Identity via ERC725 ecosystem



DApp <- px.json <- package keys
