declare module "node-macos-name-uuid" {
    /**
     * Get the friendly computer name.
     * @returns A string result. Null on failure.
     */
    export function getComputerName(): string | null;

    /**
     * Get the hardware UUID.
     * @returns A string result. Null on failure.
     */
    export function getHardwareUuid(): string | null;
}