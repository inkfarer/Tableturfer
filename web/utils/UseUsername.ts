import { useState } from '#imports';

export const useUsername = () => useState<string | null>('username', () => null);
