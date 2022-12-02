import Cards from '~/assets/cards.json';
import { Card } from '~/types/Card';

export const CardMap: Map<string, Card> = new Map();

for (let i = 0; i < Cards.length; i++) {
    const card = Cards[i];
    CardMap.set(card.name, Object.freeze(card));
}
