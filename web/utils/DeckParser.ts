import { NewDeck } from '~/types/DeckList';
import { isBlank } from '~/helpers/StringHelper';
import { TranslatableError } from '~/utils/TranslatableError';
import { CardMap } from '~/helpers/Cards';
import { DECK_SIZE } from '~/data/Constants';

export function parseDeck(deck: string): NewDeck {
    if (isBlank(deck)) {
        throw new TranslatableError('deckParser.error.noInput');
    }

    deck = deck.trim();
    let result: NewDeck;

    if (deck.startsWith('http')) {
        result = parseKoishi(deck);
    } else if (deck.startsWith('[') && deck.endsWith(']')) {
        result = parseTooltip(deck);
    } else if (deck.startsWith('{') && deck.endsWith('}')) {
        result = parseAndrioCelos(deck);
    } else {
        throw new TranslatableError('deckParser.error.unknownInput');
    }

    return {
        ...result,
        cards: normalizeCardList(result.cards)
    };
}

function normalizeCardList<T>(deck: T[]): T[] {
    const deduplicatedDeck = Array.from(new Set(deck).values());

    if (deduplicatedDeck.length === DECK_SIZE) {
        return deduplicatedDeck;
    } else if (deduplicatedDeck.length < DECK_SIZE) {
        return [
            ...deduplicatedDeck,
            ...new Array(DECK_SIZE - deduplicatedDeck.length).fill(null)
        ];
    } else {
        return deduplicatedDeck.slice(0, DECK_SIZE);
    }
}

function parseKoishi(deck: string): NewDeck {
    try {
        const parsedUrl = new URL(deck);
        const numberArray = decodeURIComponent(parsedUrl.searchParams.get('deck') ?? '');
        return {
            id: null,
            name: '',
            cards: cardNumbersToCardNames(parseNumberArray(numberArray))
        };
    } catch (e) {
        throw new TranslatableError('deckParser.error.unableToParse.koishi', e);
    }
}

function parseTooltip(deck: string): NewDeck {
    try {
        return {
            id: null,
            name: '',
            cards: cardNumbersToCardNames(parseNumberArray(deck).map(number => number + 1))
        };
    } catch (e) {
        throw new TranslatableError('deckParser.error.unableToParse.tooltip', e);
    }
}

function parseNumberArray(numberArray: string): number[] {
    const splitInput = numberArray.trim().slice(1, -1).split(',');

    return splitInput.map(number => {
        const parsedNumber = parseInt(number);
        if (isNaN(parsedNumber)) {
            throw new Error(`Not a number: "${number}"`);
        }
        return parsedNumber;
    });
}

function cardNumbersToCardNames(cardNumbers: unknown[]): string[] {
    if (!Array.isArray(cardNumbers)) {
        throw new Error(`Bad input: Expected an array of card numbers, got ${typeof cardNumbers}`);
    }
    if (cardNumbers.length === 0) {
        throw new Error('Given array to convert to card names is empty');
    }

    const cardList = Array.from(CardMap.values());
    return cardNumbers.map(cardNumber => {
        const cardName = cardList.find(card => card.number === cardNumber)?.name;
        if (cardName == null) {
            throw new Error(`Could not find card number ${cardNumber}`);
        }
        return cardName;
    });
}

function parseAndrioCelos(deck: string): NewDeck {
    try {
        const parsedDeck = JSON.parse(deck);
        return {
            id: null,
            name: typeof parsedDeck.name === 'string' ? parsedDeck.name : '',
            cards: cardNumbersToCardNames(parsedDeck.cards)
        };
    } catch (e) {
        throw new TranslatableError('deckParser.error.unableToParse.andriocelos', e);
    }
}
