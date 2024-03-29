import { AnySocketMessage } from '~/types/socket/SocketEvent';
import { useRoomStore } from '~/stores/RoomStore';
import { AnyRoomEvent } from '~/types/socket/RoomEvent';
import { SocketActionMap } from '~/types/socket/SocketAction';
import { useGameBoardStore } from '~/stores/GameBoardStore';
import { useDeckStore } from '~/stores/DeckStore';
import { navigateTo } from '#imports';
import { useCurrentMoveStore } from '~/stores/CurrentMoveStore';
import { useMoveStore } from '~/stores/MoveStore';

export class SocketService {
    private wsPingInterval: number | undefined;
    private ws: WebSocket | null;
    private readonly url: string;

    constructor(url: string) {
        this.ws = null;
        this.url = url;
    }

    isOpen(): boolean {
        return this.ws != null && (this.ws?.readyState === WebSocket.OPEN || this.ws?.readyState === WebSocket.CONNECTING);
    }

    async connect(roomCode: string | undefined, username: string): Promise<string> {
        if (this.isOpen()) {
            throw new Error('Websocket is already open');
        }

        return new Promise((resolve, reject) => {
            try {
                const url = new URL(this.url);
                if (roomCode != null) {
                    url.searchParams.set('room', roomCode);
                }
                url.searchParams.set('username', username);

                this.ws = new WebSocket(url.toString());
            } catch (e) {
                return reject(e);
            }

            const closeListener = (e: CloseEvent) => {
                removeListeners();
                reject(e);
            };

            const welcomeListener = (msg: MessageEvent) => {
                const parsed = this.parseSocketMessage(msg.data);

                if (parsed != null && parsed.event === 'Welcome') {
                    removeListeners();
                    resolve(parsed.detail.roomCode);
                }
            };

            const removeListeners = () => {
                if (this.ws != null) {
                    this.ws.removeEventListener('close', closeListener);
                    this.ws.removeEventListener('message', welcomeListener);
                }
            };

            this.ws.addEventListener('close', closeListener);
            this.ws.addEventListener('message', welcomeListener);

            this.initSocket();
        });
    }

    disconnect() {
        this.ws?.close();
    }

    private initSocket() {
        if (this.ws == null) {
            throw new Error('Tried to initialize websocket before creating a connection');
        }

        this.ws.addEventListener('message', msg => {
            const parsed = this.parseSocketMessage(msg.data);

            if (parsed != null) {
                this.handleSocketMessage(parsed);
            }
        });

        this.ws.addEventListener('close', e => {
            useRoomStore().leaveRoom();
            useDeckStore().$reset();
            window.clearInterval(this.wsPingInterval);

            if (e.code >= 4000 && e.code < 5000) {
                console.error('Websocket closed with message:', e.reason);
            }
        });

        this.ws.addEventListener('error', e => {
            console.error('ws error', e);
        });

        // Ping the server to keep the websocket connection alive
        window.clearInterval(this.wsPingInterval);
        this.wsPingInterval = window.setInterval(() => {
            this.send('Ping');
        }, 45 * 1000);
    }

    private parseSocketMessage(msg: string): AnySocketMessage | null {
        try {
            return JSON.parse(msg);
        } catch (e) {
            console.error('Could not parse message from websocket:', e);
            return null;
        }
    }

    private handleSocketMessage(msg: AnySocketMessage) {
        switch (msg.event) {
            case 'Welcome':
                useRoomStore().joinRoom(msg.detail);
                useGameBoardStore().setBoardByName(msg.detail.map);
                break;
            case 'RoomEvent':
                this.handleRoomEvent(msg.detail);
                break;
            case 'Error':
                if (msg.detail.detail != null) {
                    console.error(`Received error "${msg.detail.code}":`, msg.detail.detail);
                } else {
                    console.error(`Received error "${msg.detail.code}"`);
                }

                if (msg.detail.code === 'GameError' && msg.detail.detail.code === 'InvalidMove') {
                    useCurrentMoveStore().locked = false;
                }

                break;
            case 'Pong':
                // Do nothing for now
                break;
        }
    }

    private async handleRoomEvent(event: AnyRoomEvent) {
        switch (event.event) {
            case 'UserJoin':
            case 'UserUpdate':
                useRoomStore().upsertUser(event.detail.id, event.detail.user);
                break;
            case 'UserLeave':
                useRoomStore().removeUser(event.detail);
                break;
            case 'OwnerChange':
                useRoomStore().owner = event.detail;
                break;
            case 'MapChange':
                useGameBoardStore().setBoardByName(event.detail);
                break;
            case 'OpponentChange':
                useRoomStore().opponent = event.detail;
                break;
            case 'StartGame': {
                useCurrentMoveStore().resetGame();
                // The client can receive 'HandAssigned' before 'StartGame', so here we must be careful not to erase
                // what cards were just given to them
                useDeckStore().resetUsedCards();
                useGameBoardStore().resetBoard();
                useMoveStore().resetMoves();
                useRoomStore().resetGame();

                useGameBoardStore().setBoardByName(event.detail.mapName, false);
                const roomStore = useRoomStore();
                roomStore.started = true;
                roomStore.score = event.detail.score;
                break;
            }
            case 'MoveReceived':
                // It is likely that the client may receive 'MoveReceived' and 'MovesApplied' out of order, so
                // we send over which turn the move was for to prevent incorrect state
                if (useRoomStore().remainingTurns === event.detail.remainingTurns) {
                    useMoveStore().nextMoveCompleted[event.detail.team] = true;
                }
                break;
            case 'MovesApplied':
                useMoveStore().applyMoves(event.detail.moves);
                useRoomStore().score = event.detail.score;
                break;
            case 'HandAssigned':
                useDeckStore().availableCards = event.detail;
                break;
            case 'NextCardDrawn':
                useDeckStore().replaceCard(event.detail.replacing, event.detail.newCard);
                break;
            case 'ReturnToRoom': {
                const roomStore = useRoomStore();
                await navigateTo(`/room/${roomStore.roomCode}`);
                useCurrentMoveStore().resetGame();
                useDeckStore().resetGame();
                useGameBoardStore().resetBoard();
                useMoveStore().resetMoves();
                roomStore.resetGame();
                break;
            }
            case 'EndGame': {
                const roomStore = useRoomStore();
                roomStore.completed = true;
                roomStore.score = event.detail.score;
                const currentMoveStore = useCurrentMoveStore();
                currentMoveStore.locked = true;
                break;
            }
        }
    }

    send<K extends keyof SocketActionMap, D = SocketActionMap[K]>(action: K, args?: D): void {
        if (this.ws == null || this.ws.readyState !== WebSocket.OPEN) {
            throw new Error('Websocket is not open, cannot send message');
        }

        this.ws.send(JSON.stringify({ action, args }));
    }
}
