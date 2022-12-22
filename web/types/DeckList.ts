export interface NewDeck {
    id: string | null
    name: string
    cards: string[]
}

export interface Deck {
    id: string
    name: string
    cards: string[]
}
