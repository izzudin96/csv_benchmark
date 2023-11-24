<?php

$file_path = '../csv/Motor_Vehicle_Register_API.csv';

if (($handle = fopen($file_path, 'r')) !== FALSE) {
    while (($data = fgetcsv($handle)) !== FALSE) {
        foreach ($data as $cell) {
            echo $cell . ' ';
        }
        echo "\n";
    }
    fclose($handle);
}
