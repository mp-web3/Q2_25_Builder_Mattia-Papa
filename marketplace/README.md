# NFT Marketplace on Solana

This project implements a decentralized NFT marketplace on Solana where users can list, purchase, and delist NFTs with customizable marketplace fees.

## Program Overview

The marketplace supports:
- Creating configurable marketplaces with custom fee structures
- Listing NFTs for sale with collection verification
- Purchasing NFTs with automatic fee distribution
- Delisting NFTs by their original owners

## PHASE 1: MARKETPLACE INITIALIZATION

### Step 1: Create Marketplace
- **Admin Account**: Initializes and funds the marketplace
- **Marketplace PDA**: Generated using seeds `[b"marketplace", name.as_str().as_bytes()]`
- **Treasury PDA**: Generated using seeds `[b"treasury", marketplace.key().as_ref()]`
- **Reward Mint**: Generated using seeds `[b"rewards", marketplace.key().as_ref()]`

### Step 2: Configure Marketplace
- Administrator sets:
  - Marketplace name (`marketplace.name`)
  - Fee percentage in basis points (`marketplace.fee`)
  - Required PDAs and their bumps

## PHASE 2: NFT LISTING

### Step 1: List NFT for Sale
- **Maker (Seller)**: The NFT owner who wishes to sell
- **NFT Mint**: The mint address of the NFT being listed
- **Collection Verification**: Confirms NFT belongs to required collection
- **Listing PDA**: Generated using seeds `[marketplace.key().as_ref(), maker_mint.key().as_ref()]`
- **Vault Account**: Associated token account owned by the listing PDA

### Step 2: Store Listing Data
- Program creates listing account with:
  - Maker's public key (`listing.maker`)
  - NFT mint address (`listing.maker_mint`)
  - Sale price in lamports (`listing.price`)
  - Bump seed for PDA (`listing.bump`)

### Step 3: Escrow NFT in Vault
- NFT is transferred from maker's token account to the vault
- Vault's authority is set to the listing PDA
- This happens in the `deposit_nft()` function using `transfer_checked`

## PHASE 3: PURCHASE EXECUTION

### Step 1: Buyer Discovers Listing
- Taker (Buyer) browses available listings
- Verifies the NFT, price, and collection details

### Step 2: Execute Purchase
- Buyer initiates purchase with:
  - Their SOL wallet (to pay price)
  - Their token account (to receive NFT)
  - Listing PDA (identifying which listing to purchase)

### Step 3: Payment Processing
- Program calculates fee amount (`marketplace_fee = price * fee / 10000`)
- SOL is transferred from buyer to seller (minus fee)
- Fee is transferred to marketplace treasury

### Step 4: NFT Transfer
- Program uses the listing PDA to sign for:
  - Transferring NFT from vault to buyer's token account
  - Closing the listing after successful transfer

## PHASE 4: DELISTING (Optional)

### Step 1: Maker Requests Delisting
- Original NFT owner initiates delisting
- Program verifies maker's identity against listing data

### Step 2: Return NFT
- Program transfers NFT from vault back to maker's token account
- Listing account is closed and rent returned to maker

## Security Considerations

1. **Authority Control**: NFTs are only transferable by the program signing with the listing PDA
2. **Collection Verification**: Only verified NFTs from designated collections can be listed
3. **Maker Verification**: Only the original lister can delist their NFTs
4. **Fee Protection**: Fees are calculated trustlessly and sent directly to treasury

## Technical Components

- **State Accounts**: `Marketplace`, `Listing`
- **Instructions**: `Initialize`, `List`, `Purchase`, `Delist`
- **PDAs**: Marketplace, Listings, Treasury, Vaults
- **Error Handling**: Comprehensive error types with custom messages
