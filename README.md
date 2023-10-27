

# Ink Stable Coin

![iusd](https://github.com/1010adigupta/ink-Stable-Coin/assets/121158631/773705e3-96c5-485e-b2e9-5634795940c1)

## Description

Ink Stable Coin is a crypto-collateralized stable coin implementation written in ink! for ALEPH ZERO and Substrate-based parachains.

## Architecture

![architecture](https://github.com/1010adigupta/ink-Stable-Coin/assets/121158631/2d66d4b3-4f86-43aa-8177-af10ccbb445d)

## Deployment

To deploy this contract, follow these steps:

1. Clone the project:

   ```bash
   git clone https://github.com/1010adigupta/ink-Stable-Coin.git
   ```

2. Navigate to the project directory:

   ```bash
   cd ink-Stable-Coin
   cd contracts
   cd stable-coin
   ```

3. Generate the contract binaries:

   ```bash
   cargo contract build
   ```

4. A stable-coin-contract.contract binary will be generated. Now, go to [substrate-contracts-ui](https://contracts-ui.substrate.io/), select ALEPH ZERO testnet, and upload the .contract binary.

5. Instantiate the contract by supplying the dia-oracle address on ALEPH ZERO testnet as an argument to the constructor. The dia-oracle contract address is:

   ```
   5F5z8pZoLgkGapEksFWc2h7ZxH2vdh1A9agnhXvfdCeAfS9b
   ```

## Roadmap

Our roadmap includes the following milestones:

- **Savings Feature**: Holders of the Dai stable coin can earn the Dai Savings Rate (DSR) on the Dai they hold by locking it into a special smart contract. There are no fees involved, no geographic constraints, and no liquidity impediments—no minimum deposit is required to earn the DSR, and all or any portion of Dai can be withdrawn at any time.

   The Dai Savings Rate contract is accessible through Oasis Save and other projects that have integrated the DSR, including the OKEx marketplace and the Argent wallet. The DSR is not only a propeller of financial freedom, offering complete user control, but also game-changing for the DeFi movement.

- **Additional Browser Support**: We plan to expand our platform's browser support to ensure a seamless user experience.

- **Add More Integrations**: We are actively working on integrating our stable coin with various DeFi projects and platforms to enhance its usability.
