User Preferences Profiling - Pitch
=====

## Motivation
As we’re moving to our second monetisation strategy - Audio Advertisement - we need to be able to know more about our listeners. 

So that we can target the users with the native Audio Advertisements better.

For our advertisers we need to answer following questions: what topics our users are listening to? How many users listened to articles about cars this month? What are the users that are listening about summer vacation? 

## Solution

### Article-Level

Since we’re analysing the texts for **Topics Classifier**, we should store additional information about the articles.

* What are the keywords? 
* What topic clusters can be built? 
* Show me all the articles about cars. 
* Show me all the articles about summer vacation. 
* Show me all articles of type **Interview** that talk about **Personal Finances.**

### User-Level

We also need to build user profile clusters - so that we can address those users directly. Show me all the users that listened to articles about cars last month. Show me all the users that listened to **interviews** that contained “summer vacation”, but did not contain “planes”. Show me all the users interested in “dogs”, but not in “horses”.
