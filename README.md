# fast-ip-range

fast-ip-range is a high-performance library for efficiently checking if an IP address falls within a set of CIDR ranges. Key features include:

- Blazing fast performance, optimized for speed
- Support for both IPv4 and IPv6 addresses
- Written in Rust for maximum efficiency
- Packaged as a Node.js module using Neon bindings
- Simple API for easy integration into JavaScript/TypeScript projects
- Ideal for applications requiring frequent IP range checks at scale

Whether you're building network security tools, geo-blocking systems, or any application that needs rapid IP range matching, fast-ip-range delivers the speed and reliability you need.

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


This project was bootstrapped by [create-neon](https://www.npmjs.com/package/create-neon).
