import { parseDeck } from '~/utils/DeckParser';

describe('DeckParser', () => {
    describe('parseDeck', () => {
        it('behaves as expected when the input is blank', () => {
            expect(() => parseDeck('   ')).toThrow('deckParser.error.noInput');
        });

        it('behaves as expected when the input is unknown', () => {
            expect(() => parseDeck('mystery format that i do not understand')).toThrow('deckParser.error.unknownInput');
        });

        describe('koishi', () => {
            const errorMessage = 'deckParser.error.unableToParse.koishi';

            it('behaves as expected with invalid URLs', () => {
                expect(() => parseDeck('http:/haha just kidding there is no deck here')).toThrow(errorMessage);
            });

            it('behaves as expected if the URL has no deck info', () => {
                expect(() => parseDeck('https://tableturf.koishi.top/?something-else=hello')).toThrow(errorMessage);
            });

            it('behaves as expected if the given deck is empty', () => {
                expect.assertions(2);
                try {
                    parseDeck('https://tableturf.koishi.top/?deck=%5B%5D');
                } catch (e: any) {
                    expect(e.translationKey).toEqual(errorMessage);
                    expect(e.cause.message).toEqual('Not a number: ""');
                }
            });

            it('behaves as expected if the URL has no usable deck info', () => {
                expect.assertions(2);
                try {
                    parseDeck('https://tableturf.koishi.top/?deck=%2Csilly%20little%20test');
                } catch (e: any) {
                    expect(e.translationKey).toEqual(errorMessage);
                    expect(e.cause.message).toEqual('Not a number: "silly little tes"');
                }
            });

            it('behaves as expected if the deck contains non-existent cards', () => {
                expect.assertions(2);
                try {
                    parseDeck('https://tableturf.koishi.top/?deck=%5B6%2C13%2C22%2C28%2C40%2C999%2C45%2C52%2C55%2C56%2C159%2C137%2C141%2C103%2C92%5D');
                } catch (e: any) {
                    expect(e.translationKey).toEqual(errorMessage);
                    expect(e.cause.message).toEqual('Could not find card number 999');
                }
            });

            it('behaves as expected if the deck has too many cards', () => {
                const result = parseDeck('https://tableturf.koishi.top/?deck=%5B6%2C13%2C13%2C13%2C13%2C13%2C22%2C28%2C40%2C99%2C45%2C52%2C55%2C56%2C159%2C137%2C141%2C103%2C92%2C33%2C44%2C55%2C66%2C77%2C88%2C99%2C11%5D');
                expect(result).toEqual({
                    id: null,
                    name: '',
                    cards: [
                        'ShooterNormal00',
                        'BlasterMiddle00',
                        'RollerNormal00',
                        'ChargerNormal00',
                        'SpinnerStandard00',
                        'Utsuho',
                        'ManeuverNormal00',
                        'StringerNormal00',
                        'SaberLight00',
                        'BombSplash',
                        'Denchinamazu',
                        'TakoDozer',
                        'Shake',
                        'Batoroika',
                        'Mother'
                    ]
                });
            });

            it('behaves as expected if the deck has less cards than required', () => {
                const result = parseDeck('https://tableturf.koishi.top/?deck=%5B6%2C13%2C13%5D');
                expect(result).toEqual({
                    id: null,
                    name: '',
                    cards: [
                        'ShooterNormal00',
                        'BlasterMiddle00',
                        null,
                        null,
                        null,
                        null,
                        null,
                        null,
                        null,
                        null,
                        null,
                        null,
                        null,
                        null,
                        null
                    ]
                });
            });

            it('returns the expected result', () => {
                const result = parseDeck('https://tableturf.koishi.top/?deck=%5B6%2C13%2C22%2C28%2C40%2C34%2C45%2C52%2C55%2C56%2C159%2C137%2C141%2C103%2C92%5D');
                expect(result).toEqual({
                    id: null,
                    name: '',
                    cards: [
                        'ShooterNormal00',
                        'BlasterMiddle00',
                        'RollerNormal00',
                        'ChargerNormal00',
                        'SpinnerStandard00',
                        'SlosherStrong00',
                        'ManeuverNormal00',
                        'StringerNormal00',
                        'SaberLight00',
                        'BombSplash',
                        'Denchinamazu',
                        'TakoDozer',
                        'Shake',
                        'Batoroika',
                        'Mother'
                    ]
                });
            });
        });

        describe('tooltip', () => {
            const errorMessage = 'deckParser.error.unableToParse.tooltip';

            it('behaves as expected if the deck is empty', () => {
                expect.assertions(2);
                try {
                    parseDeck('[]');
                } catch (e: any) {
                    expect(e.translationKey).toEqual(errorMessage);
                    expect(e.cause.message).toEqual('Not a number: ""');
                }
            });

            it('behaves as expected if some card numbers cannot be parsed', () => {
                expect.assertions(2);
                try {
                    parseDeck('[6,13,22,28,40,21,45,52,55,56,abc,137,141,103,92]');
                } catch (e: any) {
                    expect(e.translationKey).toEqual(errorMessage);
                    expect(e.cause.message).toEqual('Not a number: "abc"');
                }
            });

            it('behaves as expected if the deck contains non-existent cards', () => {
                expect.assertions(2);
                try {
                    parseDeck('[6,13,22,28,40,999,45,52,55,56,159,137,141,103,92]');
                } catch (e: any) {
                    expect(e.translationKey).toEqual(errorMessage);
                    expect(e.cause.message).toEqual('Could not find card number 999');
                }
            });

            it('behaves as expected if the deck has too many cards', () => {
                const result = parseDeck('[6,13,13,13,13,13,22,28,40,99,45,52,55,56,159,137,141,103,92,33,44,55,66,77,88,99,11]');
                expect(result).toEqual({
                    id: null,
                    name: '',
                    cards: [
                        'ShooterNormal00',
                        'BlasterMiddle00',
                        'RollerNormal00',
                        'ChargerNormal00',
                        'SpinnerStandard00',
                        'Utsuho',
                        'ManeuverNormal00',
                        'StringerNormal00',
                        'SaberLight00',
                        'BombSplash',
                        'Denchinamazu',
                        'TakoDozer',
                        'Shake',
                        'Batoroika',
                        'Mother'
                    ]
                });
            });

            it('behaves as expected if the deck has less cards than required', () => {
                const result = parseDeck('[6,13,13]');
                expect(result).toEqual({
                    id: null,
                    name: '',
                    cards: [
                        'ShooterNormal00',
                        'BlasterMiddle00',
                        null,
                        null,
                        null,
                        null,
                        null,
                        null,
                        null,
                        null,
                        null,
                        null,
                        null,
                        null,
                        null
                    ]
                });
            });

            it('returns the expected result', () => {
                const result = parseDeck('[6,13,22,28,40,34,45,52,55,56,159,137,141,103,92]');
                expect(result).toEqual({
                    id: null,
                    name: '',
                    cards: [
                        'ShooterNormal00',
                        'BlasterMiddle00',
                        'RollerNormal00',
                        'ChargerNormal00',
                        'SpinnerStandard00',
                        'SlosherStrong00',
                        'ManeuverNormal00',
                        'StringerNormal00',
                        'SaberLight00',
                        'BombSplash',
                        'Denchinamazu',
                        'TakoDozer',
                        'Shake',
                        'Batoroika',
                        'Mother'
                    ]
                });
            });
        });

        describe('andrioCelos', () => {
            const errorMessage = 'deckParser.error.unableToParse.andriocelos';

            it('behaves as expected if the deck is empty', () => {
                expect.assertions(2);
                try {
                    parseDeck('{ "name": "Test Deck", "cards": [] }');
                } catch (e: any) {
                    expect(e.translationKey).toEqual(errorMessage);
                    expect(e.cause.message).toEqual('Given array to convert to card names is empty');
                }
            });

            it('behaves as expected if some card numbers cannot be parsed', () => {
                expect.assertions(2);
                try {
                    parseDeck('{ "name": "Test Deck", "cards": [12, 15, 22, "test", 55] }');
                } catch (e: any) {
                    expect(e.translationKey).toEqual(errorMessage);
                    expect(e.cause.message).toEqual('Could not find card number test');
                }
            });

            it('behaves as expected if the deck contains non-existent cards', () => {
                expect.assertions(2);
                try {
                    parseDeck('{ "name": "Test Deck", "cards": [12, 15, 22, 998, 55] }');
                } catch (e: any) {
                    expect(e.translationKey).toEqual(errorMessage);
                    expect(e.cause.message).toEqual('Could not find card number 998');
                }
            });

            it('behaves as expected if the list of cards is not an array', () => {
                expect.assertions(2);
                try {
                    parseDeck('{ "name": "Test Deck", "cards": "hello! instead of cards there is string" }');
                } catch (e: any) {
                    expect(e.translationKey).toEqual(errorMessage);
                    expect(e.cause.message).toEqual('Bad input: Expected an array of card numbers, got string');
                }
            });

            it('behaves as expected if the list of cards is missing', () => {
                expect.assertions(2);
                try {
                    parseDeck('{ "name": "Test Deck" }');
                } catch (e: any) {
                    expect(e.translationKey).toEqual(errorMessage);
                    expect(e.cause.message).toEqual('Bad input: Expected an array of card numbers, got undefined');
                }
            });

            it('behaves as expected if the deck has too many cards', () => {
                const result = parseDeck('{ "name": "Test Deck", "cards": [12, 12, 12, 15, 22, 11, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71] }');
                expect(result).toEqual({
                    id: null,
                    name: 'Test Deck',
                    cards: [
                        'BlasterShort00',
                        'BlasterLightShort00',
                        'RollerNormal00',
                        'ShooterLong00',
                        'SaberLight00',
                        'BombSplash',
                        'BombSuction',
                        'BombQuick',
                        'Sprinkler',
                        'Shield',
                        'BombFizzy',
                        'BombCurling',
                        'BombRobot',
                        'Beacon',
                        'PointSensor'
                    ]
                });
            });

            it('behaves as expected if the deck has less cards than required', () => {
                const result = parseDeck('{ "name": "Test Deck", "cards": [12, 12, 15, 22] }');
                expect(result).toEqual({
                    id: null,
                    name: 'Test Deck',
                    cards: [
                        'BlasterShort00',
                        'BlasterLightShort00',
                        'RollerNormal00',
                        null,
                        null,
                        null,
                        null,
                        null,
                        null,
                        null,
                        null,
                        null,
                        null,
                        null,
                        null
                    ]
                });
            });

            it('behaves as expected if the name of the deck is not a string', () => {
                const result = parseDeck('{ "name": { "mystery": "i have deceived you! this deck name is an object" }, "cards": [12, 12, 15, 22] }');
                expect(result.name).toEqual('');
            });

            it('returns the expected result', () => {
                const result = parseDeck('{\n\t"name": "Starter Deck",\n\t"cards": [\n\t\t6,\n\t\t34,\n\t\t159,\n\t\t13,\n\t\t45,\n\t\t137,\n\t\t22,\n\t\t52,\n\t\t141,\n\t\t28,\n\t\t55,\n\t\t103,\n\t\t40,\n\t\t56,\n\t\t92\n\t]\n}');
                expect(result).toEqual({
                    id: null,
                    name: 'Starter Deck',
                    cards: [
                        'ShooterNormal00',
                        'SlosherStrong00',
                        'Denchinamazu',
                        'BlasterMiddle00',
                        'ManeuverNormal00',
                        'TakoDozer',
                        'RollerNormal00',
                        'StringerNormal00',
                        'Shake',
                        'ChargerNormal00',
                        'SaberLight00',
                        'Batoroika',
                        'SpinnerStandard00',
                        'BombSplash',
                        'Mother'
                    ]
                });
            });
        });
    });
});
