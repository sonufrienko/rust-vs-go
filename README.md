# Compare performance and resources for CLI build with Rust and Go

Date: `June 7, 2022`

Rust: `1.61.0`

Go: `1.18.3`

## Getting Started

cd cli-rust; cargo build --release; cd ../cli-go; go build; cd ..

## Step 1: Empty CLI

|         | Bin size | Mem  |
| ------- | -------- | ---- |
| Rust    | 0.46M    | 344K |
| Go      | 1.1M     | 856K |
| Node.js | -        | 8.7M |

## Step 2: Serialize JSON 10M times

|         | Bin size | Mem   | Time   |
| ------- | -------- | ----- | ------ |
| Rust    | 531K     | 480K  | 10.88s |
| Go      | 2.2M     | 6.5M  | 39.78s |
| Node.js | -        | 11.3M | 11.92s |
