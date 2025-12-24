import installer from './NativeSlhDsa';

export function installRustCrate(): boolean {
  return installer.installRustCrate();
}

export function cleanupRustCrate(): boolean {
  return installer.cleanupRustCrate();
}
