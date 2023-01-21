export class TranslatableError extends Error {
    public translationKey: string;

    constructor(translationKey: string, cause?: unknown) {
        super(translationKey, { cause });
        this.translationKey = translationKey;
    }
}
