PushIt
======

This is a kube controller that listens for changes to deployments in any namespace, 
when it detects a change, it will serve a song of your choice to your phone via
the twilio api. I use Salt-N-Pepa's "Push It!" but you can use whatever you like.

Please don't actually use this unless you really want to.

P.S. Push it!

# Configuration

The following enviornment variables are required to be set:

1. `TWILIO_ACCOUNT_SID`
2. `TWILIO_API_KEY`
3. `TWILIO_API_KEY_SECRET`
4. `TWILIO_PHONE_NUMBER`
5. `APPLICATION_BASE_URL` (example: `https://some-url.ngrok.io`)
6. `TO_NUMBER` (NOTE: This should be the twilio formatted number that you want to call when a deploy happens)
7. `SONG_FILE_PATH` (NOTE: This should be relative, example: `static/my-song.mp3`)

The twilio callback URL should be:

```
$APPLICATION_BASE_URL/twilio/call/callback
```

# License

MIT
