const express = require('express');
const path = require('path');
const fs = require('fs');
const app = express();
const port = 3000;
const os = require('os');
const networkInterfaces = os.networkInterfaces();

let ipAddress;

for (const name of Object.keys(networkInterfaces)) {
  for (const net of networkInterfaces[name]) {
    // Skip over non-IPv4 and internal (loopback) addresses
    if (net.family === 'IPv4' && !net.internal) {
      ipAddress = net.address;
      // Stop searching after finding the first valid IP
      break;
    }
  }
  if (ipAddress) {
    break;
  }
}

// Define the path to the JSON file
const jsonFilePath = path.join(__dirname, 'public', 'data.json');

// Initial data load from the JSON file
let jsonData = require(jsonFilePath);

// Serve static files (HTML, CSS, JS)
app.use(express.static(path.join(__dirname, 'public')));

// API endpoint to serve JSON data
app.get('/api/data', (req, res) => {
  res.json(jsonData);
});

// Watch the data.json file for changes
fs.watch(jsonFilePath, (eventType, filename) => {
  if (eventType === 'change') {
    console.log(`${filename} has been updated.`);

    // Reload the JSON file when it changes
    try {
      jsonData = require(jsonFilePath); // Reload the JSON data
      console.log('JSON data reloaded successfully.');
    } catch (error) {
      console.error('Error reloading JSON file:', error);
    }
  }
});

// Start the server
app.listen(port, '0.0.0.0', () => {
  console.log(`Server is running on http://${ipAddress}:${port}`);
});
