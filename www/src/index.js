import styles from './styles.css';

   let symbols = ["usgd", "ujpy", "uaud", "ukrw", "umnt", "uchf", "uhkd", "ucny", "uthb", "unok", "uusd", "ugbp", "ucad", "usdr", "ueur", "usek", "uinr", "udkk"]
   let previous_rates = {"usgd":0.0, "ujpy":0.0, "uaud":0.0, "ukrw":0.0, "umnt":0.0, "uchf":0.0, 
                           "uhkd":0.0, "ucny":0.0, "uthb":0.0, "unok":0.0, "uusd":0.0, "ugbp":0.0, 
                           "ucad":0.0, "usdr":0.0, "ueur":0.0, "usek":0.0, "uinr":0.0, "udkk":0.0};
function WebSocketTest() {
            
    if ("WebSocket" in window) {
      console.log("WebSocket is supported by your Browser!");
      
      let ws = new WebSocket("ws://@@@IP@@@/rates");
      ws.onopen = function() {
         console.log("Connection Open");
         heartbeat(ws);
       };
  
       ws.onmessage = function (message) { 
         // console.log(message);
         // console.log(message.data);
          if (!message.data || message.data == 'heartbeat') return;
            const myObj = JSON.parse(message.data)['luna_exch_rates'];
            if (!myObj) return;
            let text = "<table>";
            text += "<tr><th>Symbols</th><th>Rates</th></tr>";
            symbols.forEach(function(symbol){
               let previous = parseFloat(previous_rates[symbol]);
               let current = parseFloat(myObj[symbol]);

               if (current != 0.0) {
                  if (current > previous && previous != 0.0){
                     text += "<tr class='increase'><td>"+ symbol +"</td><td>" + current + "</td></tr>";
                  }
                  else if (current < previous && previous != 0.0){
                     text += "<tr class='decrease'><td>"+ symbol +"</td><td>" + current + "</td></tr>";
                  }
                  else {
                     text += "<tr><td>"+ symbol +"</td><td>" + current + "</td></tr>";
                  }
               }

               previous_rates[symbol] = current;

             });

            text += "</table>";


            document.getElementById("rates").innerHTML = text;

       };
  
       ws.onclose = function() { 
          
          // websocket is closed.
          console.log("Connection is closed..."); 
       };
    } else {
      
       // The browser doesn't support WebSocket
       console.log("WebSocket NOT supported by your Browser!");
    }
  }
function heartbeat(ws) {
   if (!ws) return;
   if (ws.readyState !== 1) return;
   console.log("heartbeat sent..."); 
   ws.send("heartbeat");
   setTimeout(heartbeat, 500);
}
WebSocketTest();