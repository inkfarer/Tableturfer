export default {
    clearMocks: true,
    testEnvironment: 'jsdom',
    moduleFileExtensions: [ 'ts', 'js', 'json' ],
    moduleNameMapper: {
        '^~/(.*)': '<rootDir>/$1',
    },
    preset: 'ts-jest',
    transform: {
        '^.+\\.(ts|tsx)?$': 'ts-jest'
    },
    moduleDirectories: ['node_modules', 'src']
};
