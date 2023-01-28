# anonymous_bot
A discord bot to talk anonymously on a guild channel.


You need to set two environment variable for it to work:

  ```DISCORD_TOKEN``` : The discord token of your bot.

  ```CHANNEL_ID``` : The channel ID of the channel where the bot will post the messages.

## Commands :

**```!message <message>```**

\<message\> : The message you want to send. You can also attach files.

*Example : ```!message test```*

---

**```!newvote "<question>" "<vote 1>" "<vote 2>" "<vote n>"```**

\<question\> : The question for the vote.

\<vote\> : A possible answer for the vote. You can have between 2 and 5 possible answers.

*Example : ```!newvote "what is your favorite color ?" "red" "blue" "green"```*

---


**```!vote <answer>```**

\<answer\> : Your answer. The first possible answer is 0.

*Example : ```!vote 2```*