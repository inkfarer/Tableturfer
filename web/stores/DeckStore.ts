import { defineStore } from 'pinia';
import { useRoomStore } from '~/stores/RoomStore';
import { PlayerTeam } from '~/types/PlayerTeam';

interface DeckStore {
    availableCards: string[]
    usedCards: { [key in PlayerTeam]: Set<string> }
}

export const useDeckStore = defineStore('deck', {
    state: (): DeckStore => ({
        availableCards: [],
        // todo: fill me
        usedCards: {
            [PlayerTeam.ALPHA]: new Set(),
            [PlayerTeam.BRAVO]: new Set()
        }
    }),
    getters: {
        deck() {
            const roomStore = useRoomStore();
            if (roomStore.id == null) {
                return null;
            }

            return roomStore.users[roomStore.id]?.deck;
        },
        opponentDeck() {
            const roomStore = useRoomStore();
            if (roomStore.opponent == null) {
                return null;
            }

            return roomStore.users[roomStore.opponent]?.deck;
        }
    },
    actions: {
        replaceCard(oldCard: string, newCard: string) {
            // todo: what to do in case the server and the client disagree on what their cards are
            const cardIndex = this.availableCards.findIndex(card => card === oldCard);
            this.availableCards.splice(cardIndex, 1, newCard);
        }
    }
});
