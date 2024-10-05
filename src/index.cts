// This module is the CJS entry point for the library.

// The Rust addon.
import * as addon from "./load.cjs";

// Use this declaration to assign types to the addon's exports,
// which otherwise by default are `any`.
declare module "./load.cjs" {
  function databaseNew(): any;
  function databaseAdd(db: any, ip: string): void;
  function databaseContains(db: any, ip: string): boolean;
}

export class Database {
  private db: any;
  constructor() {
    this.db = addon.databaseNew();
  }

  add(ip: string) {
    addon.databaseAdd(this.db, ip);
  }

  contains(ip: string) {
    return addon.databaseContains(this.db, ip);
  }
}
