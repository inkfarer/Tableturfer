export function formatMissingValue<T>(obj: T): T | string {
    return obj == null ? '-' : obj;
}
