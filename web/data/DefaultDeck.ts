import { Deck } from '~/types/DeckList';
import { Composer } from 'vue-i18n';

export const DEFAULT_DECK_ID = 'default';

export const DEFAULT_DECK_CARDS = [
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
];

export function createDefaultDeck(i18n: Composer): Deck {
    return Object.freeze({
        id: DEFAULT_DECK_ID,
        name: i18n.t('deckName.defaultDeck'),
        cards: DEFAULT_DECK_CARDS
    });
}
