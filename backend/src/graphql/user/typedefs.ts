import { gql } from "apollo-server";

export const userTypeDefs = gql`
    extend type Query {
        user: User
        users: [User!]!
    }

    type User {
        id: ID!
        username: String!
    }
`