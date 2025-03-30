const express = require('express');
const path = require('path');
const fs = require('fs');
const app = express();
const port = 3000;

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
  console.log(`Server is running on http://138.67.178.92:${port}`);
});
