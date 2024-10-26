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

<img width="705" alt="Screenshot 2024-10-19 at 11 51 59 AM" src="https://github.com/user-attachments/assets/8d84bdbe-04cf-49c0-97cd-c7565d1dc762">

The data for users, messages, and group chats will be stored in MongoDB. Below is a diagram of the data schema:

<img width="492" alt="Screenshot 2024-10-19 at 11 30 52 AM" src="https://github.com/user-attachments/assets/cc45d4f1-5c09-4890-9b0b-1c06203f2426">

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

![Group 1 (1)](https://github.com/user-attachments/assets/e0569630-7a99-4980-8c4f-a0c4a108beea)
![Group 2](https://github.com/user-attachments/assets/b82de6b0-c522-4427-89b6-1a28f33d2f8f)
![Group 3](https://github.com/user-attachments/assets/e600f23a-49f9-4c26-bbca-f77aabb01ff5)
![Group 4](https://github.com/user-attachments/assets/9b5633fe-79d1-4fb5-ba60-654e7361ae8d)
![Group 5](https://github.com/user-attachments/assets/1194f5c4-7928-4dd6-ac36-f832ef7b3c73)
![Group 6](https://github.com/user-attachments/assets/ee3f57b9-6ca8-49c8-b51e-63390c29eb38)
![Group 7](https://github.com/user-attachments/assets/7bbdc18c-3382-4bff-8967-5400a799315c)

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
- Edge cases for adding/removing users to/from groups

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

