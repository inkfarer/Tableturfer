export enum PlayerTeam {
    ALPHA = 'Alpha',
    BRAVO = 'Bravo'
}

export type TeamMap<T> = { [team in PlayerTeam]: T };
