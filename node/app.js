const express = require('express');
const fileUpload = require('express-fileupload');
const { analyse_frieze } = require('../pkg/frieze_lib.js');

const app = express();
const port = 3000;
app.use(express.static(__dirname + '/public'));
app.use(fileUpload());

app.get('/', (req, res) => res.redirect("/index.html"));

app.post('/analyse', function (req, res) {
  if (!req.files || Object.keys(req.files).length === 0) {
    return res.status(400).send('No files were uploaded.');
  }
  console.log("Received " + req.files.csv_file.name + " with size: " + req.files.csv_file.size);

  let csv_file = req.files.csv_file;
  console.time(csv_file.name);
  var svg = analyse_frieze(csv_file.data)
  console.timeEnd(csv_file.name);
  res.send(svg)
})


app.listen(port, () => console.log(`Listening at http://localhost:${port}`))
