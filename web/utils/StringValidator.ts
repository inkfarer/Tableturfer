import { ValidatorResult } from '~/utils/Validator';
import isEmpty from 'lodash/isEmpty';
import { isBlank } from '~/helpers/StringHelper';

export const notBlank: (value: string | null) => ValidatorResult = (value) => ({
    isValid: !isBlank(value),
    message: 'Must not be blank'
});

export function maxLength(length: number): (value?: string | null) => ValidatorResult {
    return (value?: string | null) => {
        return {
            isValid: isEmpty(value) || length >= (value?.length ?? 0),
            message: `Must not be over ${length} characters`
        };
    };
}
