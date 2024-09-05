# tlk
> A blockchain-based decentralized chat application based on public key cryptography
## Creating a user
- When a user creates an account, a public-private key pair is generated.
- All future messages from that user are signed with their private key, and each message is attached with a struct associating the public key with the user's username.
- This allows identification without the need for passwords - before the creation of a new user, the username is checked in lists of all existing usernames, and if there is a clash, the user is prompted to choose a new username.
### Potential (non) issue of account verification
- Though hypothetically with a "hacked" client, a user could post messages under the username of another, since the public key is associated with the username, the user would not be able to sign the message with the private key of the user they are impersonating.
- The client checks not the public key associated with that particular message, but rather the public key associated with the very first message sent by that user. This is to prevent a user from changing their public key and impersonating another user.
- If such a dispute were to arise, the impersonater would be able to send messages, but since they were signed with the wrong private key, as the public key is taken from the first message, the client would not be able to verify the message, and would output garbage.
## Message chains
- Each chain can be thought of like a "chat" - a user can create a chain, and invite other IP addresses to join the chain, by sending them the initial message of a chain, which will contain a genesis block signed by the creating user.
- Once a user has joined a chain, they can post messages to the chain, and all other users in the chain will be able to see the message.
- When one user is online, logs are periodically saved to their local storage
- When they log on, the first message they recieve will be the current, updated, status of the chain.
- By confirming that 
> a. all previous messages correspond to the messages they have stored locally
> b. no messages have been sent with the same user and/or public key that they have stored locally
- this ensures that the chain has not tampered with, as no previous messages could have been altered unknowingly with a deep-rehashing attack.
- by confirming on a user by user basis, as each user logs on to confirm the chain, if one of them rejects as a message has been impossibly attributed to them, the chain is considered compromised, and all users are notified, and the default client prevents any further messages from being sent, though of course this could be bypassed by a malicious user with an aforementioned "hacked" client.