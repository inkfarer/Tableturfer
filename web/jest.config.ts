export default {
    clearMocks: true,
    restoreMocks: true,
    testEnvironment: 'jsdom',
    moduleFileExtensions: [ 'ts', 'js', 'json' ],
    moduleNameMapper: {
        '^~/(.*)': '<rootDir>/$1',
        '^#imports$': '<rootDir>/.nuxt/types/imports.d.ts'
    },
    preset: 'ts-jest',
    transform: {
        '^.+\\.(ts|tsx)?$': 'ts-jest'
    },
    moduleDirectories: ['node_modules', 'src']
};
