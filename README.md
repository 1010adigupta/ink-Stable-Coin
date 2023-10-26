
![Logo](https://github.com/1010adigupta/ink-Stable-Coin/blob/master/contracts/readme-images/iusd.png)


## Description
A crypto collaterised stable coin implementation written in ink!, for ALEPH ZERO and Substrate based parachains.
## Architecture
![Logo](https://github.com/1010adigupta/ink-Stable-Coin/blob/master/contracts/readme-images/architecture.png)
## Deployment

Clone the project

```bash
  git clone https://github.com/1010adigupta/ink-Stable-Coin.git
```

Go to the project directory

```bash
  cd ink-Stable-Coin
  cd contracts
  cd stable-coin
```

Generate binaries

```bash
  cargo contract build
```

A stable-coin-contract.contract binary will be generated. Now go to [substrate-contracts-ui](https://contracts-ui.substrate.io/), select ALEPH ZERO testnet, now upload the .contract binary.
Instantiate the contract by supplying dia-oracle address on ALEPH ZERO testnet as an argument to the constructor.
dia-oracle contract address on ALEPH ZERO testnet:
```
5F5z8pZoLgkGapEksFWc2h7ZxH2vdh1A9agnhXvfdCeAfS9b
```
## Roadmap

- Additional browser support

- Add more integrations

