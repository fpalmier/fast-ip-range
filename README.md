# fast-ip-range

fast-ip-range is a fast IP range library for Node.js.

This project was bootstrapped by [create-neon](https://www.npmjs.com/package/create-neon).

## Installation

```
npm install fast-ip-range
```

## Usage

```typescript
import { Database } from "fast-ip-range";

const db = new Database();

db.add("10.0.0.0/8");
db.add("172.16.0.0/16");
db.add("192.168.1.0/24");

console.log(db.contains("172.16.32.1"));
console.log(db.contains("19.168.1.1"));
```
