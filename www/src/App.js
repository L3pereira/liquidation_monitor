
import './App.css';
import React, { useState, useEffect,  useRef} from "react";


// import configData from "../config.json";
let symbols = ["usgd", "ujpy", "uaud", "ukrw", "umnt", "uchf", 
                "uhkd", "ucny", "uthb", "unok", "uusd", "ugbp", 
                "ucad", "usdr", "ueur", "usek", "uinr", "udkk"]

let prev_rates = symbols.reduce((acc, symbol) => {
  acc[symbol] = 0.0;
  return acc;
}, {});



function App() {

  const [previous_rates, setPrevious_rates] = useState(prev_rates);

  const ws = useRef(null);

  useEffect(() => {    
    if ("WebSocket" in window) {
      console.log("WebSocket is supported by your Browser!");
 
      ws.current = new WebSocket("ws://@@@IP@@@/rates");
      ws.current.onopen = function() {
         console.log("Connection Open");
       };       
       ws.current.onclose = function() { 
        console.log("Connection is closed..."); 
      };
 
    }
    else {
    
      // The browser doesn't support WebSocket
      console.log("WebSocket NOT supported by your Browser!");
   }
  });

  useEffect(() => {  
    ws.current.onmessage = function (message) { 
      // console.log(message);
      // console.log(message.data);
      if (!message.data ) 
        return;
      else if (message.data === 'PING'){
        console.log("PING Received");
        ws.current.send("PONG");
        console.log("PONG SENT");
        return;
      }
        

 

      const myObj = JSON.parse(message.data)['luna_exch_rates'];
      if (!myObj) return;

      symbols.forEach(function(symbol){
            let previous = parseFloat(previous_rates[symbol]);
            let current = parseFloat(myObj[symbol]);
            let row = document.getElementById(symbol);
            if (current !== 0.0) {
               if (current > previous && previous !== 0.0){
                  row.getElementsByTagName('td')[1].innerHTML = current;
                  row.className = 'increase';

               }
               else if (current < previous && previous !== 0.0){
                  row.getElementsByTagName('td')[1].innerHTML = current;
                  row.className = 'decrease';
               }
               else {
                  row.getElementsByTagName('td')[1].innerHTML = current;
                  row.className = '';
               }
            }

            previous_rates[symbol] = current;
            setPrevious_rates(previous_rates);

          });
    };  
  });
  return (
      <div>
        <h1>Exchange Rates</h1>
        <table  id="rates">
          <thead>
            <tr><th>Symbols</th><th>Rates</th></tr>
          </thead>
          <tbody>
            { 
              symbols.map((symbol, index) => {
                return  <tr id={symbol} key={symbol}><td>{symbol}</td><td>{0.0}</td></tr>
              })        
            }
          </tbody>
        </table>
      </div>
    )

}

export default App;
