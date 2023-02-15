import Maps from '~/assets/maps.json';
import { GameMap } from '~/types/GameMap';

export const RANDOM_MAP_NAME = 'random';

export const GameMapMap: Map<string, GameMap> = new Map();

for (let i = 0; i < Maps.length; i++) {
    const map = Maps[i];
    GameMapMap.set(map.name, Object.freeze(map));
}
