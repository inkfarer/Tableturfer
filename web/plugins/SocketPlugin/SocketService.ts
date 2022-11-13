import { AnySocketMessage, SocketActionMap } from '~/types/Socket';
import { useRoomStore } from '~/stores/RoomStore';

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
                this.ws?.removeEventListener('message', welcomeListener);
                reject(e);
            };

            const welcomeListener = (msg: MessageEvent) => {
                const parsed = this.parseSocketMessage(msg.data);

                if (parsed != null && parsed.event === 'Welcome') {
                    this.ws?.removeEventListener('close', closeListener);
                    resolve(parsed.detail.roomCode);
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
            useRoomStore().roomCode = null;

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
                useRoomStore().roomCode = msg.detail.roomCode;
                break;
            default:
                console.log(`Unhandled event '${msg.event}'`, msg.detail);
        }
    }

    send<K extends keyof SocketActionMap, D = SocketActionMap[K]>(action: K, args: D): void {
        if (this.ws == null || this.ws.readyState !== WebSocket.OPEN) {
            throw new Error('Websocket is not open, cannot send message');
        }

        this.ws.send(JSON.stringify({ action, args }));
    }
}
