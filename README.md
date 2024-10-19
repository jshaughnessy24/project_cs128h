# CLIChat
Created by: 
Miguel Aenlle (maenlle2),
Jenny Shaughnessy (jennys4), and
Zia Lu (zixuan43)

## Project Introduction
We plan to build a simpler version of Discord that runs in the Command Line.
Our minimum viable project is a chat app that allows users to create and sign into accounts, add other users as friends, direct message other users, and message other users in group chats. 
We chose this project because chat apps are widespread and learning to create one using rust is an interesting application. We are also interested in using databases and enabling real time communication.

## Technical Overview
CLIChat has a few key functionalities: Authentication (Creating Accounts and Signing In), Direct Messages, and Group Chats. We will develop this using Rust and MongoDB. Here is a diagram of the actions users will be able to take:

The data for users, messages, and group chats will be stored in MongoDB. Below is a diagram of the data schema:

Here are all the functions we will develop to manage this:
- Authentication
    - Sign Up 
    - Log In
- Friends
    - Add Friend
    - Remove Friend
    - Direct Message
- Group Chats
    - Create Group Chat
    - Message

Here are designs for each of the pages in the Command Line Interface:

What we plan to have finished by each checkpoint:
- 11/4 Checkpoint 1:
    - Sign-up and login functionality with MongoDB
        - Functions to add and read users using MongoDB
        - CLI UI to sign up / login
    - Friend functionality with MongoDB
        - Functions to add and read friends using MongoDB
        - CLI UI for adding friends
- 11/18 Checkpoint 2:
    - DM functionality with MongoDB
        - Functions to add and read messages to DMs using MongoDB
        - CLI UI for writing and reading messages to DMs
- 12/11 Final Submission: 
    - Group chat functionality with MongoDB
        - Functions to add and read messages to group chats using MongoDB
        - CLI UI for writing and reading messages to group chats

## Possible Challenges
- Using a MongoDB listener to read messages
- Creating a sign-on system using MongoDB

## References
- Discord
- Slack
- Inspiration:
    - https://github.com/nag763/tchatchers
    - https://github.com/tinrab/rusty-chat
    - https://github.com/LennyBoyatzis/rust-chat
- MongoDB:
    - https://www.mongodb.com/docs/drivers/rust/current/
    - https://www.mongodb.com/docs/drivers/node/current/usage-examples/changeStream/

