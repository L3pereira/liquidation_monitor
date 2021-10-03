# liquidation_monitor

Backend is built with rustc 1.53.0 (53cb7b09b 2021-06-17);

Change the config.json

"server_address": "Your IP",
"server_port": "An available port"


Logs are stored in logs folder by level
you can choose the logs level (ex warn or error) in log_config.yaml

websocket client (wss://observer.terra.dev) is in tokio tungstenite.
websocket server is in Actix.
Runtime Tokio.

How to build frontend:
install nodejs (v16.7.0) and npm (v7.20.3)

npm init -y
npm install --save-dev webpack@5.56.0 webpack-cli
npm install --save-dev html-loader
npm install --save-dev css-loader
npm install --save-dev string-replace-loader
npm install --save-dev clean-webpack-plugin
npm install --save-dev html-webpack-plugin
npm install --save-dev mini-css-extract-plugin

npm run build

Frontend assets are in www/src and built to www/dist

Program flow

main >--call--> 
stream_init_task >--call--> 
reader_task (tokio tungstenite) >--channel-->
stream_management_task >--call--> 
websocket_msg_process >--call--> 
deserialize_stream >--return via channel --> 
stream_init_task  >-- Actix Actor Broker--> 
Actix Websocket--> Frontend (ws://<YOUR IP>:<PORT> in config.json)