# Substrate Kitties based on Substrate Node Template

## Substrate Node Template

Refer to [Substrate Node Template](https://github.com/substrate-developer-hub/substrate-node-template).

## Substrate Kitties

Substrate Kitties is based on Substrate Node Template, [version 3.0.0+monthly-2021-07](https://github.com/substrate-developer-hub/substrate-node-template/releases/tag/v3.0.0%2Bmonthly-2021-07).

It is developed as a pallet of substrate and provides 5 major functions:
- **Create a kitty**: A user with a chain account can create a kitty with a specific amount of stake.
- **Transfer a kitty**: The owner of the kitty can transfer it to another account.
- **Breed a kitty**: The user can breed a kitty from the other 2 kitties.
- **Sell a kitty**: The owner of a kitty can set a price and is listed for sale.
- **Buy a kitty**: The user can buy a kitty from its owner with the list price.

The Kitties Pallet can be a very beginning scaffold of a chain game about raising kitties, such as with extension of NFT, and so on.

### Build and Run

After cloning from the repo, build the source as:

```sh
cargo build --release
```

Run as:
```sh
./target/release/node-template --dev --tmp
```

### Check the test

The command is:

```sh
cargo test -p pallet-kitties
```

The result of testing just look like the picture below:


![Test Result](https://github.com/IanGYan/substrate-kitties/blob/main/test-results.jpg)

For more information, please read the comments from the source codes.