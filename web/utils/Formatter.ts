export function formatMissingValue<T>(obj: T): T | string {
    return obj == null ? '-' : obj;
}

export function formatDate(date: Date | number): string {
    return new Intl.DateTimeFormat('en-GB', {
        timeStyle: 'short',
        dateStyle: 'short'
    }).format(date);
}
