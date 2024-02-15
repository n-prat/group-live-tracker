# Group Live tracker

Server-less (ie a static html) site displaying the live location of all users connected to a room.
FAIL: not really possible using WebRTC. It is only meant for P2P, and can not scale to "chat room" level with 50-100 peers.

No database, no history, no storing, etc

Credits: https://chat.openai.com
Based on https://github.com/codec-abc/Yew-WebRTC-Chat

## DEV/local test

- https://github.com/trunk-rs/trunk?tab=readme-ov-file#getting-started
- `trunk serve`