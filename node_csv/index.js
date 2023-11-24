const fs = require('fs');
const csv = require('csv-parser');

const file_path = '../csv/Motor_Vehicle_Register_API.csv';

fs.createReadStream(file_path)
    .pipe(csv())
    .on('data', (row) => {
        Object.values(row).forEach((cell) => {
            process.stdout.write(`${cell} `);
        });
        console.log();
    })
    .on('end', () => {
        console.log('CSV file reading is complete.');
    });