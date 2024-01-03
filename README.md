# Twitch Sub Vod Resolver

Be able to watch any sub-only Twitch vod

# Deployment

```bash
docker build . -t tsv-resolver && docker run -dp 8080:8080 tsv-resolver
```

# Instruction

1. Pick video only for subscribers.
   ![Step 1](./assets/1.jpg)

2. Just copy link from search line
   ![Step 2](./assets/2.jpg)
3. Go to localhost:8080

4. Paste it in input
   ![Step 4](./assets/4.jpg)

5. Click button "Find vod"
   ![Step 5](./assets/5.jpg)

6. Watch your favourite streamers without sub
   ![Step 6](./assets/6.png)
