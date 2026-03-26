# SelfBook Soroban

<img width="1919" height="870" alt="Screenshot 2026-03-26 224011" src="https://github.com/user-attachments/assets/c6ce1e3d-6688-4b93-be74-fcc0aa3c6b62" />


A simple smart contract-based booking system built on Soroban (Stellar).
This project demonstrates how users can create booking slots and allow others to reserve them in a trustless, on-chain environment.

---

## Overview

SelfBook is a decentralized booking system where:

* A user (owner) can create booking slots with a defined price
* Other users can book available slots
* Bookings are stored and managed on-chain
* Users can cancel their bookings securely

This project showcases core blockchain concepts such as:

* State management
* Authentication
* On-chain data storage
* Event logging

---

## Features

* Create booking slots
* Book available slots
* Cancel bookings (by the booker only)
* Retrieve all booking slots
* Input validation (price, ownership, slot index)
* Maximum slot limit per contract
* Event logging for all actions

---

## Smart Contract Structure

### Booking Struct

```rust
Booking {
  owner: Address,
  booker: Option<Address>,
  price: i128,
  booked: bool
}
```

---

## Functions

### `create_slot(owner, price)`

Create a new booking slot.

* Requires authentication
* Price must be greater than 0
* Limited by maximum slot count

---

### `book_slot(user, index)`

Book an available slot.

* Requires authentication
* Cannot book own slot
* Fails if already booked

---

### `cancel_booking(user, index)`

Cancel an existing booking.

* Only the booker can cancel
* Slot must be booked

---

### `get_slots()`

Retrieve all booking slots.

---

## Installation & Setup

### 1. Build Contract

```bash
cargo build --target wasm32-unknown-unknown --release
```

---

### 2. Generate Test Accounts

```bash
stellar keys generate najmi --network testnet --fund
stellar keys generate user1 --network testnet --fund
```

---

### 3. Deploy Contract

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/selfbook_soroban.wasm \
  --source najmi \
  --network testnet
```

Save the generated `CONTRACT_ID`.

---

## Usage (CLI)

### Create Slot

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source najmi \
  --network testnet \
  -- \
  create_slot \
  --owner najmi \
  --price 100
```

---

### Get Slots

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --network testnet \
  -- \
  get_slots
```

---

### Book Slot

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source user1 \
  --network testnet \
  -- \
  book_slot \
  --user user1 \
  --index 0
```

---

### Cancel Booking

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source user1 \
  --network testnet \
  -- \
  cancel_booking \
  --user user1 \
  --index 0
```

---

## Demo Flow

1. Owner creates a slot
2. Retrieve slots (status: not booked)
3. Another user books the slot
4. Retrieve slots (status: booked)
5. User cancels booking
6. Retrieve slots (status: available again)

---

## Key Concepts Demonstrated

* Smart contract development using Rust
* Soroban storage system
* Address-based authentication (`require_auth`)
* On-chain state updates
* Event-driven architecture

---

## Notes

* This project uses simulated pricing (no real token transfer)
* Designed for learning and demonstration purposes
* Optimized for simplicity and clarity

---

## Author

Developed as part of a Stellar Soroban workshop.
