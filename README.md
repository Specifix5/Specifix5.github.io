# [specifix.dev](https://specifix.dev)
My personal readme website type of thing \
will also used to show you the stuff im working on.
  
# credits
* [iconify](https://iconify.design)
* [twemoji](https://github.com/twitter/twemoji)
* [moe-counter by journey-ad](https://github.com/Specifix5/Moe-Counter)
* [tsparticles](https://github.com/tsparticles)

# installation
### Astro
Run ``pnpm i`` to download the requirements for Astro
Follow the instructions if it tells you to do anything further.

### API Backend (Rust)
* Get yourself a copy of maxminddb [GeoLite2-City](https://www.maxmind.com/en/geoip-databases) database
* Put it in ``api_backend/static/`` folder
* Set up an ``.env`` file in the ``api_backend/`` directory. 
### .env example
```.env
AVATAR_URL="IMAGE LINK HERE"
LOGGING_URL="IP REQUESTS DISCORD WEBHOOK LINK"
#Right now, only /ip requests gets logged into the webhook (All others are still sent through stdout)
MAILBOX_URL="MAILBOX DISCORD WEBHOOK LINK"
#Where all the mails from /mailbox get sent
WHOIS=false
#Performs WHOIS queries on the IP, may be subject to whois rate limit.
```
* finally, run ``cargo run`` and your done :3

# visitors
![visitors](https://count.specifix.dev/get/@visitors?theme=moebooru)  
