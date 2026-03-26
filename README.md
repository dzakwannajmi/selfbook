# SelfBook Soroban

A smart contract-based booking system built on Soroban (Stellar).
This project demonstrates how users can create, manage, and complete booking transactions in a trustless, on-chain environment.

---

## Overview

SelfBook is a decentralized booking system where:

* A user (owner) can create booking slots with a defined price
* Other users can book available slots
* Owners can complete bookings
* Booking lifecycle is stored and managed on-chain
* Each action is recorded with timestamps

This project showcases core blockchain concepts such as:

* State management
* Authentication
* On-chain data storage
* Event-driven logic
* Transaction lifecycle handling

---

## Project Structure

```
selfbook-soroban/
├── contracts/
│   └── notes/
│
├── src/
│   ├── lib.rs        # Main smart contract logic
│   └── test.rs       # Unit tests
│
├── target/           # Build output (auto-generated)
│
├── Cargo.toml        # Project configuration (Rust)
├── Cargo.lock        # Dependency lock file
├── Makefile          # Build and automation commands
├── .gitignore        # Ignored files for Git
├── README.md         # Project documentation
└── stellar-expert.png & stellar lab.png    # Demo UI / CLI result
```

### Description

* `src/lib.rs` → Main smart contract implementation
* `src/test.rs` → Unit testing for contract functions
* `contracts/notes/` → Additional contract-related modules (if any)
* `target/` → Compiled WASM output
* `Cargo.toml` → Rust project configuration
* `Makefile` → Simplifies build and deployment commands

---

## Features

* Create booking slots
* Book available slots
* Complete booking (by owner)
* Retrieve all booking slots
* Retrieve slots by owner
* Booking lifecycle management (available → booked → completed)
* Timestamp tracking (created & booked time)
* Input validation (price, ownership, slot index)
* Maximum slot limit per contract

---

## Smart Contract Structure

### Booking Struct

```rust
Booking {
  owner: Address,
  booker: Option<Address>,
  price: i128,
  status: u32,        // 0 = available, 1 = booked, 2 = completed
  created_at: u64,
  booked_at: Option<u64>
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
* Only available slots can be booked
* Records booking timestamp

---

### `complete_booking(owner, index)`

Mark a booking as completed.

* Only the owner can complete
* Slot must be in booked state

---

### `get_slots()`

Retrieve all booking slots.

---

### `get_slots_by_owner(owner)`

Retrieve all slots created by a specific owner.

---

## Installation & Setup

### 1. Build Contract

```bash
cargo build --target wasm32-unknown-unknown --release
```

---

### 2. Generate Test Accounts

```bash
stellar keys generate user1 --network testnet --fund
stellar keys generate user2 --network testnet --fund
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

### Get All Slots

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

### Complete Booking

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source najmi \
  --network testnet \
  -- \
  complete_booking \
  --owner najmi \
  --index 0
```

---

### Get Slots by Owner

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --network testnet \
  -- \
  get_slots_by_owner \
  --owner najmi
```

---

## Demo Flow

1. Owner creates a slot
2. Retrieve slots (status: available)
3. Another user books the slot
4. Retrieve slots (status: booked)
5. Owner completes the booking
6. Retrieve slots (status: completed)

---

## Key Concepts Demonstrated

* Smart contract development using Rust
* Soroban storage system
* Address-based authentication (`require_auth`)
* On-chain state lifecycle (multi-state transitions)
* Timestamp-based activity tracking

---

## Notes

* This project uses simulated pricing (no real token transfer)
* Designed for learning and demonstration purposes
* Focused on clarity and real-world booking logic

---

## Author

Developed as part of a Stellar Soroban workshop.