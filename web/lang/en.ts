import ENTranslations from '../lang/game/en.json';

export default {
    common: {
        card: 'Card | Cards'
    },
    nav: {
        siteName: 'tblturf.ink',
        decks: 'Decks'
    },
    index: {
        title: 'Welcome to tblturf.ink!',
        subtitle: 'The Tableturf Battle simulator.',
        wipWarning: 'A work in progress!'
    },
    roomJoiner: {
        newRoom: 'Create a new room',
        joinRoom: 'Join room',
        username: 'Name'
    },
    roomLinkCopier: {
        title: 'Copy link to room'
    },
    room: {
        title: {
            loading: 'Loading...',
            joinedRoomCode: 'Room {code}'
        },
        loading: 'Loading...',
        beforeRoomCode: 'Joined room',
        errorOccurred: 'An error has occurred.',
        backHome: 'Back to main page',
        mapName: 'Map',
        deckName: 'Deck',
        setMap: 'Change map',
        setDeck: 'Change deck',
        startGame: 'Start the game!',
        leave: 'Leave room',
        howToPlay: 'How to play',
        userSettings: 'User settings',
        userListTitle: 'Users',
        userRole: {
            owner: 'Room Owner',
            opponent: 'Opponent',
            spectator: 'Spectator'
        },
        userError: {
            deckNotChosen: 'Deck not chosen!'
        },
        deckSelect: {
            title: 'Choose a deck'
        },
        noUsername: {
            title: 'Please choose an username.',
            continue: 'Join room'
        }
    },
    game: {
        pass: 'Pass',
        special: 'Special ON!',
        placeCard: 'Place card',
        turnsRemaining: 'Turns left',
        returnToRoom: 'Back to room',
        waitingToReturn: 'Waiting for room owner to return...',
        error: {
            communicationError: 'A communication error has occurred.',
            backHome: 'Back to main page'
        },
        redrawMessage: {
            title: 'Redraw this hand?',
            mobileCardPreviewHint: 'Hint: Tap and hold on a card to view more details.',
            decline: 'Keep',
            confirm: 'Redraw!'
        },
        map: {
            unknown: '???',
            random: 'Random Map',
            ...ENTranslations.map
        },
        leavingOverlay: {
            title: 'Are you sure you\'d like to leave?',
            confirm: 'Leave',
            cancel: 'Cancel'
        },
        card: ENTranslations.card
    },
    deckList: {
        title: 'Decks',
        header: 'Decks',
        loading: 'Loading...',
        localStorageMessage: 'For the time being, your decks will be stored inside your browser. Please note that because of this, clearing site data for tblturf.ink may erase your saved decks.',
        createNew: 'Create a new deck',
        import: 'Import a deck',
        deckAction: {
            remove: 'Delete',
            edit: 'Edit',
            rename: 'Rename',
            copy: 'Copy'
        },
        renameOverlay: {
            deckName: 'Deck name',
            save: 'Save'
        }
    },
    deckName: {
        defaultDeck: 'Default Deck',
        defaultName: 'New Deck',
        copyName: '{name} (Copy)'
    },
    deckEditor: {
        title: {
            new: 'Create a deck',
            editing: {
                default: 'Editing deck',
                withName: 'Editing deck "{name}"'
            }
        },
        deckName: 'Deck name',
        squareCount: 'Total card squares',
        back: 'Go back',
        save: 'Save changes'
    },
    deckImporter: {
        title: 'Import a deck',
        sourceList: {
            title: 'Where can I import from?',
            tooltip: 'Tableturf Tooltip',
            koishi: {
                name: 'Koishi\'s Tableturf Replica',
                note: '(Select "Copy Link" when sharing a deck.)'
            },
            andriocelos: 'Andrio Celos\' Tableturf Battle'
        },
        dataInputLabel: 'Deck details',
        submit: 'Import',
        importSuccess: 'Successfully imported deck with {n} @.lower:common.card',
        defaultDeckName: 'Imported deck'
    },
    deckParser: {
        error: {
            noInput: 'Received nothing to import.',
            unknownInput: 'Couldn\'t parse the given input.',
            unableToParse: {
                tooltip: 'This looks like it came from Tooltip, but it couldn\'t be parsed.',
                koishi: 'This looks like it came from Koishi\'s Replica, but it couldn\'t be parsed.',
                andriocelos: 'This looks like it came from Andrio Celos\' simulator, but it couldn\'t be parsed.'
            },
            unknown: 'An error has occurred.'
        }
    },
    footer: {
        createdBy: 'tblturf.ink is created by {0}.',
        appInfo: {
            name: 'Tableturfer',
            template: '{name} {details}',
            buildInfo: 'commit {commit} built on {date}',
            devBuild: 'development build!'
        }
    },
    sandbox: {
        wipAlert: 'You\'ve reached the Sandbox! It\'s currently here as a quick and dirty way to develop the gameplay views, but will be turned into a fully fledged feature at a later date.'
    },
    howToPlay: {
        mainHeading: 'How to Play',
        intro: 'In brief, use your cards to cover more area than your opponent. More details are available on the {wiki}',
        wikiLinkText: 'Splatoon wiki.',
        controls: {
            heading: 'Controls',
            mouseDescription: 'Using a mouse, move the selected card by moving your mouse around the game board. Rotate your card by right clicking and confirm your move by left clicking.',
            touchDescription: 'On a touchscreen device, drag your finger across the game board to move the selected card. The card can be rotated and the move confirmed by the buttons below the board.'
        },
        dismiss: 'Dismiss'
    },
    userSettings: {
        title: 'User Settings',
        subtitle: 'Change how your game behaves.',
        onScreenMovementControls: 'Enable on-screen card movement keys',
        onScreenRotationAndPlacementControls: 'Enable on-screen keys for placing and rotating cards',
        flipBoardOnBravoTeam: 'Flip the board while playing as the Bravo (Opponent) team'
    },
    title: {
        default: 'tblturf.ink',
        template: '{page} {\'|\'} tblturf.ink',
        devBuildMarker: '[DEV]'
    }
};
