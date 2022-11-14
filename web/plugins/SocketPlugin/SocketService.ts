import { AnySocketMessage, SocketActionMap } from '~/types/socket/SocketEvent';
import { useRoomStore } from '~/stores/RoomStore';
import { AnyRoomEvent } from '~/types/socket/RoomEvent';

export class SocketService {
    private ws: WebSocket | null;
    private readonly url: string;

    constructor(url: string) {
        this.ws = null;
        this.url = url;
    }

    async connect(roomCode?: string): Promise<string> {
        if (this.ws != null && (this.ws.readyState === WebSocket.OPEN || this.ws.readyState === WebSocket.CONNECTING)) {
            throw new Error('Websocket is already open');
        }

        return new Promise((resolve, reject) => {
            try {
                this.ws = new WebSocket(roomCode == null ? this.url : `${this.url}?room=${encodeURIComponent(roomCode)}`);
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

            if (e.code >= 4000 && e.code < 5000) {
                console.error('Websocket closed with message:', e.reason);
            }
        });

        this.ws.addEventListener('error', e => {
            console.error('ws error', e);
        });
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
                break;
            case 'RoomEvent':
                this.handleRoomEvent(msg.detail);
                break;
            default:
                console.log(`Unhandled event '${msg.event}'`, msg.detail);
        }
    }

    private handleRoomEvent(event: AnyRoomEvent) {
        switch (event.event) {
            case 'UserJoin':
                useRoomStore().addUser(event.detail.id, event.detail.user);
                break;
            case 'UserLeave':
                useRoomStore().removeUser(event.detail);
                break;
            case 'OwnerChange':
                useRoomStore().owner = event.detail;
                break;
        }
    }

    send<K extends keyof SocketActionMap, D = SocketActionMap[K]>(action: K, args: D): void {
        if (this.ws == null || this.ws.readyState !== WebSocket.OPEN) {
            throw new Error('Websocket is not open, cannot send message');
        }

        this.ws.send(JSON.stringify({ action, args }));
    }
}