const user = () => {
    return {
        id: '1',
        username: 'Gabriel'
    }
}

const users = () => {
    return [
        {
            id: '2',
            username: 'Gabriel2'
        },
        {
            id: '3',
            username: 'Gabriel3'
        },
        {
            id: '4',
            username: 'Gabriel4'
        },
        {
            id: '5',
            username: 'Gabriel5'
        },
    ]
}


export const userResolvers = {
    Query: {
        user,
        users,
    },
}

