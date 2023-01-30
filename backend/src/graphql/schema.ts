import { userResolvers } from './user/resolvers';
import { userTypeDefs } from './user/typedefs';
import { gql } from "apollo-server";

const rootTypeDefs = gql`
    type Query {
        _empty: Boolean
    }
`

const rootResolvers = {
    Query: {
        _empty: () => true
    }
}

export const typeDefs = [rootTypeDefs, userTypeDefs]
export const resolvers = [rootResolvers, userResolvers]