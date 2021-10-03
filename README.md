# liquidation_monitor

Backend is built with rustc 1.53.0 (53cb7b09b 2021-06-17);<br/>
Change the config.json

"server_address": "Your IP",<br/>
"server_port": "An available port"<br/>


Logs are stored in logs folder by level<br/>
you can choose the logs level (ex warn or error) in log_config.yaml

websocket client (wss://observer.terra.dev) is in tokio tungstenite.<br/>
websocket server is in Actix.<br/>
Runtime Tokio.

How to build frontend:<br/>
install nodejs (v16.7.0) and npm (v7.20.3)

npm init -y<br/>
npm install --save-dev webpack@5.56.0 webpack-cli<br/>
npm install --save-dev html-loader<br/>
npm install --save-dev css-loader<br/>
npm install --save-dev string-replace-loader<br/>
npm install --save-dev clean-webpack-plugin<br/>
npm install --save-dev html-webpack-plugin<br/>
npm install --save-dev mini-css-extract-plugin

npm run build

Frontend assets are in www/src and built to www/dist

Program flow

main >--call--> <br/>
stream_init_task >--call--> <br/>
reader_task (tokio tungstenite) >--channel--><br/>
stream_management_task >--call--> <br/>
websocket_msg_process >--call--> <br/>
deserialize_stream >--return via channel --> <br/>
stream_init_task  >-- Actix Actor Broker--> <br/>
Actix Websocket--> Frontend ("ws://\<YOUR IP\>:\<PORT\>/stream" in config.json)
