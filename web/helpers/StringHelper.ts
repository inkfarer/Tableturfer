export function isBlank(value?: string | null): boolean {
    return value === null || value === undefined || value.trim() === '';
}
