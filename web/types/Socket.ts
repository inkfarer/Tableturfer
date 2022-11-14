export interface SocketActionMap {
    CreateRoom: never
}

export interface SocketUser {
    joinedAt: string
}

interface SocketMessageMap {
    Error: string
    Welcome: { roomCode: string, users: Record<string, SocketUser> }
    UserJoin: { id: string, user: SocketUser }
    UserLeave: string
}

type IncomingSocketMessageMap = {
    [K in keyof SocketMessageMap]: { event: K, detail: SocketMessageMap[K] }
};
export type AnySocketMessage = IncomingSocketMessageMap[keyof IncomingSocketMessageMap];
