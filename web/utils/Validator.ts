import { InjectionKey, provide, reactive, watch, WatchSource } from 'vue';

export type ValidatorMap = Record<string, ValidatorResult>;
export const VALIDATOR_INJECTION_KEY: InjectionKey<Record<string, ValidatorResult>> = Symbol();

export interface ValidatorResult {
    isValid: boolean | null
    message?: string | null
}

export function validator<T>(
    value: WatchSource<T>,
    immediate: boolean,
    ...validators: ((value: T) => ValidatorResult)[]
): ValidatorResult {
    const result: ValidatorResult = reactive({
        isValid: null
    });

    watch(value, newValue => {
        for (let i = 0; i < validators.length; i++) {
            const validatorResult = validators[i](newValue);
            if (!validatorResult.isValid) {
                result.isValid = false;
                result.message = validatorResult.message;
                break;
            } else {
                result.isValid = true;
                result.message = null;
            }
        }
    }, { immediate });

    return result;
}

export function allValid(validators: Record<string, ValidatorResult>): boolean {
    return Object.values(validators).every(validator => validator.isValid);
}

export function provideValidators(validators: Record<string, ValidatorResult>): void {
    provide(VALIDATOR_INJECTION_KEY, validators);
}
